use std::fs;
use std::error::Error;

pub struct Config {
    query: String,
    filename: String
}

impl Config {
    // ` = LT variable
    //  static = LT is until the end of program
    pub fn new(args: &[String]) -> Result<Config, &'static str>  {
        let length: usize = args.len();
        if length == 1 {
            return Err(" Args is not at all ");
        }
        else if length < 3 {
            return Err(" Insufficient(불만족) Num Of Arg");
        }
        // args[0] is bin file name
        // cloning is more Inefficient than save Reference data
        // because, arg's OW is not parse_config
        // but, LT manage is more easy
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config{ query, filename })
    }
}


/* 
    1. Box(dyn Error) from Error Trait>
        Return Various Error at Unexpected
    2. ? operator return Error instead panic from expect(func)

*/ 
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("File Content: \n {}", contents);
    Ok(())
}