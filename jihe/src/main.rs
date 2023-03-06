use std::collections::HashMap;
fn main() {
    println!("Hello, world!");
    let v: Vec<i32> = Vec::new();
    let v = vec![1,2,3,4,5];
    let mut v =  Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);
    v.push(6);
    v.push(7);

    let v1 = vec![1,2,3,4,5]; //vector 的索引和get
    let third: &i32 = &v[2];
    println!("the third element is {}",third);

    match v1.get(0){
        Some(third)=>println!("----"),
        None=>println!("no"),

    }

    let mut vv=vec![1,2,3,4,5];
    let first = &vv[0]; //immutable borrow occurs here 
    // vv.push(6);       mutable borrow occurs here
    println!("{}",first);


    let vz =vec![1,2,3,55,44];  //借用遍历
    for item in &vz{
        println!("{}",item);
    }

    let mut vc = vec![23,543,6,34,112,789];//解引用，数据操作遍历
    for i in &mut vc{
        *i+=50;
    }
    for i in  vc{
        println!("---{}",i);
    }


    let row =vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(10.12),
        SpreadSheetCell::Text(String::from("blue")),
    ];


    //String
    let data = "initial contents";
    let s = data.to_string();
    let s1 = "initial contents".to_string(); //初始化string方法一 to_string()
    let mut s2  = String::from("helli");         //初始化string方法二 
    s2.push_str(&s1);

    let mut s3  = String::from("lo");     
    s3.push('l');

    let ss1 = String::from("hello");
    let ss2 = String::from("world");
    let ss3 = String::from("toe0");
    let ss3=ss1 + &ss2;

    let s= format!("{}-{}-{}",ss2,ss2,ss3);
    println!("{}",s);

    let w= "你好还是看得见";  //string 的遍历不支持索引，其本质上是一个vector
    for i  in w.bytes(){
        println!("{}",i);
    }
    for i  in w.chars(){
        println!("{}",i);
    }
    let s = &w[0..3]; //字符切片[] 必须要沿字符边界
    println!("{}",s);


    let mut scores: HashMap<String,i32> = HashMap::new();//Hashmap是同构的，即key是同一类型，value是同一类型
    scores.insert(String::from("blue"), 10);

    let teams = vec![String::from("blue"),String::from("yellow")]; //另一种hashmap 的定义方式就是通过collect
    let initial_scores = vec![10,12];
    let scores: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();


    let field_name = String::from("favourtwer color");
    let field_value = String::from("blue");
    let mut map =HashMap::new();
    map.insert(&field_name, &field_value);//如果将值的引用插入到hashamp中，值本身不会移动，去掉& 则下面报错，因为所有权已被转移

    println!("{}:{}",field_name,field_value); //这里对于拥有所有权的值，所有权会移动给hashmap 对于实现了copy trait 的类型值会被复制到hashmap中


    let mut sco  = HashMap::new();
    sco.insert(String::from("blue"), 10);
    sco.insert(String::from("green"), 80);

    let team_name = String::from("blue"); //get 获取hashmap 的值
    let scor = sco.get(&team_name);
    match scor{
        Some(s)=>println!("{}",s),
        None=>println!("team not exist!"),
    };

    for (k,v) in &sco{     //hashmap的遍历
        println!("{}:{}",k,v);

    }
    sco.insert(String::from("blue"), 20); //替换hashmap中的值
    println!("{:?}",sco);

    sco.entry(String::from("blue")).or_insert(80); //如果blue的key不存在才会插入blue为80的键值


    let text = "hello world wanderful world by rust";
    let mut map = HashMap::new();
    for word in text.split_whitespace(){   //or_insert  如果key存在到对应的V的可变引用，如果不存在就插进去，返回此值得可变引用
        let count = map.entry(word).or_insert(0); //如果有就下一行，加一，如果没有就单词key计数为0
        *count+=1;
    }
    println!("{:#?}",map);

}

enum SpreadSheetCell{ //枚举可存放多个类型
    Int(i32),
    Float(f64),
    Text(String),
}

