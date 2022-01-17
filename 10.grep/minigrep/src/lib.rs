use std::env;
use std::error::Error;
use std::fs;
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        let mut case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        println!("case_sensitive: {}", case_sensitive);
        if args.len() < 2 {
            return Err("not find \"search string\" and \"filename\"");
        } else if args.len() < 3 {
            return Err("not find \"filename\"");
        } else if args.len() >= 4 {
            if args[3] == "true" {
                case_sensitive = false
            }
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<String> {
    let mut res_arr: Vec<String> = vec![];
    for line in contents.lines() {
        // if line.find(query) != None {
        if line.contains(query) {
            res_arr.push(String::from(line));
        }
    }
    return res_arr;
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<String> {
    let mut res_arr: Vec<String> = vec![];
    let low_query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&low_query) {
            res_arr.push(String::from(line));
        }
    }
    return res_arr;
}

pub fn run(config: Config) -> Result<Vec<String>, Box<dyn Error>> {
    let content: String = fs::read_to_string(config.filename)?;
    println!("content: \n {}", content);
    println!("---------------------------");
    let res_arr;
    if config.case_sensitive {
        res_arr = search(&config.query, &content);
    } else {
        res_arr = search_case_insensitive(&config.query, &content);
    }
    for res in &res_arr {
        println!("{}", res)
    }
    Ok(res_arr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn tow_result() {
        let query = "dUcT";
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
