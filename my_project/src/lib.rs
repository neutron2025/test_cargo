mod front_of_house{//同级可以相互调用，不管是公有私有
    pub mod hosting{  //默认私有，父级模块不可访问子模块私有条目。子模块可以访问所有祖先条目
        pub fn add_to_waitlist(){}  //被标记为pub后才能被下面调用
        fn some_function(){}
    }
}

use crate::front_of_house::hosting;  //use 函数指定到父级，struct，enum指定到完整路径    被引用到当前作用域，但是私有的some_function()无法被当前作用域使用
///pub use

pub fn eat_at_restaurant(){ //pub 标记为公有条目
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();

    let mut meal= back_of_house::Breakfast::summper("Rye");
    meal.toast=String::from("wheat");
    println!("i'd like {} toast please",meal.toast);
    // meal.seasonal_fruit = String::from("blueberries"); //私有的，报错

    hosting::add_to_waitlist();//use 将模块引入当前作用域.遵循私有性原则，不可调用some_function
}


fn serve_order(){}
mod back_of_house{
    fn fix_incorrect_order(){
        cook_order();
        super::serve_order();
        crate::serve_order()
    }

    fn cook_order(){}

    pub struct Breakfast{ // pub 标记的struct是公有的，但里面默认还是私有的，除非标记为pub
        pub toast: String,    
        seasonal_fruit: String,//这是私有的
    }
    impl Breakfast{
        pub fn summper(toast: &str)->Breakfast{
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    pub enum Appetizer{  //pub 标记的枚举，里面的条目也是公有的
        Soup,
        Salad,
    }
}



