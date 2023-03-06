use rand::Rng;
use std::cmp::Ordering;
use std::io;

/// cargo run
/// cargo build
/// cargo check
/// cargo update
///
fn main() {
    println!("猜数游戏!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("神秘数字是：{}", secret_number);

    loop {
        println!("猜测一个数：");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取");
        println!("您猜测的数字是：{}", guess);
        // let guess:u32 = guess.trim().parse().expect("please type anumber");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("you win");
                break;
            }
            Ordering::Greater => println!("too big"),
            Ordering::Less => println!("too small"),
        }
    }
}
