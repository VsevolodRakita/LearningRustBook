use std::env;
use std::process;

use chapter12minigrep::Config;

fn main() {
    let args: Vec<String> =env::args().collect();

    let config = Config::new(&args).unwrap_or_else( |err|{
        eprintln!("Problem parsing arguments: {}",err);
        process::exit(1);
    });

    println!("{:?}",config);

    println!("Looking for {} in {}.", config.query, config.filename);
    if let Err(e)= chapter12minigrep::run(config){
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
    
}




