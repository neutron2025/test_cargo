
/**
 * 测试驱动开发
 * 
 *1、 编写一个会失败的测试，运行，确保它是按照预期失败的
 *2、 编写或者修改代码，让他通过测试
 *3、 重构第二步的代码，确保通过
 * 
 *4、返回第一步 ，继续下一步开发
 */
use TDD::Config;
use std::env;
use std::process;


fn main(){
    let args: Vec<String> = env::args().collect();
    // let (query,filename) = parse_config(&args);
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing argument:{}",err);
        process::exit(1);   //调用exit程序会立即退出，状态码为1
    });
    // run(config);//如果这样调用，编译器会提示添加错误处理代码
   
    if let Err(e)  = TDD::run(config){    //由于这里没有返回错误，返回为空元组，所以只要检查是否错误就行，不需要打开检查类似unwrap_or_else这样
        println!("Application error:{}",e);
        process::exit(1);
    }

}

