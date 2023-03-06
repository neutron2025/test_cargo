use std::thread;
use std::time::Duration;
use std::iter::Map;

/**
 * 闭包：可以捕获其所在环境的匿名函数
 * 是匿名函数，保存为变量，作为参数 可以在一个地方创建，在另一个上下文中调用，可从其定义的作用域捕获值
 * 
 * 闭包的类型推断：不要求标注参数和返回值的类型，因为在狭小的上下文中工作
 * 也可以手动添加标注类型如|num：i32|->i32{}
 * |num| num+1;  只有一个表达式，花括号也可以省略
 * 
 * 函数和闭包的定义语法比较：
 * fn add_one(x:u32)->u32{ x+1 }
 * let add_one =|x:u32|->u32{x+1};
 * let add_one = |x| {x+1};
 * let add_one = |x| x+1;
 * 
 * 
 * 使用泛型参数和Fn trait 来存储闭包
 * 
 * /**
 * 让结构体持有闭包及其返回结果，至少实现了一下Fn trait之一  Fn ,FnMut, FnOnce
 */

 */


fn main() {
    println!("Hello, world!");
    env_test();
    env_test_move();
}

struct Cacher<T>
where
    T:Fn(u32)->u32,
    {
        calculation:T,
        value:Option<u32>,
    }

