#[cfg(test)]
mod tests {
    #[test]   //测试函数标注
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn another(){
        panic!("shibaile"); //用panic！直接让其失败
    }
}
//cargo new adder --lib  创建库，里面会自带一个test
//cargo test 命令会执行所有测试
//测试函数 panic就表示失败
//assert！ 测试某个状态是否为true ,false则不通过测试

#[derive(Debug)]
pub struct Rectangle{
    length:u32,
    width:u32,
}

impl Rectangle{
    pub fn canhold(&self,other:&Rectangle)->bool{ //由于返回是bool .非常适合用assert!宏 去测试
        self.length>other.length && self.width>other.width
    }
}
#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn larger_can_hold_smaller() {
        let larger=Rectangle{
            width:7,
            length:8,
        };
        let smaller =Rectangle{
            width:1,
            length:3,
        };
        assert!(larger.canhold(&smaller)); //返回true就通过
    }
}
//assert_eq! assert_ne! 这两个宏，判断两个参数相等或者不等，断言失败自动打印出两个参数的值。使用debug类型打印参数要求实现PartialEq 和Debug trait(基本类型和标准库里，大部分类型都实现了这俩)

pub fn add_two(a: i32)->i32{
    a+2
}


mod test2{
    use super::*;
    #[test]
    fn it_adds_two() {
        assert_eq!(4,add_two(2)); //相等就通过
    }

}

// assert_eq! assert_ne! assert! 可以自定义消息（最后一个参数），自定义消息会被传送给format!宏 可以使用{} 占位符
// 自定义新消息  在assert_eq! assert_ne! 里是第三个参数 ，在assert! 是第二个参数
pub fn greerting(name: &str)->String{
    format!("hello")
}
#[test]
fn greeting_contains_name() {
    let result = greerting("tom");
    assert!(result.contains("tom"),"greeting didn't contain name,value was:{}",result);
}


//should_panic 测试     是否如预期的发生恐慌
pub struct Guess{
    value: u32,
}
impl Guess{
    pub fn new(value:u32)->Guess{
        if value<1||value>100{
            panic!("should in 1-100.got:{}",value);
        }
        Guess{value}
    }
}

#[test]
#[should_panic(expected="should in")]  //应该发生恐慌，则测试通过，expected参数内容是否在恐慌里，有就通过，此参数可选
fn greater_than_100() {
    Guess::new(200);
}

//在测试中使用Result<T,E>  不要使用should_panic 标注  无需panic .返回OK通过  

#[test]
fn it_works2()->Result<(),String> {
    if 2 + 3 == 4{
        Ok(())
    }else{
        Err(String::from("some error"))
    }
}

//控制测试的运行
//cargo test 默认并行运行所有，捕获所有输出（不显示)
//cargo test --help    显示所有参数
//cargo test -- --help 显示--后跟的参数   比如cargo test -- --show-output

/**
 * 并行运行测试，确保依赖之间不会相互依赖，不依赖于某个共享状态
 * --test-threads 参数不行并行测试，或者对线程进行细粒度控制
 * 例如cargo test -- --test-threads=1
 * 
 * 
 */
//失败才会打印i got the value  若想成功也打印 cargo test -- --show-output
fn print_and_returns_10(a:i32)->i32{
    println!("i got the value {}",a);
    10
}
#[test]
fn print_and_return_pass() {
    let value = print_and_returns_10(4);
    assert_eq!(10,value);
}
#[test]
fn print_and_return_fail() {
    let value = print_and_returns_10(8);
    assert_eq!(5,value);
}

//按测试的名称运行测试  单个测试 指定名称  cargo test print_and_return_pass

//多个测试  指定名称的一部分或者模块名一部分

//忽略某些耗时测试，运行剩余测试  ignore
#[test]
fn ignore_this() {
    assert_eq!(4,2+2);
}
#[test]
#[ignore] //单独运行被ignore的测试则    cargo test -- --ignored
fn expensive_test() {
    assert_eq!(3,1+1+1);
}

/**
 * 单元测试：可测试private 接口  模块需要#[cfg(test)]标注
 * 集成测试：在库外部，不同目录，只能使用public接口，可能使用到多个模块 不需要#[cfg(test)]标注
 */
//测试私有函数
pub fn add_two2(a:i32,b:i32)->i32{
    a+b
}
fn internal_adder(a:i32,b:i32)->i32{
    a+b
}
#[test]
fn internal_adder_test() {
    assert_eq!(4,internal_adder(2, 2));
}
//集成测试