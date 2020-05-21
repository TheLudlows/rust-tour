use std::env;
use std::process::exit;

use minigrep::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd = Cmd::new(&args).unwrap_or_else(|err| {
        eprintln!("parse cmd error cause by:{}", err);
        exit(1);
    });
    if let Err(e) = run(cmd) {
        eprintln!("run error by:{}",e);
        exit(1)
    }
    get_key();
}
pub fn get_key() -> & 'static str {
    "aaabbbccc"
}



