use std::fs;
use std::error;


#[cfg(test)]
mod tests;

pub mod app;

pub mod parser;

pub mod globals;

pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut vector: Vec<&str> = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
            vector.push(line);
        }
    }

    vector
}

pub fn search_case_insensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut vector: Vec<&str> = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        let lowercase: &str = &line.to_lowercase()[0..];
        
        if lowercase.contains(&query[0..]) {
            vector.push(line);
        }
    }

    vector
}

pub fn run(config: app::Config) -> Result<(), Box<dyn error::Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.is_case_sensitive {
        search(config.search_string, &contents[0..])
    } else {
        search_case_insensitive(config.search_string, &contents[0..])
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

