use std::fs;
use std::error::Error;

pub struct Config{  //业务逻辑移到lib.rs之后记得添加pub   
    query:String,
    filename:String,
}

//  fn parse_config(args:&[String])->(&str,&str){  
//     let query = &args[1];
//     let filename = &args[2];
//     (query,filename)

//  }
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

//  fn run(config:Config){
//     let contents = fs::read_to_string(config.filename).expect("WRONG");
//     println!("with cntent:\n{}",contents);
//  }

 pub fn run(config:Config)->Result<(),Box<dyn Error>>{   //添加pub
    let contents = fs::read_to_string(config.filename)?;
    println!("with cntent:\n{}",contents);
    Ok(())
 }