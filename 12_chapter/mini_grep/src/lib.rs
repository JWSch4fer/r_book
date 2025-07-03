
use std::error::Error;
use std::fs;

pub fn run(config: Config) ->
Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    //print!("With contents\n{contents}");

    for line in search(&config.query, &contents){
        println!("{line}");
    }

    Ok(())
}

//be sure to group any information that only has meaning when
//it is together
pub struct Config{
    pub query: String,
    pub file_path: String
}

//separate logic for parsing command line arguments
//This keeps main interpretable.
impl Config{
    pub fn build(args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 3 {
            panic!("Not enough arguments were provided...\n")
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "/
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

pub fn search<'a>(query: &str, contents:&'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}