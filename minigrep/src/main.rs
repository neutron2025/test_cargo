use minigrep::Config;
use std::env;
use std::process;



// fn main() {
//     let args: Vec<String> = env::args().collect();
//     println!("{:?}",args);

//     let query = &args[1];
//     let filename = &args[2];
//     println!("search for:{}",query);
//     println!("in file:{}",filename);
//     let contents = fs::read_to_string(filename).expect("WRONG");
//     println!("with cntent:\n{}",contents);

//     // cargo run abcd poem.txt
//     //一个函数只做一件事

// }


/**
 * 二进制程序关注点分离的指导性原则
 * 将程序拆分为lib.rs 和main.rs 将业务逻辑放入lib.rs
 * 
 * 拆分之后的main功能有：
 * 调用参数值进行命令行解析逻辑
 * 进行其他配置
 * 调用lib.rs中的run函数
 * 处理run函数可能存在的错误
 */

fn main(){
    let args: Vec<String> = env::args().collect();
    // let (query,filename) = parse_config(&args);
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing argument:{}",err);
        process::exit(1);   //调用exit程序会立即退出，状态码为1
    });
    // run(config);//如果这样调用，编译器会提示添加错误处理代码
   
    if let Err(e)  = minigrep::run(config){    //由于这里没有返回错误，返回为空元组，所以只要检查是否错误就行，不需要打开检查类似unwrap_or_else这样
        println!("Application error:{}",e);
        process::exit(1);
    }

}

