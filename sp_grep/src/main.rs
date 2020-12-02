
use std::env;
use std::process;

use sp_grep::Config;

fn main() {
    // return cmd args iterator
    let args: Vec<String> = env::args().collect();
    // Err Catch From Pipe(|err|) -> Closure
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error In Parsing Args \n {} ", err);
        // return ,lppexit code 1
        process::exit(1);
    });

    // Handling Only Error
    if let Err(e) = sp_grep::run(config) {
        println!("Application Error: {}", e);
        process::exit(1);
    }
}



