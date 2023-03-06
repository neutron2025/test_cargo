
//rust 是面向对象的：struct enum 包含数据， impl块为之提供了方法
//封装：为一个对象进行交互的方法就是通过它公开的API
//rust：pub
pub struct AverageCollection{
    list: Vec<i32>,
    average: f64,
}

impl AverageCollection{
    pub fn add(&mut self,value:i32){
        self.list.push(value);
        self.update_average();
    }
    pub fn remove(&mut self)->Option<i32>{
        let result  = self.list.pop();
        match result{
            Some(value)=>{
                self.update_average();
                Some(value)
            },
            None=>None,
        }
        
    }
    pub fn average(&self)->f64{
        self.average
    }

    fn update_average(&mut self){
        let total:i32 =self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

//继承：使用对象可以沿用另外一个对象的数据和行为，且无需重复定义相关代码
//rust：没有继承  默认使用trait方法来进行代码共享
//多态：私用泛型和trait 约束来实现多态

//现在很多语言都不使用继承作为内置的程序设计方案了


// 使用trait对象来存储不同类型的值
/**
 * 需求：创建一个Gui 工具，它会遍历某个元素的列表，依次调用元素的draw方法进行绘制 例如button,textfield
 * 
 * 在面向对象语言里，定义一个component父类，里面定义了draw方法，让button,textfield 继承component父类
 */
//rust 里：用共有行为定义一个trait, 避免将struct 或者enum称为对象，因为他们与impl块是分开的

use oop17::Draw;
use oop17::{Button,Screen};


struct SelectBox{
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox{
    fn draw(&self){
        //绘制一个选择框
    }

}

fn main() {
    let screen =Screen{
        components : vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("yes"),
                    String::from("maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width :50,
                height: 10,
                label: String::from("OK"),
            }),
            // Box::new(String::from("hello")),//the trait `oop17::Draw` is not implemented for `std::string::String`
        ],
    };
    screen.run();
}

//trait 对象执行的是动态派发
//将trait约束作用于泛型时，rust编译器会执行单态化：
//编译器会为我们用来替换泛型类型参数的每一个具体类型生成对应函数和方法的非泛型实现
//通过单态化生成的代码会执行静态派发，在编译过程中，确定调用的具体方法
//动态派发：无法在编译过程中确定你调用的是哪一种方法，编译器会产生额外的代码以便在运行时找出希望调用的方法
//使用trait对象，会执行动态派发-产生运行时开销，使得部分优化操作无法进行

//trait对象必须保证对象安全 ：只能把满足对象安全的trait转化为trait对象
//rust  采用一系列规则来判定某个对象是否安全，只需要记住两条：
//1、方法的返回类型不是Self,2、方法中不包含任何泛型类型参数


// pub trait Clone{
//     fn clone(&self)->Self;
// }

// pub struct Screen1{
//     pub components:Vec<Box<dyn Clone>>, //克隆这个trait 不可以变成对象，因为返回类型是Self
// }
// fn trait_oopsafe(){

// }


/**
 *  实现面向对象的设计模式
 * 状态模式：是一种面向对象设计模式，一个值拥有的内部状态由数个状态对象表达而成，而值的行为则随着内部状态的改变而改变
 * 使用状态模式意味着：业务需求变化时不需要修改持有状态的值的代码，或者使用这个值得代码，业务只需要更新状态对象内部的代码，以便改变其规则。或者增加一些新的状态对象
 * 
 * 缺点：某些状态耦合，重复代码
 */

use oop17::Post;
fn post(){
    let mut post  =  Post::new();
    post.add_text("I ste a salad for lunch today");
    assert_eq!("",post.content());

    post.request_review();
    assert_eq!("",post.content());

    post.approve();
    assert_eq!("I ste a salad for lunch today",post.content());
}
 
//将状态和行为编码为类型