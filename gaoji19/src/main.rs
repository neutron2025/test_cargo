
/***
 * 高级特性
 * 不安全rust
 * 高级trait
 * 高级类型
 * 高级函数和闭包
 * 宏
 * 
 */

fn main() {
    println!("Hello, world!");
    unsafe{ //只能在unsafe块，unsafe函数里调用unsafe 方法
        dangerous();
    }
}

/***
 * 匹配命名变量
 * 隐藏的第二个语言：它没有强制内存安全保证：unsafe rust  和普通的rust一样，但提供了额外的超能力
 * unsafe rust存在的原因：
 * -静态分析是保守的。使用unsafe rust 我知道自己在干什么，并承担相应的风险
 * -计算机硬件本身就是不安全的，rust需要能够进行底层系统编程
 */

//unsafe 超能力：使用unsafe关键字，来切换到unsafe rust,开启一个块，里面放着unsafe代码
// unsafe rust里面可执行的四个动作（unsafe 超能力）
// -解引用原始指针
// -调用unsafe 函数或方法
// -访问或修改可变的静态变量
// -实现unsafe trait
// 注意：unsafe并没有关闭借用检查或停用其他安全检查； 任何内存安全相关的错误，必须留在unsafe块里； 尽可能隔离unsafe代码，最好将其封装在安全的抽象里，提供安全的API

// 解引用原始指针，
// 原始指针: 
// -可变的：*mut T  
// -不可变的：*const T 意味着指针在解引用后不能直接对其赋值
// -注意：这里的* 是类型名的一部分，不是解引用符号
//与引用不同，原始指针：
// -允许通过同时具有不可变和可变指针 或 多个指向同一位置的的可变指针来忽略借用规则
// -无法保证能指向合理的内存
// -允许为null
// -不实现任何自动清理
// 放弃保证的安全，换取更好的性能/与其他语言或硬件接口的能力

fn yuanshizhizhen(){
    let mut num  = 5;
    let r1 = &num as *const i32;  //r1,r2这两个原始指针其实是指向了同一块内存，一个不可变，一个可变（可修改其值）
    let r2 = &mut num as *mut i32; //在unsafe代码块外面可以创建原始指针，但是只能在unsafe代码块里面对其进行解引用
    unsafe{
        println!("r1:{}",*r1);
        println!("r2:{}",*r2);//这里可以解引用修改r2
    }

    let address= 0x12345usize;
    let r=address as *const i32;
    unsafe{
        println!("r:{}",*r);//运行会报错，非法访问(访问地址0x12345的数据)！自己负责
    }

}
//为什么使用原始指针？：1、与C语言进行接口，2、构建借用检查器无法理解的安全抽象

//调用unsafe函数或方法：在定义前加上了unsafe关键字
//-调用前需手动满足一些条件（主要看文档），因为rust无法对这些条件进行验证，
//-需要在unsafe块里进行调用

unsafe fn dangerous(){

}

//创建unsafe代码的安全抽象
//-函数包含unsafe代码，并不意味着需要将整个函数标记为unsafe
//-将unsafe代码包裹在安全函数中是一个常见的抽象

// pub fn split_at_mut(slice:&mut[i32],mid:usize)->(&mut [i32],&mut [i32]){
//     let len = slice.len();
//     assert!(mid<=len);
//     (&mut slice[..mid],&mut slice[mid..]) //两次可变引用，报错   引用了同一块的两边不同部分，rust不够智能所以报错。放在unsafe就好了
// }
fn split_at_mut_test(){
    let mut v=vec![1,2,3,4,5,6];
    let r = &mut v[..];
    let (a,b) = r.split_at_mut(3);
    assert_eq!(a,&mut[1,2,3]);
    assert_eq!(b,&mut[4,5,6]);
}

