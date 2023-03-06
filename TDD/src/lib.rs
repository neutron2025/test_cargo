use std::fs;
use std::error::Error;

pub struct Config{  //业务逻辑移到lib.rs之后记得添加pub   
    query:String,
    filename:String,
}

 impl Config{
    pub fn new(args:&[String])->Result<Config,&'static str>{  //添加pub
        if args.len()<3{
            return Err("need three arguments");
        }
        let query = args[1].clone(); //这里的切片没有实现Copy trait 
        let filename = args[2].clone();
        Ok(Config{query,filename}) // 返回这里需要有所有权
    
     }
 }

 pub fn run(config:Config)->Result<(),Box<dyn Error>>{   //添加pub
    let contents = fs::read_to_string(config.filename)?;
    for line in search(&config.query, &contents){
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

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, product.
pick three";
        assert_eq!(vec!["safe, fast, product."],search(query,contents)); //测试失败，因为不等
    }
}