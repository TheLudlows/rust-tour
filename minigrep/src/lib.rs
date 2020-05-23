use std::fs;
use std::error::Error;

pub struct Cmd<'a> {
    query: &'a str,
    file_name: &'a str,
    case_insensitive:bool
}

impl<'a> Cmd<'a> {
    pub fn new(args: &[String]) -> Result<Cmd, & str> {
        if args.len() < 3 {
            return Err("param not enough");
        }
        let query = & args[1];
        let file_name = & args[2];
        let b = args[3].eq(& String::from("0"));
        return Ok(Cmd { query, file_name, case_insensitive:b});
    }
}

pub fn run(cmd: Cmd) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(cmd.file_name)?;
    if cmd.case_insensitive {
        for l in search(cmd.query, &contents) {
            println!("{}", l)
        }
    }else {
        for l in search_case_insensitive(cmd.query, &contents) {
            println!("{}", l)
        }
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


pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let low_query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(& low_query) {
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

    #[test]
    fn one_result1() {
        let query = "ducT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_insensitive(query, contents)
        );
    }

}

