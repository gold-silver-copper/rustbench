#!/usr/bin/env python3
"""Build the `rustc-error-index` suite from rustc's error index.

Each `compiler/rustc_error_codes/src/error_codes/E*.md` file in rust-lang/rust
has an erroneous example (```compile_fail,E0xxx) and usually a corrected one
in the prose after it. A code becomes a task when an erroneous example and a
corrected example clearly correspond (similar code, the fix is not only a
deletion), the erroneous one fails with that error code and the corrected one
compiles on this machine's stable rustc (edition 2024, else 2021), std only.

Usage:
    tools/import_error_index.py --rust <rust-lang/rust checkout> --commit <sha>
        [--out tasks/rustc-error-index] [--jobs 2]

The checkout only needs `compiler/rustc_error_codes/src/error_codes` (a
sparse, shallow clone is enough). Python 3 stdlib only.
"""

import argparse
import concurrent.futures
import difflib
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

SUITE = "rustc-error-index"
ERR_DIR = "compiler/rustc_error_codes/src/error_codes"
LICENSE = "MIT OR Apache-2.0"
# The correction must keep most of the erroneous example (it is a fix of that
# code, not a different program) and be mostly made of it.
MIN_KEPT_BAD = 0.8
MIN_KEPT_GOOD = 0.4
RUST_INFO = {"", "rust", "no_run", "should_panic", "edition2015", "edition2018",
             "edition2021", "edition2024"}
SKIP_INFO = {"ignore", "text", "console", "sh", "toml", "json", "mermaid", "test_harness"}

PROMPTS = {
    "terse": [
        "fix the build",
        "make this compile; keep the behavior the same",
        "get the crate compiling without changing what it does",
        "fix the compile error",
    ],
    "chatty": [
        "Hi! This crate doesn't compile right now. Could you get it building again "
        "without changing what the code is meant to do?",
        "Hey, I'm stuck on a compiler error in this little crate. Can you fix it so "
        "it builds? Please keep the behavior as it is.",
        "Could you take a look at this code? cargo build fails and I can't see why. "
        "I'd like it to compile while still doing the same thing.",
        "Hello! I wrote this small Rust crate but the compiler rejects it. Would you "
        "mind fixing it so it compiles, without changing what it does?",
    ],
    "hurried": [
        "doesnt compile pls fix",
        "build broken, fix it but dont change what it does",
        "cargo build fails?? fix pls",
        "compile error, make it build quick",
    ],
}


def h(s: str) -> int:
    return int(hashlib.sha256(s.encode()).hexdigest(), 16)


def blocks(md: str):
    """Yield (info, code) for each fenced block, in order."""
    lines = md.splitlines()
    i = 0
    while i < len(lines):
        m = re.match(r"^(```+)\s*(.*)$", lines[i])
        if not m:
            i += 1
            continue
        fence, info = m.group(1), m.group(2).strip()
        body = []
        i += 1
        while i < len(lines) and not lines[i].startswith(fence):
            body.append(lines[i])
            i += 1
        i += 1
        yield info, "\n".join(body)


def unhide(code: str) -> str:
    """Apply rustdoc's hidden-line rules: `# x` -> `x`, `#` -> ``, `##x` -> `#x`."""
    out = []
    for line in code.splitlines():
        s = line.lstrip()
        ind = line[: len(line) - len(s)]
        if s == "#":
            out.append("")
        elif s.startswith("##"):
            out.append(ind + s[1:])
        elif s.startswith("# "):
            out.append(ind + s[2:])
        else:
            out.append(line)
    return "\n".join(out)


def comment_start(line: str):
    """Index of a `//` line comment outside string/char literals, or None."""
    in_str = False
    i = 0
    while i < len(line) - 1:
        c = line[i]
        if in_str:
            if c == "\\":
                i += 2
                continue
            if c == '"':
                in_str = False
        elif c == '"':
            in_str = True
        elif c == "/" and line[i + 1] == "/":
            return i
        i += 1
    return None


def strip_comments(code: str) -> str:
    """Drop `//` comments (not doc comments): in the error index they point at
    the error ("// error!", "^^^ expected `i32`") or at the fix."""
    out = []
    for line in code.splitlines():
        k = comment_start(line)
        if k is not None and not line[k:].startswith(("///", "//!")):
            line = line[:k].rstrip()
            if not line:
                continue
        out.append(line)
    text = "\n".join(out)
    text = re.sub(r"\n{3,}", "\n\n", text)
    return text.strip("\n") + "\n"


def tokens(code: str):
    code = re.sub(r"//[^\n]*", "", code)
    return re.findall(r"\w+|[^\w\s]", code)


def kept(a: str, b: str):
    """Fractions of a's and of b's tokens that the two have in common."""
    ta, tb = tokens(a), tokens(b)
    sm = difflib.SequenceMatcher(None, ta, tb, autojunk=False)
    same = sum(m.size for m in sm.get_matching_blocks())
    return same / max(len(ta), 1), same / max(len(tb), 1)


