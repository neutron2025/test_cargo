pub struct Post{
    content:String,
}

pub struct DraftPost{
    content:String,
}

impl Post{
    pub fn new()->DraftPost{
        DraftPost{
            content:String::new(),
        }
    }
    pub fn content(&self)->&str{
        &self.content
    }
}

//将状态和行为编码为类型
impl DraftPost{
    pub fn add_text(&mut self,text:&str){
        self.content.push_str(text);
    }

    pub fn request_review(self)-> PendingReviewPost{
        PendingReviewPost{
            content:self.content,
        }
    }
}

pub struct PendingReviewPost{
    content:String,
}

impl PendingReviewPost{
    pub fn approve(self)->Post{
        Post{
            content:self.content,
        }
    }
}

fn main(){
    let mut post =Post::new();
    post.add_text("hello");
    let post.request_review();
    let post.approve();
    assert_eq!("hello",post.content());
}