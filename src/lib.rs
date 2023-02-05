use std::fs;
use std::error;
use std::env;

#[cfg(test)]
mod tests;


pub struct Config<'u> {
    file_path: &'u String,
    search_string: &'u String,
    is_case_sensitive: bool
}

impl<'u> Config<'u> {
    pub fn build(args: &Vec<String>) -> Result<Config, String> {

        if args.len() < 3 {
          return Err("Insufficient arguments passed".to_string());  
        }

        let search_string: &String = &args[1];
    
        let file_path: &String = &args[2];
    
        let is_case_sensitive = env::var("IGNORE_CASE");

        match is_case_sensitive {
            Ok(_val) => {
                Ok(Config {
                    file_path,
                    search_string,
                    is_case_sensitive: false
                })
            },
            _=>{
                Ok(Config {
                    file_path,
                    search_string,
                    is_case_sensitive: true
                })
            }
        }

        
    }
}

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

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
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

