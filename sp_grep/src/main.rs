
use std::env;
use std::process;

use sp_grep::Config;

fn main() {
    // if false -> exist variable
    let exist_variable: bool = env::var("SP_ENV_VARIABLE").is_err();
    println!("exist_variable ? : \n {} ", exist_variable);

    // return cmd args iterator
    let args: Vec<String> = env::args().collect();
    
    // Err Catch From Pipe(|err|) -> Closure
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error In Parsing Args \n {} ", err);
        // return ,lppexit code 1
        process::exit(1);
    });

    // Handling Only Error
    if let Err(e) = sp_grep::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}



