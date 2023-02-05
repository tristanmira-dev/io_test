use std::env;
use std::process;
use io_test::*;
fn main() {
    let args: Vec<String> = env::args().collect();


    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    match run(config) {
        Err(e) => println!("Error: {}", e),
        _ => ()
    }    

}




