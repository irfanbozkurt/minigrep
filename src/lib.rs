use regex::Regex;
use std::{error::Error, fs};

pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
    pub is_case_insensitive: bool,
}

impl<'a> Config<'a> {
    // Borrow the args and not own them
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let arg_len = args.len();
        if arg_len < 3 {
            return Err("Provide at least 2 arguments: query filename");
        }

        Ok(Config {
            query: &*args[1],     // Refs to args will be alive
            file_path: &*args[2], // after this scope
            is_case_insensitive: arg_len > 3 && &*args[3] == "i",
        })
    }
}

pub struct QueryParams<'a> {
    query: &'a str,
    content: &'a str,
    is_case_insensitive: bool,
}

impl<'a> QueryParams<'a> {
    pub fn new(query: &'a str, content: &'a str, is_case_insensitive: bool) -> QueryParams<'a> {
        QueryParams {
            query,
            content,
            is_case_insensitive,
        }
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.file_path)?;

    let query_params = QueryParams::new(config.query, &file_content, config.is_case_insensitive);

    let matched_lines = search(query_params);

    for line in matched_lines {
        println!("{line}");
    }

    return Ok(());
}

pub fn search<'a>(params: QueryParams<'a>) -> Vec<&'a str> {
    let reg_case_prefix = match params.is_case_insensitive {
        true => "(?i)",
        false => "",
    };
    let pattern_for_lines = format!(r"{}.*{}.*", reg_case_prefix, params.query);
    let reg = Regex::new(&pattern_for_lines).unwrap();
    return reg.find_iter(params.content).map(|m| m.as_str()).collect();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let params = QueryParams::new(
            "duct",
            "\
Rust:
safe, fast, productive.
Pick three.
		",
            false,
        );

        assert_eq!(vec!["safe, fast, productive."], search(params));
    }

    #[test]
    fn case_insensitive() {
        let params = QueryParams::new(
            "dUcT",
            "\
Rust:
safe, fast, productive.
Pick three.
",
            true,
        );

        assert_eq!(vec!["safe, fast, productive."], search(params));
    }
}
