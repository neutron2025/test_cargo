extern crate proc_macro;

use crate::proc_macro::TokenStream;//可以借用proc_macro包提供的编译器接口，从而在代码中读取和操作rust代码
use quote::quote;//能够将syn转化的数据结构，重新转化为rust代码
use syn;  //可以将rust代码从字符串转化为可供我们进一步操作的数据结构

//当用户在某个类型上 标注 #[derive(HelloMacro)] 自动调用 hello_macro_derive，就是因为在函数上标注了 #[proc_macro_derive(HelloMacro)]
//而且在属性里面指明和HelloMacro 这个trait
#[proc_macro_derive(HelloMacro)] 
pub fn hello_macro_derive(input: TokenStream) -> TokenStream{  //此函数负责解析TokenStream
    let ast =syn::parse(input).unwrap();
    impl_hello_macro(&ast) //impl_hello_macro负责转化语法树
}
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream{
    let name =&ast.ident;//ident里面有被标注类型的名称
    let gen = quote!{ //quote!定义我们期望返回的rust代码
        impl HelloMacro for #name{ //将#name替换为变量name里面的值
            fn hello_macro(){
                println!("hello macro my name is {}",stringify!(#name));//stringify!接受一个rust表达式，编译时将表达式转换为字符串的字面值（例如“1+2”字符串），rust内置
            }
        }
    };
    gen.into() //调用into方法，被编译器理解
}