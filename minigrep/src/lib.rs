use std::{fs, error::Error, f32::consts::E, env};

pub struct Config{
    query:String,
    filename:String,
    case_sensitive: bool
}

// impl Config{
//     pub fn new (args:&[String])->Result<Config,&str>{
//         if args.len()<3{
//             return Err("参数不足");
//         }
//         let query = args[1].clone();
//         let filename = args[2].clone();
//         let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
//         Ok(Config {query, filename, case_sensitive })
//     }
// }
impl Config{
    pub fn new(mut args:std::env::Args)->Result<Config,&'static str>{
        if args.len()<3{
            return Err("参数不足");
        }
        args.next();

        let query = match args.next() {
            Some(it) => it,
            None => return Err("无法找到查询字符串"),
        };
        let filename = match args.next() {
            Some(it) => it,
            None => return Err("无法找到查询文件"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {query, filename, case_sensitive })
    }
}

pub fn run(config:Config)->Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive{
        search(&config.query, &contents)
    }else{
        search_case_insensitive(&config.query, &contents)
    };
    for ele in  results{
        println!("{}",ele);
    };
    Ok(())
}

pub fn search<'a>(query:&'a str,contents:&'a str)-> Vec<&'a str>{
    contents.lines().filter(|line|
        line.contains(query)
    ).collect()
}

pub fn search_case_insensitive<'a>(query:&'a str,contents:&'a str)-> Vec<&'a str>{
    contents.lines().filter(|line|{
        let query = query.to_lowercase();
        line.to_lowercase().contains(&query)
    }
    ).collect()
}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test1(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Duct page
Pick three.";
        assert_eq!(vec!["safe, fast, productive."],search(query,contents))
    }

    #[test]
    fn test2(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Duct page
Pick three.";
        assert_eq!(vec!["safe, fast, productive.","Duct page"],search_case_insensitive(query,contents))
    }
}