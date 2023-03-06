use std::fmt::Display;
use std::fmt::Debug;
pub trait Summary{
    fn summarize(&self)->String;
    fn summary_author(&self)->String{   //这个是trait的默认实现，可以在NewArticle实例上调用默认实现，即使NewArticle没实现
        format!("===={}",self.summarize()) //默认实现的方法可以调用trait中的其他方法，即使这些方法没有默认实现。当具体类型调用summary_author时，必须实现summarize
    }

}

pub struct NewArticle{
    pub headline:String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub reteet: bool,
}

impl Summary for NewArticle{
    fn summarize(&self)->String{ //当具体类型调用默认实现summary_author()时，必须实现summarize()
        format!("{} by {} {}",self.headline,self.author,self.location)
    }

}
impl Summary for Tweet{
    fn summarize(&self)->String{ //当具体类型调用默认实现summary_author()时，必须实现summarize()
        format!("{}:{}",self.username,self.content)

    }
    fn summary_author(&self)->String{  
        format!("====默认实现的重写实现") //默认实现的重写实现
    }
}
/**
 * 将trait作为参数
 */
pub fn notify1(item1: impl Summary,item2: impl Summary){  //impl trait 语法 将trait作为参数（简单情况），可以传Tweet，NewArticle
    println!("break news:{}",item1.summarize());
}

pub fn notify2<T: Summary>(item1: T,item2: T){  //trait bound语法 将trait作为参数（复杂情况），可以传Tweet，NewArticle
    println!("break news:{}",item1.summarize());//impl trait 语法 是trait bound语法的语法糖
}

/**
 * 指定多个trait的两种写法
 */
pub fn notify3(item1: impl Summary + Display){  //item1 实现了Summary和Display 这两个trait
    println!("break news:{}",item1.summarize());
}
pub fn notify4<T: Summary + Display>(item1: T){ //item1 实现了Summary和Display 这两个trait
    println!("break news:{}",item1.summarize());
}

/**
 * 指定多个trait的两种写法 如果泛型太多，可以使用where
 */


pub fn notify5<T: Summary + Display, U: Clone + Debug>(a: T, b: U)->String{ //a 实现了Summary和Display 这两个trait
    format!("break news:{}",a.summarize())
}

pub fn notify6<T,U>(a: T , b: U)->String   //where使用   a 实现了Summary和Display 这两个trait
where
    T:Summary + Display,
    U: Clone + Debug,
{ 
    format!("break news:{}",a.summarize())
}

// /**
//  * 将trait作为返回类型，返回类型只能有一种可能
//  */ impl trait 只能返回确定的同一种类型，返回可能不同的类型，代码会报错

// pub fn notify7(flag: bool)->impl Summary{  //返回类型只可能是一个，Summary实现了俩个，这里面有两种可能NewArticle，Tweet所以报错
//     if flag{
//         NewArticle{
//             headline: String::from("headline"),
//             content: String::from("content"),
//             author: String::from("author"),
//             location: String::from("location"),
//         }
//     }else{
//         Tweet{
//             username: String::from("horse_ebook"),
//             content: String::from("of cause you may already know"),
//             reply: false,
//             reteet: false,
//         }
//     }
// }