use std::slice;
pub fn split_at_mut1(slice:&mut[i32],mid:usize)->(&mut [i32],&mut [i32]){
    let len = slice.len();
    let ptr  = slice.as_mut_ptr(); //返回指向切片的原始指针
    assert!(mid<=len);

    unsafe{
        (
            slice::from_raw_parts_mut(ptr,mid),//返回mid长度的原始指针
            slice::from_raw_parts_mut(ptr.add(mid),len-mid), //返回从mid开始，长度len-mid的原始指针
        )
    }
}

fn split_at_mut_test1(){
    let mut v=vec![1,2,3,4,5,6];
    let r = &mut v[..];
    let (a,b) = r.split_at_mut(3);
    assert_eq!(a,&mut[1,2,3]);
    assert_eq!(b,&mut[4,5,6]);

    let address = 0x12345usize;
    let r = address as *mut i32;
    let slice: &[i32] = unsafe{
        slice::from_raw_parts_mut(r, 10000) //这里不能保证一万个元素的切片是有效的，会造成崩溃
    };
}

//使用extern函数调用外部代码
//extern关键字：简化创建和使用外部函数接口（FFI）的过程
//FFI：它允许一种编程语言定义函数，并让其他编程语言能调用这些函数
//任何在extern块里申明的函数都是不安全的
extern "C"{  //C就是应用程序二进制接口（ABI）：定义函数在汇编层的调用方式，“C” ABI是最常见的ABI，它遵循C语言的ABI
    fn abs(input:i32)->i32;
}
fn extern_test(){
    unsafe{
        println!("absolute value of -3 = {}",abs(-3));
    }
}

//用其他语言调用rust函数
//可以使用extern创建接口，其他语言通过它们，可以调用rust的函数
//在fn 前添加extern关键字，并制定ABI
//还需添加#[no_mangle]注解：避免rust在编译时改变它的名称

#[no_mangle]
pub extern "C" fn call_from_c(){
    println!("just called a rust function from c");
}

//访问或修改一个可变静态变量
//rust支持全局变量，但因为全局变量，可能产生某些问题，例如数据竞争
//在rust里，全局变量叫做静态（static）变量

static HELLO_WORLD:&str ="hello world";
fn usestatic(){
    println!("name is {}",HELLO_WORLD);
}
// 静态变量与常量类似，命名：SOME_NUM_SSD（全大写，_隔开）
//必须标注类型
//静态变量只能存储 'static 生命周期的引用，无需显式标注
//访问不可变静态变量是安全的

//常量和不可变静态变量的区别
//静态变量：有固定的内存地址，使用它的值总会访问同样的数据
//常量：允许使用它们的时候，对数据进行复制
//静态变量：可以是可变的，访问和修改静态可变变量是不安全的（unsafe）

static mut COUNTER:u32=0;
fn add_to_count(inc:u32){
    unsafe{
        COUNTER+=inc;
    }
}
fn mian2(){
    add_to_count(6);
    unsafe{
        println!("COUNTER：{}",COUNTER); //不安全的变量，放在unsafe块里。这里多线程会发生数据竞争
    }
}

//实现不安全（unsafe）trait
//当某个trait中存在至少一个方法拥有编译器无法校验的不安全因素时，就称这个trait时不安全的
//生命unsafe trait:在定义前加unsafe关键字:该trait只能在unsafe代码块中实现   

unsafe trait foo{
    //许多方法
}

unsafe impl foo for i32{
    //实现方法
}

//何时使用unsafe代码
//编译器无法保证内存安全，保证unsafe代码正确并不简单。有充足理由使用unsafe代码时就可以这样做。通过显式标记unsafe，可以在出现问题时轻松的定位

/***
 * 高级trait：在trait定义中使用关联类型来指定占位类型
 * -关联类型是trait中的类型占位符，它可以用于trait的方法签名中
 * -可以定义出包含某些类型的trait，而在实现前无需知道这些类型是什么
 */
pub trait Iterator{
    type Item; //Item就是类型占位符，就是关联类型，具体在迭代的过程中就是用Item类型来替代迭代中出现的值
    fn next(&mut self)->Option<Self::Item>;
}


