/**
 * 智能指针：拥有它所指向的数据 通常用struct实现，并且实现了Deref和Drop这两个trait
 * 引用：只是借用数据
 * String 和Vec<T> 就是智能指针，拥有一片内存区域，允许用户对其操作
 * 
 * Deref trait 允许智能指针struct的实例像引用一样使用
 * Drop trait 允许自定义当智能指针离开作用域时运行的代码
 * 
 * 标准库中常见的智能指针：
 * -Box<T>:在heap内存上分配值
 * -Rc<T> :启用多重所有权的引用类型
 * -Ref<T> 和 RefMut<T>: 通过RefCell访问：在运行时而不是编译时强制借用规则的类型
 * 
 * 此外：
 * 内部可变模式：不可变类型暴露出可修改其内部值的API
 * 引用循环：  它们如何泄露内存，以及如何防止其发生
 * 
 */
//Box<T> 是一种最简单的智能指针 stack 上是指向heap数据的指针
//Box<T>:
//的使用场景1、编译时无法确定某类型的大小，但使用该类型时，上下文却需要知道他的确切大小
//的使用场景2、当你有大量的数据，想移交所有权，但需要确保在操作时数据不会被复制
//的使用场景3、使用某个值时，你只关心它是否实现了特定的trait，而不关心它的具体类型

use crate::List::{Cons,Nil};
use std::ops::Deref;


 fn main() {
     let b = Box::new(5);
    println!("Hello, world! {}",b);
    let list =  Cons(1,
        Box::new(Cons(2,
           Box::new(Cons(3,
             Box::new(Nil))))));

    just::text_list2();
    some::testfn();
    // xunhuantest();
    sandv::sandv_test();



}
//在编译时，rust需要知道一个类型所占的空间大小，而递归的类型大小无法在编译时确定，可用Box 解决
//Const List 里每个成员有两个元素组成 1、当前项的值，2、下一个元素 最后一个成员只包含一个Nil值，没有下一个元素

enum List{
    Cons(i32,Box<List>),//Box 指针大小确定，间接指向数据
    Nil,
}


//Deref trait 实现Deref trait 使我们可以 自定义解引用运算符 * 的行为
//通过实现 Deref , 智能指针可像常规指针一样来处理

fn pointer(){
    let x= 5;
    let y = &x;
    let z = Box::new(x); //Box 可以像引用一样处理
    let v = MyBox::new(x); //MyBox 可以像引用一样处理  因为实现了Deref Trait

    assert_eq!(5,x);
    assert_eq!(5,*y);
    assert_eq!(5,*z);
    assert_eq!(5,*v); //标准库中的 Deref trait要求我们实现一个deref方法：该方法借用self，返回一个指向内部数据的引用
    //*v 相当于 *(v.deref())
}

