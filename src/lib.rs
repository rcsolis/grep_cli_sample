use std::{fs, io, error};
// Define Config struct
// with public fields
pub struct Config{
    pub query_string: String,
    pub file_path: String,
}
// Implement Config struct methods
impl Config {
    // Function to build a Config struct instance
    pub fn build(args: &[String])-> Result<Config, &'static str>{
        if args.len() < 3{
            return Err("Not enough arguments");
        }
        // Save arguments in variables
        let query_string = args[1].clone();
        let file_path = args[2].clone();
        // Return a Config struct instance
        Ok(Config { query_string, file_path})
    }
}
// Funciton to read file contents
fn read_file_contents(file_path: &str)-> Result<String, io::Error>{
    // Read file contents
    fs::read_to_string(file_path)
}
// Public function to run the program logic
pub fn run (config: Config)-> Result<(), Box<dyn error::Error>>{
    println!("Searching for: {} in file: {}", config.query_string, config.file_path);
    // Read file contents
    let file_contents = read_file_contents(&config.file_path)?;
    println!("File contents: {}", file_contents);
    
    Ok(())
}