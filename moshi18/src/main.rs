
/**模式匹配
 * rust中的一种语法，用来匹配复杂和简单类型的结构
 * 将模式与匹配表达式和其他构造结合使用，可以更好的控制程序的控制流
 * 模式由以下元素组成：
 * 字面值
 * 结构的数组，enum,struct,tuple
 * 变量
 * 通配符
 * 占位符
 * 
 * 想要使用模式，需要将其与某个值进行比较：如果模式匹配，就可以在代码中使用这个值的相应部分
 */

 //用到模式的地方


/**
 * match 的arm
 * 
 * match value{
 * pattern => expression,
 * pattern => expression,
 * pattern => expression,
 * }
 * 
 * match 表达式的要求：详尽，尽可能包含所有可能性
 * 
 * 一个特殊的模式： _(下划线)
 * -它会匹配任何东西，所有可能的值
 * -不会绑定到变量
 * -通常用于match的最后一个arm;或用于忽略某些值
 * 
 * 条件if let 表达式：作为一种简短的方式来等价的代替只有一个匹配项的match
 * -if let 可选的可以拥有else,包括：else if     else if let 但if let 不会检查穷尽性
 * 
 */

fn main() {
    let favorite_color : Option<&str> =None;
    let is_tuesday= false;
    let age:Result<u8,_> = "34".parse();


    if let Some(color) = favorite_color {
        println!("using your favorite color {}",color);
        
    }else if is_tuesday{
        println!("tuesday is green day");
    }else if let Ok(age) = age {
        if age > 30{
            println!(">30");
        }else{
            println!("<30");
        }
        
    }else{
        println!("blue ");
    }
    println!("Hello, world!");

    forxunhuan();
    fooo(3, 4);//3 被忽略
}

// while let 条件循环: 只要模式继续满足匹配的条件，那它允许while循环一直运行

fn whilelet(){
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top)  = stack.pop(){
        println!("{}",top);
    }
}

// for循环: 模式紧随for 关键字后面的值

fn forxunhuan(){
    let v=vec!['a','b','c'];
    for (index,value) in v.iter().enumerate(){
        println!("{}:{}",value,index);
    }
}



// let语句也是模式 
// let pattern = expression;

fn letyuju(){
    let a=5;
    let (q,w) = (1,2);
}

//函数参数也是模式 
fn foo(x:i32){

}
fn print_coordinates(&(x,y):&(i32,i32)){
    println!("current location:({},{})",x,y);
}
fn test(){
    let point = (3,5);
    print_coordinates(&point);
}

/***
 *可辩驳性：模式是否会无法匹配
 *模式的两种形式：可辩驳的，无可辩驳的（可失败的，不可失败的）
 * -能够匹配任何可能传递的值的模式：无可辩驳的  let = 5；
 * -对于某些值，无法进行匹配的模式：可辩驳的    if let Some(x) = a_value
 * -函数参数，let语句，for循环只接受无可辩驳的模式
 * -if let ,   while let 接受可辩驳，无可辩驳两种模式 
 * 
 */

 fn bianbo(){
     let a:Option<i32> = Some(5);
    //  let Some(x) = a; //这里Some是可失败的，就是可辩驳的。不匹配出错
    if let Some(x) = a{
         
    }
 }
 //match 表达式，除了最后一个分支，其他的分支必须都是可辩驳的（可失败的）

 /**
  * 匹配字面值：模式可直接匹配字面值
  */

  fn pipei1(){
      let x = 1;
      match x{
          1=>println!("one"),
          2=>println!("two"),
          3=>println!("three"),
          _=>println!("anything"),
      }
  }

/**
  * 匹配命名变量：命名的变量是可匹配任何值得无可辩驳模式
  */
  fn pipei2(){
    let x = Some(5);
    let y = 10; 

    match x{
        Some(50)=>println!("got 50"),
        Some(y)=>println!("{:?}",y),  //这边的y 其实是括号作用域内部的变量，匹配
        _=>println!("{:?}",x),
    }
    println!("x={:?},y={:?}",x,y);
}
/**
  * 匹配多重模式：在match表达式中，使用 | 就是或的意思，可以匹配多种模式
  * 
  */

  fn pipei3(){
    let x =1;
    match x{
       1|2 =>println!("1"),
        3=>println!("2"),  //这边的y 其实是括号作用域内部的变量，匹配
        _=>println!("3"),
    }
}

/**
  * 使用 ..=来匹配某个范围内的值
  * 
  */
  fn pipei4(){
    let x =1;
    match x{
       1..=5 =>println!("1,2,3,4,5其中一个值"),
        _=>println!("other"),
    }
    let x ='c';
    match x{
       'a'..='j' =>println!("a--j"),
       'k'..='z' =>println!("k--z"),
        _=>println!("other"),
    }
}
/**
  * 结构以分解值：可以使用struct,enum,tuple,从而引用这些类型值的不同部分
  * 
  */
  struct Point{
      x:i32,
      y:i32,
  }

  fn pipei5(){
    let p =Point{x:0,y:7};
    let Point{x:a,y:b} =p;//此处解构
    assert_eq!(0,a);
    assert_eq!(7,b);

    let Point{x,y} =p;//此处解构简写
    assert_eq!(0,a);
    assert_eq!(7,b);

    match p{
        Point{x,y:0}=>println!("{}",x),
        Point{x:0,y}=>println!("{}",y),
        Point{x,y}=>println!("{}:{}",x,y),

    }
}

