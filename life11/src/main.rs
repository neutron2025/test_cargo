use std::fmt::Display;
fn main() {
    let string1 =String::from("abcd");
    let string2  = "xyz";
    let result = longest2(string1.as_str(), string2);
    println!("Hello, world!");
   
    {
        let ss= "asda"; //这个是静态的，生命周期是全局。整个程序执行期内都有效
    }
    let s: &'static str = "i have a static time"; //所有的字符串字面值都有’static生命周期
}

// fn longest(x:&str,y:&str)->&str{ //跟函数签名有关系，不知道生命周期，无法返回。因为不知道借用的值来自x 还是y
//     if x.len()>y.len(){
//         x
//     }else{
//         y
//     }
// }

fn longest2<'a>(x:&'a str,y:&'a str)->&'a str{ //尖括号是泛型生命周期，三个都添加生命周期，表明x y 和返回三个生命周期是一样的
    if x.len()>y.len(){
        x
    }else{
        y
    }
}
/**
 * 生命周期的标注，只是描述了多个引用的生命周期的关系，并不会影响生命周期
 * 单个生命周期的引用没有意义
 * longest2<'a> 返回的是x 和 y 中生命周期较短的那一个
 */

fn longest3<'a>(x:&'a str,y:&'a str)->&'a str{ 
    let result = String::from("asdasd");
    result.as_str() //这里的result已经离开作用域，（生命周期已经完结）被清理了，但是返回值的引用却还要在外部调用它。造成悬垂引用
}

fn longest4<'a>(x:&'a str,y:&'a str)->String{ 
    let result = String::from("asdasd");
    result //我直接返回值，这就相当于把函数的所有权移交给函数的调用者了，这块内存也应该是由函数的调用者来清理
}

//根本上讲 生命周期语法 就是用来关联函数的不同参数以及返回值的生命周期的 


struct ImportantExcerpt<'a> {
    part: &'a str,     //里面的part引用，存活的必须必结构体实例要长
}

fn test(){
    let novel = String::from("sdsssc. ssssssss");
    let first_sentence  =novel.split('.')  //first_sentence这个引用是从 52-59有效  实例i
    .next()
    .expect("Could not found a .");

    let i = ImportantExcerpt{//实例i是从56-58有效  ，因此里面的part生命周期覆盖 i  这段代码没问题
        part: first_sentence,
    };
}

/**
 * 我们知道，每个引用都有生命周期
 * 需要为使用生命周期的函数或者struct 指定生命周期参数
 * 生命周期省略是由编译器推断的。很明显的就不需要添加，由编译器推断
 * 
 * 输入生命周期：方法/函数的参数中   应用规则1
 * 输出生命周期：方法/函数的返回值中 应用规则2，3
 * 
 * 生命周期省略的三个规则 适用于fn 函数 impl块
 * 规则1 每个引用类型都有自己的生命周期   
 * 规则2 如果只有一个生命周期参数 ，那么该生命周期会被赋给所有输出生命周期参数   
 * 规则3 如果有多个输入生命周期参数，但其中一个是&self 或&mut self (因为这是方法)，那么self 的生命周期会被赋给所有的输出生命周期参数
 * 
 */

 //假如我们是编译器  使用三条省略规则步步推断
 fn first_world(s:&str)->&str{}
 fn first_world<'a>(s: &'a str)->&str{}  //1
 fn first_world<'a>(s: &'a str)->&'a str{} //2

 fn longest(x:&str,y:&str)->&str{}
 fn longest<'a,'b>(x:&'a str,y:&'b str)->&str{} //1  此时2,3规则不适用，却没有计算出返回类型的生命周期，所以编译器报错

///在哪申明和使用生命周期参数依赖于生命周期参数是否和字段，方法的参数和返回值有关
/// struct字段的生命周期名在impl 后声明 在struct名后使用，这些生命周期是struct类型的一部分
/// impl 块内的方法签名：引用必须绑定于struct字段引用的生命周期，或者引用是独立的也可以
/// 生命周期省略规则经常使得方法中的生命周期标注不是必须的

 //方法定义中的生命周期标注
struct ImportantExcerptrr<'a> {
    part: &'a str,     //里面的part 引用存活的必须必结构体实例要长
}
impl <'a> ImportantExcerpt<'a>{  //此处两个生命周期标注不能忽略，根据第一条，也不用为self 标注生命周期
    fn level(&self)->i32{
        3
    }
    fn announce_and_return_part(&self,announcement: &str)->&str{ //根据第三条，所有生命周期都可推断出来，可省略标注
        println!("Attention please:{}",announcement);
        self.part
    }

}


//静态生命周期 'static 标注的生命周期是整个程序持续时间，
let s: &'static str = "i have a static time";//所有的字符串字面值都有’static生命周期


//一个使用了泛型参数类型，trait bound  以及生命周期的例子
fn announce_and_return_par<'a,T>(x:&'a str, y:&'a str,ann:T)->&'a str  //生命周期也是泛型的一种
where
    T:Display,  //根据where约束 T 可以替换为任何实现了 Display trait 的类型
{
    println!("Announcement!{}",ann);
    if x.len()> y.len(){
        x
    }else{
        y
    }

}

