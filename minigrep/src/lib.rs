use std::error::Error;
use std::fs;
use std::env;


pub fn run(config:Config)->Result<(),Box<dyn Error>> {
    // 从指定文件中读取数据
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    }else{
        search_case_insensitive(&config.query, &contents)
    };
    for line in results{
        println!("{}",line)
    }
    Ok(())
}

pub struct Config{
    pub query: String,
    pub filename: String,
    pub case_sensitive : bool,
}

impl Config{
    pub fn new(mut arg: env::Args)->Result<Config,&'static str>{
        if arg.len() < 3  {
            return Err("not enough argsuments");
        }
        // 因为需要的是第二个和第三个 ，所以第一个直接抛弃掉
        arg.next();
        let query = match arg.next(){
            Some(arg) =>arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match arg.next(){
            Some(arg) =>arg,
            None => return Err("Didn't get a filename"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config{query,filename,case_sensitive})
    }
}

pub fn search<'a >(query:&str,contents: &'a str)->Vec<&'a str>{
    // let mut results =Vec::new();
    //  for line in contents.lines(){
    //     if line.contains(query) {
    //         results.push(line)
    //     } 
    //  }
    //  results

    // 使用迭代器 和闭包减少代码 ，减少多余变量的维护
     contents.lines()
            .filter(|line| line.contains(query))
            .collect()
}

pub fn search_case_insensitive<'a >(query:&str,contents: &'a str)->Vec<&'a str>{
    // let mut results =Vec::new();
    // let query =query.to_lowercase();
    //  for line in contents.lines(){
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line)
    //     } 
    //  }
    //  results
    contents.lines().filter(|line| line.to_lowercase().contains(&query.to_lowercase())).collect()
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
Rust: 
safe ,fast ,productive.
Pick Thress
Duct tape.";
        assert_eq!(vec!["safe ,fast ,productive."],
                    search(query,contents))
    }
    #[test]
    fn case_insenitive(){
        let query = "rUst";
        let contents = "\
Rust: 
safe ,fast ,productive.
Pick Thress
Trust me.";
assert_eq!(vec!["Rust: ","Trust me."],
                search_case_insensitive(query,contents))

    }
}