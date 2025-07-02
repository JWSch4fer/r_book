use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(args);

    let config_args= Config::new(&args);
    println!("Search for {}",config_args.query);
    println!("In file {}", config_args.file_path);

    let contents = fs::read_to_string(config_args.file_path)
    .expect("Should have been able to rea the file");

    print!("With contents\n{contents}");
}

//be sure to group any information that only has meaning when
//it is together
struct Config{
    query: String,
    file_path: String
}

//separate logic for parsing command line arguments
//This keeps main interpretable.
impl Config{
    fn new(args: &[String]) -> Config{
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path } 
    }
}
