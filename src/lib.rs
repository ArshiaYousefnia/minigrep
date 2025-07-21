use std::error::Error;
use std::fs;
use search::{search_case_sensitive, search_case_insensitive};
use structs::Config;
pub mod structs;
mod search;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.get_filename())
        .expect("error reading file");

    let results = if config.get_ignore_case() {
        search_case_insensitive(&config.get_query(), &contents)
    } else {
        search_case_sensitive(&config.get_query(), &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}



#[cfg(test)]
mod tests;