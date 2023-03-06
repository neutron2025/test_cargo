
fn main() {
    println!("Hello, world!");
    let user1 = User{ 
        username: String::from("tom"),
        email: String::from("123@qq.com"),
        active:true,
        account: 232,
    };

    let user2=User{   //struct 的更新语法
        username: String::from("John"),
        email: String::from("222@gmail.com"),
        ..user1
    };

    let black = Color(0,0,0);
    let orange = Point(0,0,0);//这俩是不同的类型 因为是不同的实例

    let w=80;
    let h=30;
    println!("{}",area(w, h));

    let rect=(30,50);
    println!("{}",area2(rect));

    let rectangle = Rectangle{
        width:40,
        height:50,
    };
    let rectangle2 = Rectangle{
        width:35,
        height:55,
    };
    let rectangle3 = Rectangle{
        width:10,
        height:40,
    };

    let square = Rectangle::square(20);

    println!("{}",area3(&rectangle));
    println!("{:#?}",rectangle);
    println!("{}",rectangle.area3());
    println!("{}",rectangle.can_hold(&rectangle3));
    println!("这是一个关联函数构造的边长为 {:?} 的正方形",square);

    let home =IpAddr{
        kind: IpAddKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback =IpAddr{
        kind: IpAddKind::V6,
        address: String::from("::1"),
    };

    let home1=IpAddKind2::V4(127,0,0,1);
    let loopback2 = IpAddKind2::V6(String::from("::1"));

    let q = message::Quit;
    let m = message::Move{x: 12,y: 23};
    let w = message::Write(String::from("hello"));
    let c = message::ChangeColor(0,255,255);
    m.call();


    let some_number=Some(5); //option枚举在遇到入模块中，直接使用
    let some_string = Some("a string");
    let y:Option<i8> = Some(67);

    let c=Coin::Quarter(UsState::Alaska);
    println!("{}",value_in_cents(c));

    let five= Some(5);
    let six=plus_one(five);
    let none = plus_one(None);




}


struct User{ //struct    一旦struct的实例是可变的，那么实例中所有的字段都是可变的，此外struct可以作为函数的返回值
    username: String,
    email: String,
    account: u64,
    active: bool,

}

struct Color(i32,i32,i32);//tuple struct
struct Point(i32,i32,i32);



fn build_user(email: String,username: String)->User {
    User{
        email:email,
        username,
        account:2,
        active:true,
    }
}

fn area(width: i32,heigth: i32)->i32{ //长宽无关联
    width*heigth
}

fn area2(dim: (i32,i32))->i32{ //元组关联了，但是可改变
    dim.0*dim.1
}

#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32,
}
fn area3(rect: &Rectangle)->u32{ //结构体关联，借用不可改变
   rect.height*rect.width
}


impl Rectangle{ //实现结构体的方法  struct 方法

     fn can_hold(&self,other:&Rectangle)->bool{
         self.height>other.height&&self.width>other.width
     }
     fn square(size: u32)->Rectangle{ // 这个是关联函数，用于构造函数
         Rectangle{
             width:size,
             height:size,
         }
     }
}

impl Rectangle{
    fn area3(&self)->u32{ 
        self.height*self.width
     }
}
//关联函数： 可以在impl块里定义不把self作为第一个参数的函数，通常用语构造器


enum IpAddKind{ //枚举
    V4,
    V6,
}

enum IpAddKind2{ //将数据附加到枚举的变体中
    V4(u8,u8,u8,u8),
    V6(String),
}
struct IpAddr{
    kind:IpAddKind,
    address: String,
}

enum message{
    Quit,
    Move {x: i32,y: i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}
impl message{ //枚举跟struct一样，也是用impl来定义方法
     fn call(&self) {}
 }

 enum Coin{
     Penny,
     Nickle,
     Dime,
     Quarter(UsState),
 }
#[derive(Debug)]
 enum UsState{
     Alabama,
     Alaska,
 }

 fn value_in_cents (coin: Coin)->u8{
     match coin{
         Coin::Penny=>{
             println!("penny");
             1
         },
         Coin::Nickle=>5,
         Coin::Dime=>10,
         Coin::Quarter(state)=>{
             println!("{:?}",state);
             25
         },
     }
 }

 fn plus_one(x: Option<i32>)->Option<i32>{
     match x{ // match 匹配必须穷举所有可能
         None=>None,
         Some(i)=>Some(i+1),
     }
 }

fn option_qiongju(){
    let v= Some(0u8);
    match v {
        Some(1)=>println!("one"),
        Some(2)=>println!("two"),
        Some(3)=>println!("three"),
        _=>(), //穷举，但是什么也不发生   代替其余没列出的值
    }


    if let Some(3) = v {  //if let 用于单一匹配    处理只关心一种匹配而忽略其他匹配的情况
        println!("three");
    }else{                    //也可以搭配else使用
        println!("others");
    }
}

