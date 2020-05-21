use std::env;
use std::process::exit;

use minigrep::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let cmd = Cmd::new(&args).unwrap_or_else(|err| {
        println!("parse cmd error cause by:{}", err);
        exit(1);
    });
    if let Err(e) = run(cmd) {
        print!("run error by:{}",e);
        exit(1)
    }
}




