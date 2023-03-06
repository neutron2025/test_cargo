/**
 *  闭包改进搜索
 * 
 * 零成本抽象：使用抽象时不会引入额外的运行时开销   迭代器就是如此
 */
use TDD3::Config;
use std::env;
use std::process;

fn main(){
   

    let config = Config::new(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem parsing argument:{}",err);
        process::exit(1);   //调用exit程序会立即退出，状态码为1
    });
    // run(config);//如果这样调用，编译器会提示添加错误处理代码
   
    if let Err(e)  = TDD3::run(config){    //由于这里没有返回错误，返回为空元组，所以只要检查是否错误就行，不需要打开检查类似unwrap_or_else这样
        eprintln!("Application error:{}",e);
        process::exit(1);
    }

}

//标准输出 println!   
//标准错误 eprintln!
//cargo run > output.txt 标准错误输出到屏幕
// cargo run duct poem.txt > output.txt  标准输出 到output.txt