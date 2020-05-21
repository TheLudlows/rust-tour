use std::fs;
use std::error::Error;

pub struct Cmd<'a> {
    query: &'a str,
    file_name: &'a str,
}

impl<'a> Cmd<'a> {
    pub fn new(args: &[String]) -> Result<Cmd, &str> {
        if args.len() < 3 {
            return Err("param not enough");
        }
        let query = &args[1];
        let file_name = &args[2];
        return Ok(Cmd { query, file_name });
    }
}

pub fn run(cmd: Cmd) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(cmd.file_name)?;
    for l in search(cmd.query, &contents) {
        println!("{}", l)
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
    /*    #[test]
        fn run_test() {
            let v = vec![String::from("a"),String::from("/User/liuchao56/1.txt")];
            let cmd = Cmd::new(& v).unwrap_or_else(|err|{
                println!("err{}",err);
                exit(1)
            });
            run(cmd);
        }*/
}