struct MyBox<T>(T);  //被定义成一个拥有元素的tuple struct
impl <T> MyBox<T>{
    fn new(x:T)->MyBox<T>{
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T>{
    type Target = T;
    fn deref(&self)->&T{
        &self.0
    }
}


//函数和方法的隐式解引用转化：隐式解引用转化是函数和方法提供的一种便捷特性
//假设 T 实现了Deref trait ：隐式解引用转化（deref coercion）可以把T的引用转化为T经过Deref操作后生成的引用
//当把某类型的引用传递给函数或方法时，但他的类型与定义的参数类型不匹配：
//-这时候隐式解引用转化（deref coercion）就会自动发生
//-编译器会对deref进行一系列调用，来把他转为所需的参数类型（此操作在编译时完成）

fn hello(name:&str){
    println!("hello {}",name);
}
fn use_hello(){
    let m = MyBox::new(String::from("rust"));
    hello(&m);
    hello("rust");
    //&m  &MyBox<String>  由于MyBox实现了deref trait  所以rust调用deref 将&MyBox<String> 转化为&String
    //deref &String    由于String（返回切片）实现了deref trait  所以rust调用deref 将&String 转化为&str
    //deref &str

    hello(&(*m)[..]); //如果没有实现 隐式解引用转化（deref coercion）是这样的
}


// !解引用与可变性：可使用DerefMut trait 重载可变引用的*运算符
// ! 在类型和trait在下列三种情况发生时，rust 会执行隐式解引用转化（deref coercion）
// ! 1、当T：Deref<Target =U>,允许&T转换为&U
// ! 2、当T：DerefMut<Target =U>,允许&mut T 转换为&mut U
// ! 3、当T：Deref<Target =U>,允许&mut T转换为&U
// ! 

//Drop trait 允许自定义当智能指针离开作用域时的代码
//-例如，文件，网络资源释放等
//-只要求你实现drop方法  参数：对self 的可变引用
//-在(prelude)预导入模块里 

struct CustomSmartPointer{
    data:String,
}
impl Drop for CustomSmartPointer{
    fn drop(&mut self){
        println!("Droping CustomSmartPointer with data {}",self.data);
    }
}

fn test_CustomSmartPointer(){
    let c = CustomSmartPointer{data: String::from("my stuff")};//3执行
    drop(c); //这里提前释放，最先执行c
    let d = CustomSmartPointer{data: String::from("other stuff")};//这两个方法在离开作用域时会调用drop方法  2执行
    println!("CustomSmartPointers created") //压栈  1执行

}

//使用std::mem::drop 来提前drop值
//-因为很难直接禁用自动的drop功能，也没必要 -Drop trait 的目的就是进行自动的释放处理逻辑  | Rust不允许手动调用Drop trait的drop方法
//但是可以使用std::mem::drop 来提前drop值. 在prelude模块中，可以不用导入时

//Rc<T> 引用计数智能指针 : 有时，一个值会有多个所有者 例如 图
//为了支持多重所有权  Rc<T>   1、(reference counting) 引用计数 2、追踪所有值的引用 3、0个引用时，该值可以被清理掉
//使用场景：需要在heap上分配数据，这些数据被程序的多个部分读取(只读)，但在编译时无法确定哪个部分最后使用完这些数据
//Rc<T> 只能用于单线程场景

//Rc<T>不在prelude模块
//Rc::clone(&a) 函数：增加引用计数
//Rc::strong_count(&a) :获得强引用计数  与之相对的 Rc::weak_count(&a) 是弱引用计数
//例子：两个list 共享另一个list所有权

// mod jus{
//     enum List2{
//         Cons(i32,Box<List2>),
//         Nil,
//     }
//     use crate::jus::List2::{Cons,Nil};
//     fn text_list2(){
//         let a = Cons(5,
//             Box::new(Cons(10,
//             Box::new(Nil))));
//         let b = Cons(3, Box::new(a));
//         let c = Cons(4, Box::new(a));  //这里会报错，所有权已转移
//     }
// }
pub mod just{
    enum List{
        Cons(i32,Rc<List>), //使用引用计数 而不是智能指针
        Nil,
    }
    use crate::just::List::{Cons,Nil};
    use std::rc::Rc;
    pub fn text_list2(){
        let a = Rc::new(Cons(5, //引用计数器变为1，因为被引用了一次，a获得了其所有权
            Rc::new(Cons(10,
                Rc::new(Nil)))));
                println!("{}",Rc::strong_count(&a)); //打印的时候获得引用计数

                //a.clone(); 会深度克隆（类型克隆），   Rc::clone(&a)只会增加引用计数速度比较快（RC克隆）
            let b = Cons(3, Rc::clone(&a));  //引用计数器变为2  没有所有权，就克隆以便共享a的数据
            println!("{}",Rc::strong_count(&a));

            {
                let c = Cons(4, Rc::clone(&a));   //引用计数器变为3
                println!("{}",Rc::strong_count(&a));
            } //c 在这里离开作用域，会减少一次计数（离开作用域自动清理，rc<T> 实现了drop Trait）  a 的引用次数减少一次 变为2

            println!("{}",Rc::strong_count(&a)); //引用计数器变为2

    }
}

//Rc<T> 通过不可变引用，使你可以在程序的不同部分之间共享只读数据


//RefCell 和内部可变性
//内部可变性是rust的设计模式之一：它允许你在只持有不可变引用的前提下对数据进行修改
//数据结构中使用了unsafe代码来绕过rust正常的可变性和借用规则
//与Rc<T>不同，RefCell<T>类型代表了其持有数据的唯一所有权
//回忆一下借用规则：在任何给定的时间里，你要么只能拥有一个可变引用。要么只能拥有任意数量的不可变引用
//RefCell<T> 与Box<T>的区别
//Box<T>在编译阶段强制代码遵守借用规则，否则出错 而 RefCell<T> 只会在运行时检查借用规则，否则触发panic
//RefCell<T> 只能用于单线程场景

// ! 选择Box<T>,Rc<T>,RefCell<T>的依据
// !                    Box<T>,         Rc<T>,      RefCell<T>
// ! 同一数据的所有者     一个             多个          一个
// ! 可变性，借用检查  可变，不可变借用    不可变借用    可变，不可变借用
// ! 检查时间             编译              编译         运行
// ! 
// ! 其中：即使RefCell<T>本身不可变，但仍能修改其中存储的值
// !  
//内部可变性: 可变地借用一个不可变的值（对外部不可变，对方法内部可变）

mod cell{
    pub trait Messager{
        fn send(&self,msg:&str);
    }

    pub struct LimitTracker<'a,T:'a+Messager>{
        messager: &'a T,
        value: usize,
        max:usize,
    }

    impl<'a,T>LimitTracker<'a,T>
    where
        T:Messager,
        {
            pub fn new(messager:&T,max:usize)->LimitTracker<T>{
                LimitTracker{
                    messager,
                    value:0,
                    max,
                }
            }

            pub fn set_value(&mut self,value:usize){
                self.value = value;
                let percentage_of_max = self.value as f64 /self.max as f64;
                if percentage_of_max >= 1.0{
                    self.messager.send("Error too big");
                }else if percentage_of_max >= 0.9{
                    self.messager.send("Error too big 0.9");
                }else if percentage_of_max >= 0.75{
                    self.messager.send("Error too big 0.75");

            }
        }
    }

}

#[cfg(test)]
mod tests{
    use super::cell::*;
    use std::cell::RefCell;  //修改
    struct MockMessager{
        send_messages:RefCell<Vec<String>>, //修改
    }
    impl MockMessager{
        fn new()->MockMessager{
            MockMessager{
                send_messages: RefCell::new(vec![]),//修改
            }
        }
    }
    impl Messager for MockMessager{
        // fn send(&mut self,message: &str){  //这里报错，因为定义的是不可变引用，所以上面的修改处使用了Refcell
        //     self.send_messages.push(String::from(message)); //而在这里修改了里面的值，所以要可变引用
        // }
        fn send(&self,message: &str){  //修改
            self.send_messages.borrow_mut().push(String::from(message)); //修改 添加borrow_mut()可以获得内部值的可变引用
        }
    }
    #[test]
    fn it_send_an_over_75_percentage_warning_message() {
        let mock_message = MockMessager::new();
        let mut limit_tracker = LimitTracker::new(&mock_message, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_message.send_messages.borrow().len(),1);//修改 添加borrow() 获得内部值的不可变引用
    }
}

//使用RefCell<T> 在运行时记录借用信息
//-两个方法（安全接口）：
//-borrow 方法：返回智能指针Ref<T>,它实现了Deref
//-borrow_mut 方法：返回智能指针RefMut<T>,它实现了Deref

//使用RefCell<T> 在运行时记录借用信息
//RefCell<T>会记录当前存在多少个活跃的Ref<T>和RefMut<T>智能指针：
//-每次调用borrow,不可变借用计数加一，任何一个Ref<T>的值离开作用域被释放时，不可变借用计数减一
//-每次调用borrow_mut,可变借用计数加一，任何一个RefMut<T>的值离开作用域被释放时，可变借用计数减一

//以此技术来维护借用检查规则：任何一个给定时间里，只允许拥有多个不可变借用或一个可变借用

//Rc<T> 和RefCell<T>结合使用来实现一个拥有多重所有权的可变数据

pub mod some{
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::some::List::{Cons,Nil};

    #[derive(Debug)]
    enum List{
        Cons(Rc<RefCell<i32>>,Rc<List>),
        Nil,

    }
    pub fn testfn(){
        let value = Rc::new(RefCell::new(5));
        let a = Rc::new(Cons(Rc::clone(&value),Rc::new(Nil)));
        let b = Cons(Rc::new(RefCell::new(6)),Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(10)),Rc::clone(&a));

        *value.borrow_mut() += 10; //自动解引用，把rc<t> 解引用为refcell<t>

        println!("a after = {:?}",a);
        println!("b after = {:?}",b);
        println!("c after = {:?}",c);

    }
}

//其他可实现内部可变性的类型：Cell<T> 通过复制来访问数据    Mutex<T> 用于实现跨线程情形下的内部可变性模式

//循环引用可导致内存泄漏：使用Rc<T> 和RefCell<T> 就可能创造出循环引用，从而发生内存泄漏
//-每个项的引用数量不会变成0，值也不会被处理掉

pub mod xunhaun{

    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::xunhaun::List::{Cons,Nil};

