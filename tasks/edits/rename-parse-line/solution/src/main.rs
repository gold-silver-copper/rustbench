use task::{Summary, parse_entry};

const SAMPLE: &str = "\
08:00:00 INFO service started
08:01:10 WARN cache miss rate high
08:02:45 ERROR database unreachable
garbage
";

fn main() {
    let s = Summary::from_text(SAMPLE);
    println!("info={} warn={} error={} malformed={}", s.info, s.warn, s.error, s.malformed);
    if let Some(first) = SAMPLE.lines().next().and_then(parse_entry) {
        println!("first: {} {:?}", first.time, first.level);
    }
    for msg in task::report::errors(SAMPLE) {
        println!("error: {msg}");
    }
}
