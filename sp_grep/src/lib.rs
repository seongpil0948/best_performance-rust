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

    for line in search(&config.query, &contents) {
        println!("Matching \n Result: {}", line)
    }
    
    Ok(())
}


// configure as $ cargo test
#[cfg(test)]
mod test {
    use super::*; // import from ../(super)

    #[test]
    fn one_result() {
        let query = "good";
        let contents = "\
        Rust is Good.. ><
        hahaha    
        ";

        assert_eq!(
            vec!["Rust is Good.. ><"],
            search(query, contents)
        );
    }
}

// 'a : LT variable
// contents is must connect return LT
// contents 는 빌려온 데이터다. 그리고 수명이 이후로도 지속되어야 한다.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    // to_lowercase is create new String
    // shadowing and query become String(heap)(Clone) not str(string slice)
    // but contains accept only str
    let query = query.to_lowercase();
    // iter by each line
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}