const MAX_POINTS: u32 = 100_000;
use std::iter;
use std::pin;
use std::adapters::Enumerate;
///
/// 变量与常量，常量在绑定值以后也是不可变的，但是它与不可变得变量有很多区别
/// -不可以使用mut，常量永远都是不可变的
/// -申明常量使用const关键字，他的类型必须被标注
/// -常量可以在任何作用域内进行申明，包括全局作用域
/// -常量只可以绑定到常量表达式，无法绑定到函数的调用结果或只能在运行时才能计算出的值
/// 在rust里面常量全部使用大写申明
fn main() {
    println!("Hello, world!");
    let x = 5;
    println!("value of x is {}",x);
    let y = MAX_POINTS;
    let spaces= "    ";
    let spaces = spaces.len(); //shadow 概念  
    println!("value of x is {}",spaces);

    let guess:u32 ="42".parse().expect("not a number!");
    println!("{}",guess);

    let i: u32 = 8;
    let x = 2.0;
    let y: f32 = 3.0;
    let t= false;
    let z = '😂';
    let s: char = 'A';

    let tup:(i32,u8,f32)=(3,34,2.0); //tuple
    println!("{},{},{}",tup.0,tup.1,tup.2);
    let (x,y,z) = tup; //结构元组，以此获取元素

    let z=[1,2,3,4,5]; //数组
    let a:[i32;5] = [1,2,3,4,5];
    let a=[3;5];       //相当于 let a=[3,3,3,3,3];
    let a=[3,3,3,3,3];

    let array1=a[4];

    amother_function(8);

    let c = 6;
    let v = {
        let c = 1;
        c+1
    };
    println!("-----{}",v);

    fn five() -> i32 {
        5
    }

    fn plus_five(x:i32)->i32{
        x+5
    }


    for_yinyong();
    shiyong();
    test_jieyong();
    kebianyinyong();

    let mut s= String::from("hello world");
    let wordIndex=first_world(&s); //这里 s.clear()   wordIndex和s字符串之间无关联，推荐使用切片
    println!("{}",wordIndex);

    qiepian();

    //优化之后的可以传切片和字符串
    let  mystring = String::from("hello world");
    let wordIndex = first_worldyouhua(&mystring[..]); 

    let  mystring_literal = "hello world";
    let wordIndex2 = first_worldyouhua(mystring_literal); 

    
    shuzu_qiepian()


}

fn amother_function(x:u32){
    let number = 7;
    if x < number{              //if 表达式    ，let 后面可以跟表达式
        println!("数字小于7")
    } else if (number<x) {
        println!("数字大于7")
    }else{
        println!("异常")
    }
    println!("{}",x);
}

fn function(){
    let condition  = true;
    let number = if condition{ 5 } else { 6 };
    println!("{}",number);
}

fn xunhuan(){                   //循环
    let mut counter = 0;
    let  result= loop{
        counter +=1;
        if counter==10{
            break counter*2;
        }
    };
    println!("{}",result);
}

fn whilexunhuan(){
    let mut  number = 3;
    while number != 0{
        println!("{}",number);
        number-=number;
    }
    println!("LIFTOFF")
}

fn bianlishuzu(){
    let a=[1,2,3,4,5,6];
    let mut index=0;
    while index<5{
        println!("{}",index);
        index+=index;
    }
}

fn for_xunhuanz(){
    let a= [1,23,4,5,6,7,8,9];
    for element in a.iter(){
        println!("{}",element);
    }
}

fn for_yinyong(){
    for number in (1..4).rev(){
        println!("{}",number);
    }


}

    /**
     * 堆克隆，栈复制
     */
fn shiyong(){
    let mut s=String::from("hello");
    s.push_str("world");
    println!("{}",s);

    let s1=String::from("hel");
    let s2=s1; //此后无法调用s1，防止释放两次，s1所有权被转移给s2  除非事先实现Copy trait
    let s3=s2.clone();
    println!("{}",s3);

    let  sp:(i32,i32,i32) =(1,2,5);

}
///如果一个类型或者该类的一部分实现了Drop trait,那么rust不允许让它再实现Copy trait
///任何简单标量的组合类型都可以是Copy的
///任何需要分配内存或者某种资源的都不是Copy的
/// 拥有copy trait的类型 u32,bool, char,tuple

fn test_jieyong(){
    let s1=String::from("helo");
    let len=calculate_length(&s1);
    println!("{}",len);
}
fn calculate_length(s: &String) ->usize{
    
    s.len()
}

fn kebianyinyong(){ //不同作用域可以有多个可变引用赋值
    let mut s=String::from("hello");
    {
        let s1=&mut s;
    }
    let s3 = &s;
    let s4 = &s;
    //不可以将同时借用为可变引用和不可变引用
    // let s2 = &mut s;    

    // println!("{}{}{}",s2,s3,s4);
    
}

// fn yezhizhen()->&String{
//     let s=String::from("helloo");
//     &s  //此处引用被返回时出了作用域
// }

fn  first_world(s:&String)->usize{
    let bytes = s.as_bytes();

    for (i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }
    s.len()

}

fn qiepian(){
    let s=String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..];
    let all = &s[..];

    println!("{}{}{}",hello,world,all);
}

fn  first_worldyouhua(s:&str)->&str{
    let bytes = s.as_bytes();

    for (i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[..i];
        }
    }
    &s[..]

}

fn  shuzu_qiepian(){
    let a = [1,2,3,4,5,6];
    let slice = &a[1..3];
    println!("{}",slice[1]);
}