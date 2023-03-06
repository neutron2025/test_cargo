use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;
//  hello_macro目录下 cargo build hello_macro
//  hello_macro_derive目录下 cargo build hello_macro_derive
//  pancakes 目录下 cargo run
#[derive(HelloMacro)]
struct Pancakes;

fn main() {
   Pancakes::hello_macro();
}