/***
 * 关联类型与泛型的区别
 * 泛   型：     每次实现trait时标注类型，      可以为一个类型多次实现某个trait（不同的泛型参数）
 * 关联类型：    无需标注类型，（但在里面指明）     无法为单个类型多次实现某个trait
 */

pub trait Iterator2<T>{
    fn next(&mut self)->Option<T>;
}
struct Counter{}
impl Iterator for Counter{
    type Item = u32;    //无需标注类型，（但在里面指明） 只能实现一次
    fn next(&mut self)->Option<Self::Item>{
        None
    }
}
impl Iterator2<String> for Counter{//使用不同的泛型参数，就可以多次实现这个trait
    fn next(&mut self)->Option<String>{
        None
    }
}
impl Iterator2<u32> for Counter{//使用不同的泛型参数，就可以多次实现这个trait
    fn next(&mut self)->Option<u32>{
        None
    }
}

/***
 * 默认泛型参数和运算符重载
 * 可以在使用泛型参数时为泛型指定一个默认的具体类型
 * 语法： <PlaceholderType =ConcreteType>
 * 这种技术常用语运算符重载
 * rust不允许创建自己的运算符及重载任意的运算符
 * 但可以通过实现std::ops中列出的那些trait来重载一部分相应的运算符
 */

 use std::ops::Add; //pub trait Add<Rhs = Self> { ... }  Rhs默认为Self,就是Pointt
#[derive(Debug,PartialEq)]
struct Pointt{
    x:i32,
    y:i32,
}
impl Add for  Pointt{  //默认泛型参数
    type Output = Pointt;
    fn add(self,other:Pointt)->Pointt{
        Pointt{
            x:self.x + other.x,
            y:self.y + other.y,
        }
    }
}
fn testeq(){
    assert_eq!(Pointt{x:1,y:0}+Pointt{x:2,y:3}, Pointt{x:3,y:3});
}

struct Millimeters(u32);
struct Meters(u32);
impl Add<Meters> for Millimeters{ //具体指明了泛型参数
    type Output = Millimeters;

    fn add(self,other:Meters)->Millimeters{
        Millimeters(self.0+(other.0*1000))
    }
}

//默认参数类型的主要应用场景：扩展一个类型而不破坏现有代码，允许在大部分用户都不需要的特定场景下进行自定义

//完全限定语法：如何调用同名方法
trait Pilot{
    fn fly(&self);
}
trait Wizard{
    fn fly(&self);
}
struct Human;
impl  Pilot for Human{
    fn fly(&self){
        println!("pilot fly");
    }
}
impl Wizard for Human{
     fn fly(&self){
        println!("Wizard fly");
    }
}
impl   Human{
    fn fly(&self){
        println!("Human fly");
    }
}

fn xiangtong(){
    let person = Human;
    person.fly();
    Wizard::fly(&person);
    Pilot::fly(&person); //调用同名方法 通过传入的person识别
}
//#################################
//完全限定语法:<Type as Trait>::function(接受者，函数参数)
//-可以在任何调用函数或方法的地方使用
//-允许忽略那些从其他上下文推导出来的部分
//-当rust无法区分你期望调用哪个具体实现的时候，才需使用这种语法
trait Animal{
    fn baby_name()->String;
}
struct Dog;
impl Dog{
    fn baby_name()->String{
        String::from("spot")
    }
}
impl Animal for Dog{
    fn baby_name()->String{
        String::from("puppy")
    }
}
fn main2(){
    println!("{}",Dog::baby_name()); //baby_name是Dog本身的关联方法
    // println!("{}",Animal::baby_name());//dog 没有传进去，所以无法识别出调用哪个实现的baby_name方法
    println!("{}",<Dog as Animal>::baby_name()); //这时候就可以使用完全限定语法

}

//使用supertrait 来要求trait附带其他trait的功能
//需要在一个trait中使用其他trait的功能
//-需要被依赖的trait也被实现
//-那个被间接依赖的trait就是当前trait的supertrait