impl<T> Cacher<T>
where
    T:Fn(u32)->u32,
{
    fn new(calculation:T)->Cacher<T>{
        Cacher{
            calculation,
            value:None,
        
        }
    }

    fn value(&mut self,arg: u32)->u32{
        match self.value{
            Some(v)=>v,
            None=>{
                let v= (self.calculation)(arg); //self.calculation这是闭包，后面传的是参数
                self.value=Some(v);
                v
            }
        }
    }


}
fn generate_workout(intensity:u32,random_number:u32){
    let expensive_closure = |num|{  //这里赋值定义，并不调用
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity<25{
        println!("today do {} pushups",expensive_closure(intensity));//闭包括号里赋值才调用
        println!("next,do {} situps",expensive_closure(intensity)); 
 

    }else{
        if random_number ==3{
            println!("take a break;");
        }else{
            println!("run for {} minutes",expensive_closure(20));
        }
    }
}

//让结构体持有闭包及其返回结果，实现了一下Fn trait
fn generate_workout2(intensity:u32,random_number:u32){
    let mut expensive_closure = Cacher::new( |num|{  //这里赋值定义，并不调用
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity<25{
        println!("today do {} pushups",expensive_closure.value(intensity));//闭包括号里赋值才调用
        println!("next,do {} situps",expensive_closure.value(intensity)); 
 

    }else{
        if random_number ==3{
            println!("take a break;");
        }else{
            println!("run for {} minutes",expensive_closure.value(20));
        }
    }
}
//使用Cacher缓存器实现的限制，1闭包不同的参数返回同样的值 2，只能接受一个u32类型的参数和u32类型的返回值
#[test]
fn test_call_different() {
    let mut c = Cacher::new(|a| a);
    let v1=c.value(1);
    let v2=c.value(2);
    assert_eq!(v2,2);  //test test_call_different ... FAILED  因为闭包不同的参数返回同样的值 left: `1`,  right: `2`

}

// 1、可以使用hashmap 代替单个值 key做参数，value做闭包执行结果
// 2、可以引入两个或者多个泛型参数就可以了

fn env_test(){
    let x= 4;
    let equal_to_x = |z| z ==x;  //闭包可以访问定义它作用域内的变量。而普通函数则不行
    let y =4;
    assert!(equal_to_x(y));
    println!("{:?}",(equal_to_x(y)));

}

//闭包从坐在环境中捕获变量的方式 1、取得所有权 FnOnce  2、可变借用 FnMut  3、不可变借用 Fn

//创建闭包时    通过对闭包环境值的使用，rust推断出具体使用了哪个trait
//-所有闭包都实现了FnOnce
//-没有移动捕获变量的实现了FnMuut   (实现了FnMut的都实现了FnOnce)
//-无需可变访问捕获变量的闭包实现了Fn    (实现了FnMut)

//move 关键字：在参数列表前使用move关键字，可以强制闭包取得它所使用环境的所有权
//-当将闭包传递给新线程以移动数据使其归新线程所有时，此技术最为有用
fn env_test_move(){
    let x= vec![1,2,3];
    let equal_to_x = move |z| z ==x;  //闭包可以访问定义它作用域内的变量。而普通函数则不行
    let y =vec![1,2,3];
    //println!("{:?}",x);  // value moved into closure here  x 的所有权被移动到闭包里面了
    assert!(equal_to_x(y));
    //println!("{:?}",(equal_to_x(y)));

}
//当指定Fn trait bound 之一时。首先用Fn,基于闭包里面的情况如果需要FnMut或FnOnce，编译器会再告诉你

//迭代器
//所有迭代器都实现了 iterator trait  
//iterator trait   只需要实现一个方法 next()
fn iter_test(){
    let x= vec![1,2,3];
    let v1 = x.iter();  //v1 取得了所有权，并在内部把他变成可变的了
    for val in v1{
        println!("{}",val);
    }
}
#[test]
fn iterator_test() {
    let v1=vec![1,2,3];
    let mut iter_v = v1.iter();
    assert_eq!(iter_v.next(),Some(&1));
    assert_eq!(iter_v.next(),Some(&2));
    assert_eq!(iter_v.next(),Some(&3));

}

//几个迭代方法
//-iter方法：在不可变引用上创建迭代器
//-into-iter 方法：创建的迭代器会获取所有权
//-iter-mut 方法：迭代可变引用

//在标准库中，iterator trait 有一些带默认实现的方法
//其中有一些方法会调用next 方法 ，实现iterator trait 就必须实现next方法
//把调用next的方法叫做消耗型适配器，因为调用next 会把迭代器里的元素一个一个的吃掉 ，最终耗尽迭代器 例如sum方法
//定义在iterator trait上的另外一些方法叫做 迭代器适配器（可以转换迭代器的类型），可以链式调用多个迭代器适配器来执行复杂的操作 例如map:接受一个闭包，作用于每一个元素，产生一个新迭代器
//collect 方法：消耗型适配器，把结果收集到一个集合类型中
#[test]
fn iter_sum() {
    let v1=vec![1,2,3];
    let v1_iter  =v1.iter();
    let total :i32 = v1_iter.sum();
    assert_eq!(total,6);
}

#[test]
fn iterator_map_sum() {
    let v1 =vec![1,2,3];
    // v1.iter().map(|x| x +1 );//需要用消耗型适配器消耗，否则不会进行+1 操作   
    let v2: Vec<_>=v1.iter().map(|x| x +1 ).collect();//Vec<_>这个下划线表示让编译器推断出这个veator里面的类型
    assert_eq!(vec![2,3,4],v2);
}

//迭代器之 使用闭包捕获环境
//filter 方法:接受一个闭包，闭包遍历迭代器元素时，返回bool类型，返回true的元素才会在新生成的迭代器中

#[derive(PartialEq,Debug)]
struct Shoe{
    size:u32,
    style:String,
}
fn shoes_in_my_size(shoes:Vec<Shoe>,shoe_size:u32)->Vec<Shoe>{
    shoes.into_iter().filter(|x| x.size==shoe_size).collect()  //shoes.into_iter() 这个迭代器是取得所有权的
}
#[test]
fn filter_by_size() {
    let shoes = vec![
        Shoe{
            size:10,
            style:String::from("sneaker"),
        },
        Shoe{
            size:13,
            style:String::from("sandal"),
        },
        Shoe{
            size:10,
            style:String::from("boot"),
        },
    ];
    let in_my_soze = shoes_in_my_size(shoes, 10);
    assert_eq!(
        in_my_soze,
        vec![
            Shoe{
                size:10,
                style:String::from("sneaker"),
            },
            Shoe{
                size:10,
                style:String::from("boot"),
            },
        ]

    )
}

//使用 iterator trait 创建自定义的迭代器  其实就是实现next方法

struct Counter{
    count:u32,
}
impl Counter{
    fn new()->Counter{
        Counter{count:0}
    }

}

impl Iterator for Counter{ //为Counter 实现Iterator trait
    type Item = u32;  //迭代类型返回u32
    fn next(&mut self)->Option<Self::Item>{
        if self.count < 5{
            self.count+=1;
            Some(self.count)
        }else{
            None

        }
    }
}
#[test]
fn calling_next_directly() {
    let mut counter  = Counter::new();
    assert_eq!(counter.next(),Some(1));
    assert_eq!(counter.next(),Some(2));
    assert_eq!(counter.next(),Some(3));
    assert_eq!(counter.next(),Some(4));
    assert_eq!(counter.next(),Some(5));
    assert_eq!(counter.next(),None);
}

#[test]
fn using_other_iterator_trait_method() {
    let sum:u32 = Counter::new()
    .zip(Counter::new().skip(1))  //拉链 第一个迭代器和第二个错位迭代器
    .map(|(a,b)| a*b) //map:接受一个闭包，作用于每一个元素，产生一个新迭代器  6 12
    .filter(|x| x%3==0)
    .sum();

    assert_eq!(18,sum);

}