    #[derive(Debug)]
    pub enum List{
        Cons(i32,RefCell<Rc<List>>),
        Nil,
    }

    impl List{
        pub fn tail(&self)->Option<&RefCell<Rc<List>>>{ //此方法方便访问第二个元素
            match self{
                Cons(_,item) =>Some(item),
                Nil=>None,
            }
        }
    }

}


fn xunhuantest(){

    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::xunhaun::List::{Cons,Nil};


    let a = Rc::new(Cons(5,RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}",Rc::strong_count(&a));
    println!("a next item ={:?}",a.tail()); //a 第二个元素是Nil

    let b = Rc::new(Cons(10,RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}",Rc::strong_count(&a)); // a 的强引用计数变成2
    println!("b initial rc count = {}",Rc::strong_count(&b)); //b的引用计数为1
    println!("b next item ={:?}",b.tail());  //b 的第二个元素就是a

    if let Some(link) = a.tail(){   //这里将a 的第二个元素取出来指向 b
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}",Rc::strong_count(&b)); //计数变成2
    println!("a rc count after changing a = {}",Rc::strong_count(&a)); //计数变成2
    // a->5,b       b->10,a   这样造成循环引用

    println!("a next item :{:?}",a.tail()); //循环引用 内存泄漏

}

//防止循环引用 把Rc<T> 换成Weak<T> 
//-Rc:clone 为Rc<T> 实例的strong_count 加 1，Rc<T>的实例只有在strong_count为0 的时候才会被清理

//Rc<T> 实例通过调用Rc::downgrade 方法可创建值的 Weak Reference(弱引用)
//-返回类型是 Weak<T>(智能指针
//-调用Rc::downgrade会为weak_count 加 1
//Rc<T> 使用weak_count来追踪存在多少Weak<T>
//weak_count不为0并不影响Rc<T>实例的清理

//强弱引用区别
//strong reference(强引用) 是关于如何分享Rc<T>实例的所有权
//Weak Reference(弱引用) 并不表达上述意思
//当使用 Weak Reference 并不会创建循环引用：当strong reference 数量为0 的时候，Weak Reference会自动断开
//在指向Weak<T>前，需保证它指向的值仍然存在：在Weak<T>实例上调用 upgrade方法，返回Option<Rc<T>>


pub mod sandv{
    use std::rc::{Rc,Weak};
    use std::cell::RefCell;

    #[derive(Debug)]
    struct Node{
        value: i32,
        parent:RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }

    pub fn sandv_test(){
        let leaf =Rc::new(Node{
            value:3,
            parent:RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!("leaf parent ={:?}",leaf.parent.borrow().upgrade());//这里获得不可变引用，然后将weak<T> 转化为Rc<T>


        println!("leaf strong  = {},weak ={}", Rc::strong_count(&leaf),Rc::weak_count(&leaf));// strong 1, weak 0

        {
            let branch = Rc::new(Node{
                value:5,
                parent:RefCell::new(Weak::new()),
                children:RefCell::new(vec![Rc::clone(&leaf)]),  // strong 2, weak 0
            });
    
            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);//这里获得可变引用，然后将Rc<Node> 变成weak<Node> 并存在parent里面 |leaf 做了一个关联，所以有弱引用  // strong 2, weak 1
    
            println!("leaf parent ={:?}",leaf.parent.borrow().upgrade());   // strong 2, weak 0   父节点为None

            println!("branch strong  = {},weak ={}", Rc::strong_count(&branch),Rc::weak_count(&branch));// strong 1, weak 1
            println!("leaf strong  = {},weak ={}", Rc::strong_count(&leaf),Rc::weak_count(&leaf));// strong 2（创建的时候，branch引用的时候）, weak 1  
        }
        
        println!("leaf parent  = {:?}", leaf.parent.borrow().upgrade());  //leaf parent  = None  走出作用域
        println!("leaf strong  = {},weak ={}", Rc::strong_count(&leaf),Rc::weak_count(&leaf));// strong 1, weak 0

    }
}
