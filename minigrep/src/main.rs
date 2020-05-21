use std::{env, fs};
use std::process::exit;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let cmd = Cmd::new(&args).unwrap_or_else(|err| {
        println!("error cause by:{}", err);
        exit(1);
    });
    run(cmd);
}

struct Cmd<'a> {
    query: &'a str,
    file_name: &'a str,
}

impl<'a> Cmd<'a> {
    fn new(args: &[String]) -> Result<Cmd, &str> {
        if args.len() < 3 {
            return Err("param not enough");
        }
        let query = &args[1];
        let file_name = &args[2];
        return Ok(Cmd { query, file_name });
    }
}
fn run(cmd: Cmd) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(cmd.file_name)?;
    println!("With text:\n{}", contents);
    Ok(())
}
