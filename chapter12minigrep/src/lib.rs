use std::fs;

use std::error::Error;

pub fn run(config: Config)->Result<(),Box<dyn Error>>{
    let content= fs::read_to_string(config.filename)?;
    
    let result= if config.case_sensitive{
        search(&config.query, &content)
    }
    else {
        search_case_insensitive(&config.query, &content)
    };

    for line in result{
        println!("{}",line);
    }

    Ok(())
}

#[derive(Debug)]
pub struct Config{
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config{
    pub fn new(args: &[String]) -> Result<Config, &str>{
        if args.len()<3{
            return Err("Not enough arguments.");
        }
        let query=args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = args.len()<4 || args[3]!="-ci";

        Ok(Config{ query, filename, case_sensitive})
    }
}

pub fn search<'a>(queary: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.contains(queary){
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str>{
    let query=query.to_lowercase();
    let mut result =Vec::new();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            result.push(line);
        }
    }
    result
}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result(){
        let queary = "duct";
        let contents = "\
            Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(queary, contents))
    }

    #[test]
    fn case_insensitive(){
        let queary = "rUsT";
        let contents = "\
            Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], 
            search_case_insensitive(queary, contents))
    }

    #[test]
    fn test2(){
        let queary = "as";
        let contents = "\
        Something, Something, skiij
sdsdgsadga
dsgdsgsa dfgadssasg. sdaggdsg
Sdsags.  
sdgadsgsa. adsgsadgas, sadgsadg.
dgg. As";
        assert_eq!(vec!["dsgdsgsa dfgadssasg. sdaggdsg", "sdgadsgsa. adsgsadgas, sadgsadg.", "dgg. As"], 
            search_case_insensitive(queary, contents))
    }

    #[test]
    fn test3(){
        let queary = "as";
        let contents = "\
        Something, Something, skiij
sdsdgsadga
dsgdsgsa dfgadssasg. sdaggdsg
Sdsags.  
sdgadsgsa. adsgsadgas, sadgsadg.
dgg. As";
        assert_eq!(vec!["dsgdsgsa dfgadssasg. sdaggdsg", "sdgadsgsa. adsgsadgas, sadgsadg."], 
            search(queary, contents))
    }

}