use std::fmt;
trait OutlinePrint: fmt::Display{  //使用到to_string，冒号后面是依赖的trait
    fn outline_print(&self){
        let output = self.to_string();
        let len  = output.len();
        println!("{}","*".repeat(len+4));
        println!("*{}*"," ".repeat(len+2));
        println!("* {} *",output);
        println!("*{}*"," ".repeat(len+2));
        println!("{}","*".repeat(len+4));
    }
}
struct Point2{
    x:i32,
    y:i32,
}

impl OutlinePrint for Point2{} //为Point2实现OutlinePrint trait, 也要为Point2实现fmt::Display trait
impl fmt::Display for Point2{
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
        write!(f,"({},{})",self.x,self.y)
    }
}

/***
 * 使用newtype模式在外部类型上实现外部trait
 * 孤儿规则：只有当trait或类型定义在本地包时，才能为该类型实现这个trait
 * 可以通过newtype模式来绕过这一规则
 * -利用tuple struct(元组结构体)创建一个新的类型
 */

 ////我们想为vector实现Display这个trait，而他俩都在外部包中，所以无法直接实现
 /// 怎么做呢? 我们把vector 包裹在wrapper这个类型里面，这个wrapper是本地创建的tuple struct，
 /// 然后我们就可以为wrapper 实现Display这个 trait。我们想把里面的vector 调出来，就用self.0, 然后做具体的实现就可以了
 struct Wrapper(Vec<String>);//我们想为vector实现Display这个trait，而他俩都在外部包中，所以无法直接实现
 impl fmt::Display for Wrapper{
     fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
         write!(f,"[{}]",self.0.join(", "))
     }
 }

 fn test_wrapper(){
     let w=Wrapper(vec![String::from("hello"),String::from("world")]);
     println!("w={}",w);
 }

 /// 类型的高级应用
 /// 使用newtype模式实现类型的安全和抽象
 /// new type模式可以：用来静态的保证各种值之间不会混淆并表明值的单位，
 /// -为类型的某些细节提供抽象能力
 /// 通过轻量级的封装来隐藏内部实现细节
 /// 
 /// 使用类型别名创建类型同义词
 /// rust提供了类型别名功能：为现有类型产生另外的名称（同义词），并不是一个独立的类型，使用type关键字
 /// 主要用途：减少代码字符重复
 
 type Kilimeters = i32;  //定义了i32的类型别名 Kilimeters
 fn testtype(){
     let x:i32 =5;
     let y:Kilimeters =5;
     println!("{}+{}={}",x,y,x+y);
 }


//  ######################### 
type Thunk =Box<dyn Fn() + Send + 'static>;  //用Thunk代替类型别名，代码简洁很多

 fn takes_long_type(f:Thunk){

 }
 fn returns_long_type()->Thunk{
     Box::new(|| println!("hi"))
 }

 fn mian3(){

    let f:Thunk = Box::new(|| println!("hi"));
 }
 //  ######################### 

 use std::io::Error;
//  type Result<T> = Result<T,std::io::Error>;   //这句话已经在std::io 中申明了
type Result<T> = std::io::Result<T>;
pub trait Write{
    fn write(&mut self,buf:&[u8])->Result<usize>;
    fn flush(&mut self)->Result<()>;

    fn write_all(&mut self,buf:&[u8])->Result<()>;
    fn write_fmt(&mut self,fmt:fmt::Arguments)->Result<()>;
}

///Never类型
/// 有一个名为！的特殊类型：
/// 它没有任何值，行话称为空类型（empty type）
/// 我们倾向于叫它never 类型，因为它不在返回的函数中充当返回类型
/// 不返回值的函数也被称为发散函数
//  fn bar()->!{   //表明这个函数永远不会有返回值，expected `!`, found `()`   ()叫做单元类型

