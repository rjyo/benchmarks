use docopt::Docopt;
use serde::Deserialize;

const USAGE: &'static str = "
Benchmarks.

Usage:
  bench mq (nanomsg | zmq) <url> [--server | --client]
  bench (-h | --help)
  bench --version

Options:
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_client: isize,
    flag_server: bool,
    flag_version: bool,
    arg_url: Vec<String>,
    cmd_mq: bool,
    cmd_nanomsg: bool,
    cmd_zmq: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
}