//解构enum
enum Message{
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}
fn enumpp(){
    let msg = Message::ChangeColor(0,160,225);
    match msg{
        Message::Quit=>{println!("quit")}
        Message::Move{x,y}=>{println!("x{}y{}",x,y) }
        Message::Write(text)=>println!("{}",text),
        Message::ChangeColor(r,g,b)=>println!("{}{}{}",r,g,b),
    }
}

//解构嵌套的结构体和enum
enum Color{
    Rgb(i32,i32,i32),
    Hsv(i32,i32,i32),
}
enum Messages{
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(Color),
}
fn enumstructpp(){
    let msg = Messages::ChangeColor(Color::Hsv(0,160,22));
    match msg{
        Messages::ChangeColor(Color::Rgb(r,g,b))=>println!("r,g,b{}{}{}",r,g,b),
        Messages::ChangeColor(Color::Hsv(h,s,v))=>println!("h{},s{},v{}",h,s,v),
        _=>(),

    }
}
//解构struct和tuple
struct Points{
    x:i32,
    y:i32,
}
fn strtup(){
    let ((feet,inches),Point{x,y})=((3,10),Point{x:3,y:-10});   
}
/**
 * 在模式中忽略值：有几种方式可以在模式中忽略整个值或者部分值
 * _：忽略整个值
 * _配合其他模式: 忽略部分
 * 以_开头：
 * ..：忽略值得剩余部分
 */

 // _：忽略整个值
 fn fooo(_:i32,y:i32){
     println!("only use y parameter:{}",y);

 }

 // _配合其他模式: 忽略值的部分
 fn _somehulue(){
     let mut setting_value = Some(5);
     let new_setting_value = Some(10);

     match (setting_value,new_setting_value){
         (Some(_),Some(_)) =>{  //里面的值都是Some就行
             println!("匹配some类型")
         }
         _=>{
             setting_value = new_setting_value;
         }
     }
     println!("setting is {:?}",setting_value);
 }

 fn _pipei(){
     let numbers=(2,4,6,8,10);
     match numbers{
         (first,_,third,_,fifth)=>{
             println!("{}{}{}",first,third,fifth);
         }
     }
 }

 // 通过使用_开头命名来忽略未使用的变量

 fn hulue2(){
     let _x= 10;//忽略未使用的变量,不会提示警告

     let y = 10;

     let s= Some(String::from("hello"));
    //  if let Some(_s) = s {  //这样会进行绑定操作，s所有权给 _s
        if let Some(_) = s { //这样不会进行绑定操作
         println!("found a string");
         
     }
     println!("{:?}",s);//可以使用s
 }

 //使用..忽略值的剩余部分
 struct PoinT{
     x:i32,
     y:i32,
     z:i32,
 }
 fn somefn(){
     let origin = PoinT{x:0,y:0,z:0};
     match origin {
         PoinT{x, ..}=>println!("x is {}",x),  //只匹配x,剩余的不管
     }

     let numbers =(2,4,8,16,32);
     match numbers{
         (first,..,last)=>{ //只匹配第一个和最后一个 用变量first last 来表示
             println!("Some numbers:{},{}",first,last);
         }
     }

    //  match numbers{
    //     (..,second,..)=>{//这里发生歧义，不知道是中间哪一个
    //         println!("Some numbers:{}",second);
    //     }
    // }
 }

 /**
  * 使用match 守卫来提供额外的条件
  *  match 守卫就是match arm模式后额外的if 条件，想要匹配该条件也必须满足这个if
  *  match守卫适用于比单独的模式更复杂的场景
  * 
  */
  fn matchshouwei(){
      let num  =Some(4);
      match num{
          Some(x) if x <5 =>println!("less than five:{}",x),//if x <5 就是match 守卫
          Some(x) =>println!("{}",x),
          None =>(),
      }
  }

  fn matchshouwei2(){
    let x  =Some(5);
    let y  =10;    

    match x{
        Some(50) => println!("GOT 50"),
        Some(n) if n==y  => println!("mached n ={:?}",n),//match 守卫  n就是5
        _ =>println!("default case, x={:?}",x),
    }
    println!("at the end :x = {:?},y={:?}",x,y);
}

fn matchshouwei3(){
    let x  =4;
    let y  =false;    

    match x{
        4|5|6 if  y =>println!("yes"),//match 守卫
        _ =>println!("no"),
    }
    
}

/***
 * @绑定：@符号可以让我们创建一个变量，该变量可以在测试某个值是否与模式匹配的同时保存该值
 * 
 */

 enum Mess{
     Hello{id:i32},
 }
 fn function(){
     let msg = Mess::Hello{ id: 5};
     match msg{
         Mess::Hello{
             id:id_variable @ 3..=7,//要求id的值在3-7范围内，同时使用@将值存在变量id_variable中
         }=>{
             println!("found an id in range {}",id_variable)
         }Mess::Hello{
             id:10..=12
         }=>{
             println!("found an id in another range")
         }
         Mess::Hello{id}=>{println!("found some another id:{}",id)}
     }
 }