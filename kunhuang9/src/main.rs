use std::fs::File;
use std::io::ErrorKind;
use std::io::{self,Read};
fn main() {
    println!("Hello, world!");
    // panic!("crush the program");  不可恢复的错误，panic!宏

  
    let v=vec![1,2,3];
  

    //可恢复的错误，Result枚举
    let f=File::open("hello.txt"); //这里返回Result
    let f = match f{
        Ok(file)=>file,
        Err(error)=>match error.kind(){
            ErrorKind::NotFound=>match File::create("hello.txt"){
                Ok(fc)=>fc,
                Err(e)=>panic!("Error creating file {}",e),
            },
            other_error=>panic!("Error opening file{:?}",other_error),
        },

    };

    let result =read_username_from_file();//此函数达到了传播错误的作用

    loop{ //这里体现的是通过有效性检查产生实例
        let guess="32";
        let guess: i32 = match guess.trim().parse(){
        Ok(num)=>num,
        Err(_)=>continue,
    };

    let guess = guess::new(guess);
    }

}

fn gailiang(){   //闭包改良
    let f= File::open("hello.txt").unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound{
            File::create("hello.txt").unwrap_or_else(|error|{
                panic!("Error creating file : {:?}",error);
            })
        }else {
            panic!("Error  opening file: {:?}",error);
        }
    });
}


fn gailiang2(){//unwrap  是match表达式的一个快捷方法。缺点就是不可以自定义错误信息
    let f=File::open("hello.txt").unwrap(); //这一行代码相当于自动抛出异常 如果Result结果是OK 则返回OK里面的值，结果是Err，则调用panic!宏
    let f=File::open("hello.txt").expect("无法打开文件hello.txt"); //expact可以自定义 panic!宏的错误信息

}

fn read_username_from_file()->Result<String,io::Error>{ //此函数达到了传播错误的作用
    let f=File::open("hello.txt");
    let mut f=match f{
        Ok(file)=>file,
        Err(e)=> return Err(e)

    };
    let mut s=String::new(); //读到s
    match f.read_to_string(&mut s){
        Ok(_)=>Ok(s),
        Err(e)=>Err(e),
    }


}

fn read_username_from_file2()->Result<String,io::Error>{ //? 运算符是错误传播的一种快捷方式，正常则返回Ok()里面的值，返回类型为Result<T,E>的函数才能用？
    let mut f=File::open("hello.txt")?;
    let mut s=String::new();
    f.read_to_string(&mut s)?;
    Ok(s)

}
fn read_username_from_file3()->Result<String,io::Error>{ //链式优化
    let mut s=String::new();
    let mut f=File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)

}

pub struct guess{
    value:i32,
}
impl guess{
    pub fn new(value:i32)->guess{ 
        if value<1||value>100{
            panic!("{}",value);
        }
        guess {value}
    }

    pub fn value(&self) ->i32{
        self.value
    }
}
///总体原则
/// 优先考虑 返回result
/// 
/// 可以使用panic的 
/// 演示概念  unwrap
/// 原型代码 ,测试  unwrap expect
/// 
/// 你的代码可能处于损坏的情况下使用  panic
