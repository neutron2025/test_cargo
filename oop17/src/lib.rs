
pub trait Draw{
    fn draw(&self);
}

pub struct Screen{
    pub components:Vec<Box<dyn Draw>>, //只要是实现了Draw trait的类型都可以放在里面，比使用泛型好
}
impl Screen{
    pub fn run(&self){
        for component in self.components.iter() {
            component.draw();   //画元素时，只关心有没有draw,不关心类型
        }
    }
}

//--------------------------------------

// pub struct Screen<T:Draw>{   //使用泛型，加入T是Button 类型，那就只能在下面的vector中放Button，而上面的trait的动态匹配则更好一点
//     pub components:Vec<T>,
// }
// impl<T> Screen<T>{
//     pub fn run(&self){
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

pub struct Button{
    pub width:u32,
    pub height:u32,
    pub label:String,
}

impl Draw for Button{
    fn draw(&self){
        //绘制一个按钮
    }
}


pub struct Post{
    state:Option<Box<dyn State>>,
    content:String,
}

impl Post{
    pub fn new()->Post{
        Post{
            state:Some(Box::new(Draft {})),
            content:String::new(),
        }
    }

    pub fn add_text(&mut self,text:&str){
        self.content.push_str(text);
    }

    pub fn content(&self)->&str{
        self.state.as_ref().unwrap().content(&self)
    }
    pub fn approve(&mut self){
        if let Some(s) = self.state.take() {
            self.state=Some(s.approve())
            
        }
    }

    pub fn request_review(&mut self){
        if let Some(s) = self.state.take() {
            self.state=Some(s.request_review())
            
        }
    }
}

trait State{
    fn request_review(self:Box<Self>)->Box<dyn State>;
    fn approve(self:Box<Self>)->Box<dyn State>;
    fn content<'a>(&self,post:&'a Post)->& 'a str{
        ""
    }
}
struct Draft {}

impl State for Draft{
    fn request_review(self:Box<Self>)->Box<dyn State>{
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>)->Box<dyn State>{
        self
    }
}
struct PendingReview {}
impl State for PendingReview{
    fn request_review(self: Box<Self>)->Box<dyn State>{
        self
    }
    fn approve(self: Box<Self>)->Box<dyn State>{
        self
    }
}


struct Published {}

impl State for Published{
    fn request_review(self: Box<Self>)->Box<dyn State>{
        self
    }
    fn approve(self: Box<Self>)->Box<dyn State>{
        self
    }
    fn content<'a>(&self,post:&'a Post)->& 'a str{
        &post.content()
    }
}