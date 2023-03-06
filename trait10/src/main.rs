use std::{self,fmt::Display};
fn main() {
    let number_list = vec![12,34,53,23,2]; //将这些代码提取成函数 move occurs because `number_list` has type `Vec<i32>`, which does not implement the `Copy` trait
    // let mut largest=number_list[0];
    // for number in number_list{
    //     if number>largest{
    //         largest=number;
    //     }
    // }
    // println!("the largest number is {}",largest);

    let result = largest1(&number_list);//value borrowed here after move
    println!("the largest number is {}",result);


    // let char_list = vec!['s','k','e','g']; 
    // let result = largest3(&char_list);

    let char_list = vec![String::from("hello"),String::from("world")]; //字符串存在堆上，没有实现Copy trait所以报错
    let result = largest6(&char_list);//用largest5报错
    println!("largest world is :{}",result);

    let p1 = Point{x:5,y:3};
    let p2 = Point{x:"hello",y:'c'};
    let p3 = p1.mixup(p2);
    println!("p3.x={},p3.y={}",p3.x,p3.y);


    let s = 3.to_string();  //因为整数实现了Display trait   在标准库中针对所有实现Display trait的类型 都给他们实现了ToString这个trait
    //ToString 这个trait里面就有to_string()这个方法

}


fn largest1(list: &[i32])->i32{
    let mut largest = list[0];
    for &item in list{
        if item>largest{
            largest=item;
        }
    }
    largest
}
fn largest2(list: &[i32])->i32{
    let mut largest = list[0];
    for item in list{
        if *item>largest{
            largest=*item;
        }
    }
    largest
}

// fn largest3<T>(list: &[T])->T{ //泛型
//     let mut largest = list[0];
//     for &item in list{
//         if item > largest{//T类型不能比较大小，如果要比较大小要实现某个trait
//             largest=item;  //T 实现了std::cmp::PartialOrd  trait 才能使用 > 
//         }
//     }
//     largest
// }

// fn largest4<T: std::cmp::PartialOrd>(list: &[T])->T{ //泛型
//     let mut largest = list[0];  //2、这里报错,元素不能移出list[],因为list[] 切片里的T 没有实现 Copy 这个trait
//     for &item in list{
//         if item > largest{//1、T类型不能比较大小，如果要比较大小要实现某个trait
//             largest=item;  
//         }
//     }
//     largest
// }

fn largest5<T: std::cmp::PartialOrd+ Copy>(list: &[T])->T{ //泛型
    let mut largest = list[0];  //2、这里报错,元素不能移出list[],因为list[] 切片里的T 没有实现 Copy 这个trait
    for &item in list{
        if item > largest{//1、T类型不能比较大小，如果要比较大小要实现某个trait
            largest=item;  
        }
    }
    largest
}

fn largest6<T: std::cmp::PartialOrd+ Clone>(list: &[T])->T{ //泛型
    let mut largest = list[0].clone();  // T在堆上的时，要实现Clone() trait 
    for item in list{
        if item > &largest{
            largest=item.clone();  
        }
    }
    largest
}

fn largest7<T: std::cmp::PartialOrd>(list: &[T])->&T{ //和 largest6 功能一样 返回引用就不需要Clone trait了
    let mut largest = &list[0];  // 直接引用，不需要克隆
    for item in list{
        if item > &largest{
            largest=item;  
        }
    }
    largest
}

struct point<T>{  //结构体泛型
    x:T,
    y:T,
}

impl<T> point<T>{  //impl<T> point<T>,表示在类型T上实现方法。结构体实现方法中应用泛型
    fn x(&self)->&T{
        &self.x
    }
}

impl  point <i32>{ //针对具体类型实现方法。结构体实现方法中应用i32类型，只有i32类型能使用本方法
    fn x1(&self)->&i32{
        &self.x
    }
}

fn struct_trait(){
    let integer = point{x:1,y:2};
    let float = point{x:1.0,y:2.0};
}

enum Option<T>{  //枚举泛型
    Some(T),
    None,
}
enum Result<T,E>{
    Ok(T),
    Err(E),
}

struct Point<T,U>{
    x:T,
    y:U,
}
impl<T,U> Point<T,U>{ //结构体泛型类型可以和实现方法的泛型不一样，当然也可以一样
    fn mixup<V,W>(self,other:Point<V,W>)->Point<T,W>{
        Point{
            x:self.x,
            y:other.y,
        }
    }
}


struct Pair<T>{
    x: T,
    y: T,

}
impl <T> Pair<T>{
    fn new(x: T,y: T)->Self{
        Self {x,y}
    }
}

impl<T: Display+ PartialOrd>Pair<T>{ //当T 实现了Display+ PartialOrd 这两个trait 他才拥有方法cmp_display
    fn cmp_display(&self){
        if self.x >=self.y{
            println!("the largest number is:{}",self.x);
        }else{
            println!("the largest number is:{}",self.y);
        }
    }
}


//对所有满足Display trait 约束的类型 T 都实现了ToString 这个trait  这个称为覆盖实现
//也就是对任何实现了Display trait的类型 调用ToString 这个trait
// impl <T: Display> ToString for T {}  

