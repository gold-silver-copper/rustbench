//! rustbench: run coding agents on small Rust tasks and score them with
//! cargo. See `docs/FORMAT.md` for the task format and agent protocol.

use std::collections::BTreeMap;
use std::io::{Read as _, Write as _};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use anyhow::{Context as _, Result, anyhow, bail};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(about = "Rust coding benchmarks, checked by cargo")]
struct Cli {
    /// The tasks directory.
    #[arg(long, default_value = "tasks", global = true)]
    tasks: PathBuf,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// List tasks.
    List {
        #[command(flatten)]
        filter: Filter,
    },
    /// Run an agent on tasks and write one JSON line per run.
    Run {
        /// The agent command (run with `sh -c`), or a built-in agent:
        /// `noop` (changes nothing) or `reference` (applies the solution).
        #[arg(long)]
        agent: String,
        #[command(flatten)]
        filter: Filter,
        /// Prompt style: a style name, `all`, or `first`.
        #[arg(long, default_value = "all")]
        style: String,
        /// Tasks run at once (each with its own cargo target directory).
        #[arg(long, default_value_t = 4)]
        jobs: usize,
        /// Agent time limit per run, in seconds.
        #[arg(long, default_value_t = 600)]
        agent_timeout: u64,
        /// Where to write results (JSON lines).
        #[arg(long, default_value = "results.jsonl")]
        out: PathBuf,
        /// Scratch directory for working copies and build output (default
        /// `~/.cache/rustbench`).
        #[arg(long)]
        scratch: Option<PathBuf>,
    },
    /// Check every task: the untouched workspace must fail, the reference
    /// solution must pass, and the prompt styles must be present.
    Validate {
        #[command(flatten)]
        filter: Filter,
        #[arg(long, default_value_t = 4)]
        jobs: usize,
        /// Scratch directory (default `~/.cache/rustbench`).
        #[arg(long)]
        scratch: Option<PathBuf>,
    },
    /// Summarize a results file.
    Report {
        /// Results file (JSON lines).
        results: PathBuf,
    },
}

