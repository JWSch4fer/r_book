use std::{env,process};
use mini_grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(args);

    let config_args = Config::build(&args)
    .unwrap_or_else(|err| {
        eprintln!("Problem pargisng arguments: {err}");
        process::exit(1);
    });
    println!("Search for {}",config_args.query);
    println!("In file {}", config_args.file_path);

    if let Err(e) = mini_grep::run(config_args){
        eprintln!("Application Error: {e}");
        process::exit(2);
    }
}