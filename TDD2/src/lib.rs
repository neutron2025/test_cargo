use std::fs;
use std::error::Error;
use std::env;

pub struct Config{  //业务逻辑移到lib.rs之后记得添加pub   
    pub query:String,
    pub filename:String,
    pub case_sensitive:bool,

}

 impl Config{
    pub fn new(args:&[String])->Result<Config,&'static str>{  //添加pub
        if args.len()<3{
            return Err("need three arguments");
        }
        let query = args[1].clone(); //这里的切片没有实现Copy trait 
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();  //如果出现CASE_INSENSITIVE就不出现错误,返回false，is_err表示是否出现错误，如果没出现错误，整个表达式返回false 这里只关心CASE_INSENSITIVE出不出现，
        Ok(Config{query,filename,case_sensitive}) // 返回这里需要有所有权
    
     }
 }

 pub fn run(config:Config)->Result<(),Box<dyn Error>>{   //添加pub
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive{
        search(&config.query, &contents)
    }else{
        search_case_insensitive(&config.query, &contents)
    };
    for line in results{
        println!("{}",line);
    }
    Ok(())
 }



pub fn search<'a>(query:&str,contents:&'a str)->Vec<&'a str>{ //返回的切片是从content来的，生命周期要标注
    let mut results =Vec::new();

    for line in contents.lines(){
        if line.contains(query){   
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query:&str,contents:&'a str)->Vec<&'a str>{ //返回的切片是从content来的，生命周期要标注
    let mut results =Vec::new();
    let query = query.to_lowercase();


    for line in contents.lines(){
        if line.to_lowercase().contains(&query){   
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, product.
pick three.
Duck tape";
        assert_eq!(vec!["safe, fast, product."],search(query,contents)); //测试失败，因为不等
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, product.
pick three.
Trust me.";
        assert_eq!(vec!["Rust:","Trust me."],search_case_insensitive(query,contents)); //测试失败，因为不等  实现函数search_case_insensitive返回为空vec![]
    }
}