def only_deletes(a: str, b: str) -> bool:
    ops = difflib.SequenceMatcher(None, tokens(a), tokens(b), autojunk=False).get_opcodes()
    return all(op in ("equal", "delete") for op, *_ in ops)


def changed_tokens(a: str, b: str) -> int:
    ops = difflib.SequenceMatcher(None, tokens(a), tokens(b), autojunk=False).get_opcodes()
    return sum(max(i2 - i1, j2 - j1) for op, i1, i2, j1, j2 in ops if op != "equal")


def has_main(code: str) -> bool:
    return re.search(r"\bfn\s+main\s*\(", code) is not None


def as_program(code: str) -> str:
    """rustdoc's doctest wrapping: no `fn main` -> body goes inside one."""
    if has_main(code):
        return code
    attrs, rest = [], []
    for line in code.splitlines():
        (attrs if line.startswith("#![") else rest).append(line)
    body = "\n".join(("    " + l) if l.strip() else "" for l in rest)
    head = ("\n".join(attrs) + "\n\n") if attrs else ""
    return f"{head}fn main() {{\n{body}\n}}\n"


def unusable(code: str) -> str | None:
    if re.search(r"#!\[\s*feature", code):
        return "feature gate"
    for m in re.finditer(r"\bextern\s+crate\s+(\w+)", code):
        if m.group(1) not in ("std", "core", "alloc", "self"):
            return "external crate"
    if re.search(r"#!\[\s*no_std|#!\[\s*no_main|#!\[\s*no_core", code):
        return "no_std/no_main"
    return None


def rustc(src: str, kind: str, edition: str, work: Path):
    work.mkdir(parents=True, exist_ok=True)
    f = work / ("lib.rs" if kind == "lib" else "main.rs")
    f.write_text(src)
    out = work / ("libtask.rlib" if kind == "lib" else "task")
    cmd = ["rustc", "--edition", edition, "--crate-name", "task",
           "--crate-type", "lib" if kind == "lib" else "bin",
           "--emit=link", "-o", str(out), str(f)]
    try:
        r = subprocess.run(cmd, capture_output=True, text=True, timeout=120,
                           env={k: v for k, v in os.environ.items() if k != "RUSTC_BOOTSTRAP"})
    except subprocess.TimeoutExpired:
        return False, "timeout", None
    return r.returncode == 0, r.stderr, out


def run_bin(path: Path):
    try:
        r = subprocess.run([str(path)], capture_output=True, text=True, timeout=5,
                           stdin=subprocess.DEVNULL)
    except subprocess.TimeoutExpired:
        return None
    return r.stdout if r.returncode == 0 else None


def pick_pair(code: str, md: str):
    """The erroneous example and the correction that best fit together."""
    bad, good = [], []
    for info, body in blocks(md):
        attrs = {a.strip() for a in info.split(",")}
        if "compile_fail" in attrs:
            errs = {a for a in attrs if re.fullmatch(r"E\d{4}", a)}
            if not errs or code in errs:
                bad.append(strip_comments(unhide(body)))
        elif attrs <= RUST_INFO:
            good.append((len(bad), strip_comments(unhide(body))))
    if not bad:
        return None, "no compile_fail example"
    best, seen = None, False
    for bi, b in enumerate(bad):
        for after, g in good:
            if after <= bi:  # the correction comes after the erroneous example
                continue
            seen = True
            if g == b or only_deletes(b, g):
                continue
            kb, kg = kept(b, g)
            if kb < MIN_KEPT_BAD or kg < MIN_KEPT_GOOD:
                continue
            if best is None or kb + kg > best[2]:
                best = (b, g, kb + kg)
    if not seen:
        return None, "no corrected example after it"
    if best is None:
        return None, "no corresponding correction"
    return best, None


def build_task(code: str, md: str, scratch: Path):
    pair, why = pick_pair(code, md)
    if pair is None:
        return None, why
    bad, good, _ = pair
    for c in (bad, good):
        why = unusable(c)
        if why:
            return None, why
    layouts = []
    if not has_main(bad) and not has_main(good):
        layouts.append(("lib", bad, good))
    layouts.append(("bin", as_program(bad), as_program(good)))
    reason = "solution does not compile"
    for kind, b, g in layouts:
        for edition in ("2024", "2021"):
            ok, err, out = rustc(g, kind, edition, scratch / f"good-{kind}-{edition}")
            if not ok:
                continue
            bok, berr, _ = rustc(b, kind, edition, scratch / f"bad-{kind}-{edition}")
            if bok:
                reason = "erroneous example compiles"
                continue
            if f"error[{code}]" not in berr:
                reason = "erroneous example fails with a different error"
                continue
            stdout = None
            if kind == "bin" and out is not None:
                s1 = run_bin(out)
                if s1 and s1 == run_bin(out):
                    stdout = s1
            return {
                "kind": kind, "edition": edition, "bad": b, "good": g,
                "stdout": stdout,
                "changed": changed_tokens(b, g),
            }, None
    return None, reason


