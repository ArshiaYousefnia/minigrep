use std::env;

pub struct Config {
    query: String,
    filename: String,
    ignore_case: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let query = args[1].clone();
        let filename = args[2].clone();

        if args.len() < 3 {
            return Err("not enough arguments provided");
        }

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, filename, ignore_case })
    }

    pub fn to_string(&self) -> String {
        format!("query: {}, filename: {}", self.query, self.filename)
    }

    pub fn get_ignore_case(&self) -> bool {
        self.ignore_case
    }

    pub fn get_query(&self) -> &str {
        &self.query
    }

    pub fn get_filename(&self) -> &str {
        &self.filename
    }
}