use std::env;
use std::process;
// Import libray crate
use grep_cli_sample::{Config, run};
// Main function
fn main(){
    // Get command line arguments and store them in a vector
    // let args: Vec<String> = env::args().collect();
    // Get environment variable
    let ignore_case = env::var("GREP_IGNORE_CASE").is_ok();
    // Save arguments in variables, handle error if any and exit with code 1
    // using unwrap_or_else method of Result type and a closure that takes
    // the error as argument and exits the program
    let config: Config = Config::build(env::args(), ignore_case).unwrap_or_else(|err|{
        eprintln!("Error: Cannot parse arguments. {}", err);
        process::exit(1);
    });
    // Runs logic, handle error if any and exit with code 1 using
    // if let statement
    if let Err(e) = run(config) {
        eprintln!("Error: Application error: {}", e);
        process::exit(1);
    }
}