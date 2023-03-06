
use trait10_2::Summary;
use trait10_2::Tweet;
fn main() {
    let tweet  = Tweet{
        username: String::from("horse_ebook"),
        content: String::from("of cause you may already know"),
        reply: false,
        reteet: false,
    };
    println!("Hello, world!--{}",tweet.summarize());
    println!("Hello, world!--{}",tweet.summary_author());
}
