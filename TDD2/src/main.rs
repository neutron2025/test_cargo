/**
 * 环境变量，参数选择
 */
use TDD2::Config;
use std::env;
use std::process;

fn main(){
    let args: Vec<String> = env::args().collect();
    // let (query,filename) = parse_config(&args);
    let config = Config::new(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing argument:{}",err);
        process::exit(1);   //调用exit程序会立即退出，状态码为1
    });
    // run(config);//如果这样调用，编译器会提示添加错误处理代码
   
    if let Err(e)  = TDD2::run(config){    //由于这里没有返回错误，返回为空元组，所以只要检查是否错误就行，不需要打开检查类似unwrap_or_else这样
        eprintln!("Application error:{}",e);  //这里使用了标准错误
        process::exit(1);
    }

}

//标准输出 println!   
// cargo run duct poem.txt > output.txt  标准输出 到output.txt

//标准错误 eprintln!
//cargo run > output.txt 标准错误输出到屏幕