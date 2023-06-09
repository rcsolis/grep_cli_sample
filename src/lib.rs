use std::{fs, io, error};
// Define Config struct
// with public fields
pub struct Config{
    pub query_string: String,
    pub file_path: String,
    pub ignore_case:bool,
}
// Implement Config struct methods
impl Config {
    // Function to build a Config struct instance
    pub fn build(
        mut args: impl Iterator<Item = String>,
        ignore_case: bool
    )-> Result<Config, &'static str>{
        // Ignore the first element
        args.next();

        // Get query string
        let query_string = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        // Get file path
        let file_path = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        // Return a Config struct instance
        Ok(Config { query_string, file_path, ignore_case})
    }
}
// Funciton to read file contents
fn read_file_contents(file_path: &str)-> Result<String, io::Error>{
    // Read file contents
    fs::read_to_string(file_path)
}
// Function to search for a query string in a file contents
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    // let mut result: Vec<&str> = Vec::new();
    // for line in contents.lines(){
    //     if line.contains(query){
    //         result.push(line);
    //     }
    // }
    // return result;
    contents.lines()
    .filter(|line| line.contains(query))
    .collect()
}

fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result: Vec<&str> = Vec::new();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
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
    
    let results = if config.ignore_case {  
        println!("Ignoring case search");
        search_case_insensitive(&config.query_string, &file_contents)
    }else{
        search(&config.query_string, &file_contents)
    };
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

    #[test]
    fn case_insensitive_one_result(){
        let query = "me";
        let contents = "\
Rust:
safe,fast,productive.
Pick three.
Trust me.
I'm want to be a Rustacean!";
        
        assert_eq!(
            vec!["Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn case_insensitive_multiple_results(){
        let query = "rUsT";
        let contents = "\
Rust:
safe,fast,productive.
Pick three.
Trust me.
I'm want to be a Rustacean!";
        
        assert_eq!(
            vec!["Rust:", "Trust me.", "I'm want to be a Rustacean!"],
            search_case_insensitive(query, contents)
        );
    }
}