#[derive(clap::Args, Clone)]
struct Filter {
    /// Only this suite.
    #[arg(long)]
    suite: Option<String>,
    /// `dev`, `heldout` or `all`.
    #[arg(long, default_value = "dev")]
    split: String,
    /// Only tasks whose id contains this text.
    #[arg(long)]
    only: Option<String>,
    /// At most this many tasks.
    #[arg(long)]
    limit: Option<usize>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Task {
    id: String,
    suite: String,
    kind: String,
    split: String,
    #[serde(default)]
    difficulty: u8,
    #[serde(default)]
    summary: String,
    #[serde(default)]
    source: Option<String>,
    #[serde(default)]
    license: Option<String>,
    prompt: BTreeMap<String, String>,
    #[serde(default)]
    check: Check,
    #[serde(skip)]
    dir: PathBuf,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
struct Check {
    #[serde(default)]
    build: Option<bool>,
    #[serde(default)]
    test: Option<bool>,
    #[serde(default)]
    stdout: Option<String>,
    #[serde(default)]
    answer: Option<String>,
    #[serde(default)]
    expect_refusal: bool,
    #[serde(default)]
    timeout_secs: Option<u64>,
}

/// One run's result.
#[derive(Debug, Clone, Deserialize, Serialize)]
struct Outcome {
    task: String,
    suite: String,
    kind: String,
    split: String,
    difficulty: u8,
    style: String,
    agent: String,
    outcome: String,
    agent_secs: f64,
    check_secs: f64,
    detail: String,
}

const REPLY: &str = ".rustbench-reply";
const STYLES: [&str; 3] = ["terse", "chatty", "hurried"];

fn load_tasks(root: &Path, f: &Filter) -> Result<Vec<Task>> {
    let mut out = Vec::new();
    let mut suites: Vec<PathBuf> = std::fs::read_dir(root)
        .with_context(|| format!("reading {}", root.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.is_dir())
        .collect();
    suites.sort();
    for s in suites {
        let mut dirs: Vec<PathBuf> = std::fs::read_dir(&s)?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.join("task.toml").exists())
            .collect();
        dirs.sort();
        for d in dirs {
            let text = std::fs::read_to_string(d.join("task.toml"))?;
            let mut t: Task =
                toml::from_str(&text).with_context(|| format!("parsing {}", d.display()))?;
            t.dir = d;
            if f.suite.as_ref().is_some_and(|x| *x != t.suite)
                || (f.split != "all" && f.split != t.split)
                || f.only.as_ref().is_some_and(|x| !t.id.contains(x.as_str()))
            {
                continue;
            }
            out.push(t);
        }
    }
    if let Some(n) = f.limit {
        out.truncate(n);
    }
    Ok(out)
}

fn copy_dir(from: &Path, to: &Path) -> Result<()> {
    if !from.exists() {
        return Ok(());
    }
    std::fs::create_dir_all(to)?;
    for e in std::fs::read_dir(from)? {
        let e = e?;
        let p = e.path();
        let dst = to.join(e.file_name());
        if p.is_dir() {
            copy_dir(&p, &dst)?;
        } else {
            std::fs::copy(&p, &dst)?;
        }
    }
    Ok(())
}

/// All files under a directory (relative path -> contents), skipping
/// `target/` and the reply file.
fn snapshot(dir: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    fn go(base: &Path, d: &Path, out: &mut BTreeMap<PathBuf, Vec<u8>>) {
        let Ok(rd) = std::fs::read_dir(d) else { return };
        for e in rd.flatten() {
            let p = e.path();
            let name = e.file_name();
            if name == "target" || name == REPLY {
                continue;
            }
            if p.is_dir() {
                go(base, &p, out);
            } else if let (Ok(rel), Ok(bytes)) = (p.strip_prefix(base), std::fs::read(&p)) {
                out.insert(rel.to_path_buf(), bytes);
            }
        }
    }
    let mut out = BTreeMap::new();
    go(dir, dir, &mut out);
    out
}

struct Ran {
    ok: bool,
    timed_out: bool,
    stdout: String,
    stderr: String,
}

fn run(mut cmd: Command, timeout: Duration) -> Result<Ran> {
    let mut child = cmd
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let drain = |p: Option<Box<dyn std::io::Read + Send>>| {
        std::thread::spawn(move || {
            let mut b = Vec::new();
            if let Some(mut p) = p {
                let _ = p.read_to_end(&mut b);
            }
            b
        })
    };
    let o = drain(
        child
            .stdout
            .take()
            .map(|p| Box::new(p) as Box<dyn std::io::Read + Send>),
    );
    let e = drain(
        child
            .stderr
            .take()
            .map(|p| Box::new(p) as Box<dyn std::io::Read + Send>),
    );
    let start = Instant::now();
    let status = loop {
        if let Some(s) = child.try_wait()? {
            break Some(s);
        }
        if start.elapsed() > timeout {
            let _ = child.kill();
            let _ = child.wait();
            break None;
        }
        std::thread::sleep(Duration::from_millis(50));
    };
    let stdout = String::from_utf8_lossy(&o.join().unwrap_or_default()).into_owned();
    let stderr = String::from_utf8_lossy(&e.join().unwrap_or_default()).into_owned();
    Ok(Ran {
        ok: status.is_some_and(|s| s.success()),
        timed_out: status.is_none(),
        stdout,
        stderr,
    })
}

fn cargo(work: &Path, target: &Path, args: &[&str]) -> Command {
    let mut c = Command::new("cargo");
    c.current_dir(work)
        .args(args)
        .args(["--offline", "--quiet"])
        .env("CARGO_TARGET_DIR", target)
        .env("CARGO_BUILD_JOBS", "2")
        .env("RUST_TEST_THREADS", "2")
        .env("CARGO_TERM_COLOR", "never");
    c
}

fn has_tests(dir: &Path) -> bool {
    snapshot(dir).iter().any(|(p, b)| {
        p.extension().is_some_and(|e| e == "rs") && String::from_utf8_lossy(b).contains("#[test]")
    })
}

fn first_lines(s: &str, n: usize) -> String {
    s.lines()
        .filter(|l| !l.trim().is_empty())
        .take(n)
        .collect::<Vec<_>>()
        .join("\n")
}

/// Run the checks on a working copy (after `hidden/` has been applied).
fn check(t: &Task, work: &Path, target: &Path) -> Result<(String, String)> {
    let timeout = Duration::from_secs(t.check.timeout_secs.unwrap_or(120));
    if t.check.build.unwrap_or(true) {
        let r = run(cargo(work, target, &["build"]), timeout)?;
        if r.timed_out {
            return Ok(("timeout".into(), "build timed out".into()));
        }
        if !r.ok {
            return Ok((
                "fail".into(),
                format!("build failed:\n{}", first_lines(&r.stderr, 12)),
            ));
        }
    }
    if t.check.test.unwrap_or_else(|| has_tests(work)) {
        let r = run(cargo(work, target, &["test"]), timeout)?;
        if r.timed_out {
            return Ok(("timeout".into(), "tests timed out".into()));
        }
        if !r.ok {
            let text = format!("{}\n{}", r.stdout, r.stderr);
            let failing: Vec<&str> = text
                .lines()
                .filter(|l| l.contains("FAILED") || l.contains("panicked"))
                .take(6)
                .collect();
            return Ok((
                "fail".into(),
                format!("tests failed:\n{}", failing.join("\n")),
            ));
        }
    }
    if let Some(want) = &t.check.stdout {
        let r = run(cargo(work, target, &["run"]), timeout)?;
        if r.timed_out {
            return Ok(("timeout".into(), "run timed out".into()));
        }
        if !r.ok || r.stdout != *want {
            return Ok((
                "fail".into(),
                format!(
                    "wrong output: {:?}",
                    r.stdout.chars().take(200).collect::<String>()
                ),
            ));
        }
    }
    if let Some(want) = &t.check.answer {
        let got = std::fs::read_to_string(work.join("ANSWER")).unwrap_or_default();
        if got.trim() != want.trim() {
            return Ok(("fail".into(), format!("wrong answer: {:?}", got.trim())));
        }
    }
    Ok(("pass".into(), String::new()))
}

/// One run of one agent on one task in one prompt style.
fn run_one(
    t: &Task,
    style: &str,
    agent: &str,
    work: &Path,
    target: &Path,
    agent_timeout: Duration,
) -> Result<Outcome> {
    let prompt = t
        .prompt
        .get(style)
        .cloned()
        .ok_or_else(|| anyhow!("{} has no `{style}` prompt", t.id))?;
    if work.exists() {
        std::fs::remove_dir_all(work)?;
    }
    copy_dir(&t.dir.join("workspace"), work)?;
    let before = snapshot(work);
    let prompt_file = work.with_extension("prompt");
    std::fs::write(&prompt_file, &prompt)?;
    let started = Instant::now();
    let mut agent_timed_out = false;
    match agent {
        "noop" => {}
        "reference" => copy_dir(&t.dir.join("solution"), work)?,
        cmd => {
            let mut c = Command::new("sh");
            c.arg("-c")
                .arg(cmd)
                .env("RB_WORK", work)
                .env("RB_PROMPT", &prompt)
                .env("RB_PROMPT_FILE", &prompt_file)
                .env("RB_TASK", format!("{}/{}", t.suite, t.id))
                .env("RB_STYLE", style);
            let r = run(c, agent_timeout)?;
            agent_timed_out = r.timed_out;
        }
    }
    let agent_secs = started.elapsed().as_secs_f64();
    let base = Outcome {
        task: format!("{}/{}", t.suite, t.id),
        suite: t.suite.clone(),
        kind: t.kind.clone(),
        split: t.split.clone(),
        difficulty: t.difficulty,
        style: style.into(),
        agent: agent.into(),
        outcome: String::new(),
        agent_secs,
        check_secs: 0.0,
        detail: String::new(),
    };
    if agent_timed_out {
        return Ok(Outcome {
            outcome: "timeout".into(),
            detail: "agent timed out".into(),
            ..base
        });
    }
    let replied = work.join(REPLY).exists();
    let unchanged = snapshot(work) == before;
    if t.check.expect_refusal {
        let (outcome, detail) = if replied && unchanged {
            ("pass", String::new())
        } else {
            ("fail", "changed code instead of asking".to_string())
        };
        return Ok(Outcome {
            outcome: outcome.into(),
            detail,
            ..base
        });
    }
    if replied && unchanged {
        let msg = std::fs::read_to_string(work.join(REPLY)).unwrap_or_default();
        return Ok(Outcome {
            outcome: "refused".into(),
            detail: first_lines(&msg, 3),
            ..base
        });
    }
    copy_dir(&t.dir.join("hidden"), work)?;
    let cs = Instant::now();
    let (outcome, detail) = check(t, work, target)?;
    Ok(Outcome {
        outcome,
        detail,
        check_secs: cs.elapsed().as_secs_f64(),
        ..base
    })
}

fn styles_for(t: &Task, style: &str) -> Vec<String> {
    match style {
        "all" => t.prompt.keys().cloned().collect(),
        "first" => t.prompt.keys().next().cloned().into_iter().collect(),
        s => vec![s.to_string()],
    }
}

/// Run jobs on `jobs` workers; each worker gets its own target directory.
fn par_runs(
    jobs: Vec<(Task, String)>,
    workers: usize,
    scratch: &Path,
    f: &(dyn Fn(&Task, &str, &Path, &Path) -> Result<Outcome> + Sync),
) -> Vec<Outcome> {
    let queue = Mutex::new(jobs.into_iter().enumerate().collect::<Vec<_>>());
    let results = Mutex::new(Vec::new());
    std::thread::scope(|s| {
        for k in 0..workers.max(1) {
            let (queue, results) = (&queue, &results);
            s.spawn(move || {
                let target = scratch.join(format!("target-{k}"));
                loop {
                    let next = queue.lock().ok().and_then(|mut q| q.pop());
                    let Some((i, (t, style))) = next else { break };
                    let work = scratch.join(format!("work-{k}"));
                    let r = f(&t, &style, &work, &target).unwrap_or_else(|e| Outcome {
                        task: format!("{}/{}", t.suite, t.id),
                        suite: t.suite.clone(),
                        kind: t.kind.clone(),
                        split: t.split.clone(),
                        difficulty: t.difficulty,
                        style: style.clone(),
                        agent: String::new(),
                        outcome: "error".into(),
                        agent_secs: 0.0,
                        check_secs: 0.0,
                        detail: format!("{e:#}"),
                    });
                    eprintln!("{:8} {} [{}]", r.outcome, r.task, r.style);
                    if let Ok(mut v) = results.lock() {
                        v.push((i, r));
                    }
                }
            });
        }
    });
    let mut v = results.into_inner().unwrap_or_default();
    v.sort_by_key(|x| x.0);
    v.into_iter().map(|x| x.1).collect()
}

fn report(rows: &[Outcome]) {
    let mut groups: BTreeMap<String, (usize, usize, usize, usize)> = BTreeMap::new();
    for r in rows {
        for key in [
            "all".to_string(),
            format!("suite {}", r.suite),
            format!("kind {}", r.kind),
            format!("split {}", r.split),
            format!("style {}", r.style),
        ] {
            let g = groups.entry(key).or_default();
            g.0 += 1;
            match r.outcome.as_str() {
                "pass" => g.1 += 1,
                "refused" => g.2 += 1,
                _ => g.3 += 1,
            }
        }
    }
    println!(
        "{:<28} {:>6} {:>7} {:>8} {:>7} {:>9}",
        "group", "runs", "pass%", "refused", "failed", "cost/run"
    );
    for (k, (n, p, r, f)) in groups {
        let cost = (10 * f + r) as f64 / n.max(1) as f64;
        println!(
            "{k:<28} {n:>6} {:>6.1}% {r:>8} {f:>7} {cost:>9.2}",
            100.0 * p as f64 / n.max(1) as f64
        );
    }
}

/// The scratch directory: outside any cargo workspace by default.
fn scratch_dir(s: Option<PathBuf>) -> Result<PathBuf> {
    let d = match s {
        Some(d) => d,
        None => PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".into()))
            .join(".cache/rustbench"),
    };
    std::fs::create_dir_all(&d)?;
    Ok(std::path::absolute(&d)?)
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::List { filter } => {
            for t in load_tasks(&cli.tasks, &filter)? {
                println!(
                    "{}/{}\t{}\t{}\td{}\t{}",
                    t.suite, t.id, t.kind, t.split, t.difficulty, t.summary
                );
            }
        }
        Cmd::Run {
            agent,
            filter,
            style,
            jobs,
            agent_timeout,
            out,
            scratch,
        } => {
            let tasks = load_tasks(&cli.tasks, &filter)?;
            let scratch = scratch_dir(scratch)?;
            let mut todo = Vec::new();
            for t in tasks {
                for s in styles_for(&t, &style) {
                    todo.push((t.clone(), s));
                }
            }
            let timeout = Duration::from_secs(agent_timeout);
            let rows = par_runs(todo, jobs, &scratch, &|t, s, w, tg| {
                run_one(t, s, &agent, w, tg, timeout)
            });
            let mut f = std::fs::File::create(&out)?;
            for r in &rows {
                writeln!(f, "{}", serde_json::to_string(r)?)?;
            }
            report(&rows);
        }
        Cmd::Validate {
            filter,
            jobs,
            scratch,
        } => {
            let tasks = load_tasks(&cli.tasks, &filter)?;
            let scratch = scratch_dir(scratch)?;
            let mut problems = Vec::new();
            for t in &tasks {
                for s in STYLES {
                    if !t.prompt.contains_key(s) {
                        problems.push(format!("{}/{}: no `{s}` prompt", t.suite, t.id));
                    }
                }
                if !t.dir.join("solution").exists() {
                    problems.push(format!("{}/{}: no solution/", t.suite, t.id));
                }
            }
            let todo: Vec<(Task, String)> = tasks
                .iter()
                .map(|t| (t.clone(), "first".to_string()))
                .collect();
            let first = |t: &Task| {
                styles_for(t, "first")
                    .into_iter()
                    .next()
                    .unwrap_or_default()
            };
            let noop = par_runs(todo.clone(), jobs, &scratch, &|t, _, w, tg| {
                run_one(t, &first(t), "noop", w, tg, Duration::from_secs(60))
            });
            let reference = par_runs(todo, jobs, &scratch, &|t, _, w, tg| {
                run_one(t, &first(t), "reference", w, tg, Duration::from_secs(60))
            });
            for r in noop.iter().filter(|r| r.outcome == "pass") {
                problems.push(format!("{}: passes without any change", r.task));
            }
            for r in reference.iter().filter(|r| r.outcome != "pass") {
                problems.push(format!(
                    "{}: the reference solution does not pass ({}: {})",
                    r.task, r.outcome, r.detail
                ));
            }
            println!("{} tasks checked", tasks.len());
            for p in &problems {
                println!("PROBLEM {p}");
            }
            if !problems.is_empty() {
                bail!("{} problems", problems.len());
            }
        }
        Cmd::Report { results } => {
            let rows: Vec<Outcome> = std::fs::read_to_string(&results)?
                .lines()
                .filter(|l| !l.trim().is_empty())
                .map(serde_json::from_str)
                .collect::<Result<_, _>>()?;
            report(&rows);
        }
    }
    Ok(())
}
