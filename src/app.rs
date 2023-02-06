use std::env;

pub struct Config<'u> {
    pub file_path: &'u String,
    pub search_string: &'u String,
    pub is_case_sensitive: bool
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