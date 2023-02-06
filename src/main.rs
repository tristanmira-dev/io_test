use std::env;
use std::process;
use io_test::{app, run};


fn main() {
    let args: Vec<String> = env::args().collect();


    let config = app::Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    match run(config) {
        Err(e) => println!("Error: {}", e),
        _ => ()
    }    

}