//  }
 //那么它有什么用那？ never类型的表达式可以被强制的转化为任意其他的类型
 fn never(){
     let guess = "guess";
     loop{
         let guess:u32 = match guess.trim().parse(){
             Ok(num)=>num,
             Err(_)=>continue,//这里的continue代表never,被强制转化为u32类型
         };
     }

    //  match self{
    //      Some(val)=>val,
    //      None =>panic!("终止程序"),//panic!返回类型就是never类型 ，也就是 ！，中断程序，不会返回值
    //  }

    loop{ //永远不会结束，返回类型也是never类型，当然可以break终止循环
        println!("never")
    }

 }

 ///动态大小和 sized trait
 /// rust 在编译时确定为一个特定类型的值分配多少空间
 /// 动态大小的类型概念：编写代码时使用，只有在运行时才能确定大小的值
 /// str时动态大小的类型，(注意不是&str):只有运行时才能确定字符串的长度
 /// 下列代码无法正常工作：他俩同一个类型，需要的空间不一样 （同一类型所有值必须使用等量的内存）
//  let s1:str = "hello";
//  let s2:str = "how's it going";
///使用&str来解决：存的是str的地址，和长度 。大小是固定的
/// rust使用动态大小类型的通用方式：附带一些额外的元素数据来存储动态信息的大小
/// -使用动态大小类型时总会把它的值放在某种指针后面
/// 另外一种动态大小的类型：trait
/// 每个trait都是一个动态大小的类型，可以通过名称对其进行引用
/// 为了将trait用作trait对象，必须将它放置在某种指针之后，例如：&dny Trairt 或Box<dny trait>(Rc<dny Trait>)之后
/// 
/// sized trait:为了处理动态大小的类型，rust提供了一个sized trait来确定一个类型的大小在编译时是否已知
/// -编译时可计算出大小的类型会自动实现这一trait
/// -rust还会为每一个泛型函数隐式的添加sized约束
/// 
// fn generic<T>(t:T){   }  //实际上会被隐式的转化为下面这个
// fn generic<T:Sized>(t:T){   }
///默认情况下，泛型函数只能被用于编译时已经知道大小的类型，可以通过特殊语法解除这一限制  ?Sized trait约束
fn generic<T:?Sized>(t:&T){   }//表达了一种不确定性，表示这个T 可能是sized，也可能不是sized.  ?只能被用在Sized trait，不能用在其他trait
//这时候使用&T，因为大小可能是不确定的 


///高级函数和闭包
/// 函数指针：可以将函数传递给其他函数，函数在传递过程中会被强制转换成fn 类型。fn类型就是函数指针
fn add_one(x:i32)->i32{
    x+1 
}
fn do_twice(f: fn(i32)->i32,arg:i32)->i32{  //传入函数及其参数
    f(arg)+f(arg)
}
fn main4(){
    let answer = do_twice(add_one, 5);
    println!("the answer is :{}",answer);
}

///函数指针与闭包的不同
/// fn 是一个类型，不是一个trait
/// -可以直接指定fn为参数类型，不用声明一个以fn trait为约束的泛型参数
/// 函数指针实现了全部三种闭包trait （Fn,FnMut,FnOnce）
/// -总是可以把函数指针用作参数传递给一个接受闭包的函数，所以倾向于搭配闭包trait的泛型来编写函数：可以同时接收闭包和普通函数
/// 某些情形，只想接收Fn,而不接受闭包：例如 与外部不支持闭包的代码交互：C函数
/// 
fn main5(){
    let list_of_numbers = vec![1,2,3]; //元素类型为i32
    let list_of_strings:Vec<String> = list_of_numbers  //但是我们需要返回string元素类型
    .iter()
    .map(|i| i.to_string())  //传闭包
    .collect();

    let list_of_numbers = vec![1,2,3];
    let list_of_strings:Vec<String> = list_of_numbers
    .iter()
    .map(ToString::to_string)  //传函数
    .collect();
}

fn main6(){
    enum Status{
        Value(u32),
        Stop,
    }

    let v = Status::Value(3); //可以把这种构造器也作为实现了闭包trait的函数指针来进行使用
    let list_of_status: Vec<String> =(0u32..20)
    .map(Status::Value)//直接传入构造器
    .collect();
}

