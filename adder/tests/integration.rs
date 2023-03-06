use adder;
mod common;
#[test]
fn ittt_adds_two_really() {
    common::setup();
    assert_eq!(4,adder::add_two(2));
}

//创建集成测试步骤：创建tests目录  tests目录下的每一个测试文件都是一个单独的create(包)
//运行一个特定的集成测试    cargo test 函数名
//运行某个测试文件内的所有测试 cargo test --test 文件名
//集成测试中的子模块
//tests 下面的目录不会被试做单独的create编译并单独编译，需要的时候可以导入

//针对binary create 只含有src/main.rs 不含有src/lib.rs  不能再tests目录下创建集成测试
//只有library create 才能暴露函数给其他create用，binary create意味着独立运行