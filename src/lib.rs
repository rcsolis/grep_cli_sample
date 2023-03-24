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
// Function to search for a query string in a file contents
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{

    let mut result: Vec<&str> = Vec::new();

    for line in contents.lines(){
        if line.contains(query){
            result.push(line);
        }
    }

    return result;
}

// Public function to run the program logic
pub fn run (config: Config)-> Result<(), Box<dyn error::Error>>{
    println!("Searching for: {} in file: {}", config.query_string, config.file_path);
    // Read file contents
    let file_contents = read_file_contents(&config.file_path)?;
    // println!("File contents: \n {}", file_contents);
    // Search for query string in file contents
    let results = search(&config.query_string, &file_contents);
    if results.len()==0{
        return Err("Query string not found, try again".into());
    }
    // Print results
    println!("\nQuery string found! \nTotal matches:{}", results.len());
    println!("Found in lines: ");
    for result in results{
        println!("->{}", result);
    }
    Ok(())
}

// Create tests module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe,fast,productive.
Pick three.
I'm want to be a Rustacean!";
        
        assert_eq!(
            vec!["safe,fast,productive."],
            search(query, contents)
        );
    }
    #[test]
    fn multiple_results(){
        let query = "Rust";
        let contents = "\
Rust:
safe,fast,productive.
Pick three.
I'm want to be a Rustacean!";
        
        assert_eq!(
            vec!["Rust:", "I'm want to be a Rustacean!"],
            search(query, contents)
        );
    }
}