def summary_of(md: str) -> str:
    para = []
    for line in md.splitlines():
        if not line.strip():
            if para:
                break
            continue
        if line.startswith("#"):
            continue
        para.append(line.strip())
    s = " ".join(para)
    s = re.sub(r"\bE0\d{3}\b", "this error", s)
    return s[:200]


def toml_str(s: str) -> str:
    return json.dumps(s, ensure_ascii=True)


def write_task(out: Path, code: str, md: str, t: dict, commit: str):
    tid = code.lower()
    d = out / tid
    if d.exists():
        shutil.rmtree(d)
    split = "heldout" if h(code) % 3 == 0 else "dev"
    difficulty = 1 if t["changed"] <= 4 else 2 if t["changed"] <= 15 else 3
    prompt = {style: opts[h(code + style) % len(opts)] for style, opts in PROMPTS.items()}
    lines = [
        f'id = "{tid}"',
        f'suite = "{SUITE}"',
        'kind = "fix-error"',
        f'split = "{split}"',
        f"difficulty = {difficulty}",
        f"summary = {toml_str(summary_of(md))}",
        f'source = "rust-lang/rust@{commit} {ERR_DIR}/{code}.md"',
        f'license = "{LICENSE}"',
        "",
        "[prompt]",
    ]
    lines += [f"{k} = {toml_str(v)}" for k, v in prompt.items()]
    lines += ["", "[check]", "build = true"]
    if t["stdout"] is not None:
        lines.append(f"stdout = {toml_str(t['stdout'])}")
    (d / "workspace" / "src").mkdir(parents=True)
    (d / "solution" / "src").mkdir(parents=True)
    (d / "task.toml").write_text("\n".join(lines) + "\n")
    (d / "workspace" / "Cargo.toml").write_text(
        f'[package]\nname = "task"\nversion = "0.1.0"\nedition = "{t["edition"]}"\n')
    name = "lib.rs" if t["kind"] == "lib" else "main.rs"
    (d / "workspace" / "src" / name).write_text(t["bad"])
    (d / "solution" / "src" / name).write_text(t["good"])


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--rust", required=True, type=Path)
    ap.add_argument("--commit", required=True)
    ap.add_argument("--out", type=Path,
                    default=Path(__file__).resolve().parent.parent / "tasks" / SUITE)
    ap.add_argument("--jobs", type=int, default=2)
    ap.add_argument("--skip", type=Path,
                    help="file of codes to leave out (one per line, `#` comments), "
                         "e.g. ones `rustbench validate` rejects")
    args = ap.parse_args()

    skip = {}
    if args.skip and args.skip.exists():
        for line in args.skip.read_text().splitlines():
            code, _, why = line.partition("#")
            if code.strip():
                skip[code.strip().upper()] = why.strip() or "listed in skip file"

    files = sorted((args.rust / ERR_DIR).glob("E*.md"))
    if not files:
        sys.exit(f"no error code files under {args.rust / ERR_DIR}")
    args.out.mkdir(parents=True, exist_ok=True)
    for d in args.out.iterdir():
        if d.is_dir() and re.fullmatch(r"e\d{4}", d.name):
            shutil.rmtree(d)

    base = Path(os.environ.get("CLAUDE_CODE_TMPDIR")
                or Path.home() / ".cache" / "claude-tmp")
    base.mkdir(parents=True, exist_ok=True)
    reasons = {}
    kept = []
    with tempfile.TemporaryDirectory(dir=base, prefix="erridx-") as tmp:
        def one(f: Path):
            code = f.stem
            if code in skip:
                return code, None, skip[code]
            md = f.read_text()
            return (code, md, *build_task(code, md, Path(tmp) / code))

        with concurrent.futures.ThreadPoolExecutor(max_workers=args.jobs) as ex:
            for code, md, *res in ex.map(one, files):
                if md is None:
                    t, why = None, res[0]
                else:
                    t, why = res
                if t is None:
                    reasons.setdefault(re.sub(r" \(.*", "", why), []).append(code)
                    continue
                write_task(args.out, code, md, t, args.commit)
                kept.append(code)

    dev = sum(1 for c in kept if h(c) % 3 != 0)
    print(f"examined {len(files)} codes, kept {len(kept)} "
          f"(dev {dev}, heldout {len(kept) - dev})")
    for why, codes in sorted(reasons.items(), key=lambda kv: -len(kv[1])):
        print(f"  skipped {len(codes):4} {why}")


if __name__ == "__main__":
    main()