///返回闭包
/// 闭包使用trait进行表达，无法在函数中直接返回一个闭包，可以将一个实现了该trait的具体类型作为返回值
/// 
// fn return_closure()->Fn(i32)->i32{   //这个返回闭包在编译时没有一个已知的大小，rust无法推断出需要多少空间来存储这个返回的闭包
//     |x| x+1
// }

fn return_closure()->Box<dyn Fn(i32)->i32>{ //这样返回类型就有固定的大小了，编译就不会出问题了。相当于放在某种指针后边了，返回类型就有固定的大小了
    Box::new(|x| x+1)
}

///宏：在rust里指的是一组相关特性的集合称谓
/// -使用macro_rules! 构建的声明 宏
/// -3种过程宏 
/// 1、自定义#[derive]宏，用于struct或enum,可以为其指定随derive属性添加的代码
/// 2、类似属性的宏，在任何条目上添加自定义属性
/// 3、类似函数的宏，看起来像函数调用，对其指定为参数的token进行操作
/// 
/// 函数和宏的差别：
/// 1、本质上，宏时用来编写可以生成其他代码的代码（元编程）
/// 2、函数在定义签名时，必须声明参数的个数和类型，宏可处理可变的参数
/// 3、编译器会在解释代码前展开宏
/// 4、宏的定义比函数复杂的多，难以理解，阅读，维护
/// 5、在某个文件调用宏时，必须提前定义宏或将宏引入当前作用域
/// 6、函数可以在任何位置定义并在任何位置使用
/// 
/// macro_rules!声明宏（弃用）：rust中最常见的宏形式，类似于match的模式匹配
/// 

fn macrohong(){
    let v: Vec<u32>= vec![1,2,3];

    #[macro_export]  //缺少了这个标注，就不能被引入作用域
    macro_rules! vec {     //match 是匹配值，这里是匹配rust的代码结构
        ($($x:expr),*) => { //$x:expr 匹配任何rust表达式命名为$x ,* 表示匹配多个
            {
                let mut temp_vec=   Vec::new();
                $(
                    temp_vec.push($x);  //每次匹配都会生成对应的代码，三个元素生成三次
                )*
                temp_vec
            }
            
        };
    }

}

///基于属性来生成代码的过程宏
/// 这种形式更像函数一些（某些形式的过程）
/// -接受并操作输入的rust代码
/// -生成另外一些rust代码作为结果
/// 
/// 三种过程宏：1、自定义派生，2、属性宏，3、函数宏
/// 创建过程宏时：宏定义必须单独放在它们自己的包中，并使用特殊的包类型
/// 
// use proc_macro::TokenStream;

// #[some_attribute] //是一个指定过程宏类型的占位符
// pub fn some_name(input: TokenStream) -> TokenStream{ //定义过程宏函数 TokenStream类型在proc_macro包中定义

// }

///自定义derive宏（派生宏）
/// 需求：创建一个hello_macro 包，定义一个拥有关联函数hello_macro 的HelloMacro trait
/// 我们提供一个能自动实现trait的过程宏
/// 在他们的类型上标注#[derive(HelloMacro)],进而得到hello_macro的默认实现
/// 详见gaojimacro19

///类似属性的宏
/// 属性宏与自定义derive宏类似
/// -允许创建新的属性
/// -但不是为derive属性生成代码
///属性宏更加灵活
/// -derive只能用于struct 和enum
/// -属性宏可以用于任意条目，例如函数

// #[route(GET,"/")]
// fn index(){  }

// #[proc_macro_attribute]  //属性宏和自定义的派生宏的工作方式几乎一样，都需要建立一个proc_macro的包并提供相应代码的函数
// pub fn route(attr:TokenStream,item:TokenStream)->TokenStream{ }

///类似函数的宏
/// -函数宏定义类似于函数调用的宏，但比普通函数更加灵活
/// -函数宏可以接受TokenStream作为参数
/// -与另外两种过程宏一样，在定义中使用rust代码来操作TokenStream
/// 
// let sql  = sql!(select * from posts where id =1);  和派生宏类似
// #[proc_macro]
// pub fn sql(input:TokenStream)->TokenStream{}