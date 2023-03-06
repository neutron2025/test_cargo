
/**
 * release profile 是预定义的可以自定义
 * cargo 的两个profile
 * dev profile: 适用于开发  cargo build
 * release profile 适用于发布  cargo build --release
 * 
 * [profile.dev]  
 * opt-level = 0
 * 
 * 如何发布create 到 create.io
 *  文档注释 ///  用于生成文档，支持markdown,放置在说明条目之前 ,cargo doc 命令生成文档
 * cargo doc 命令生成文档   
 * cargo doc --open  直接打开了 不用手点
 */

 

fn main() {
    println!("Hello, world!");
}


//# example
//其他常用章节
//-Panics 可能产生panic的场景
//-Errors 如果返回Result,描述可能的错误种类
//-Safety 如果函数处于unsafe调用，就应该解释函数unsafe的原因

// 文档注册作为测试：运行cargo test 将把文档中的示例代码当做测试来运行

//为包含注释的项添加文档注释 //! 通常用于描述create 和模块

//  使用pub use 导出方便使用的公共API 例如 pub use self::mod::enum;
//pub use self::mod::fn;    标注在本文档-使用在其他文档 use create::enum;


//发布 crates   登录crates.io 并创建token 运行命令 cargo login token 
//- 通知cargo 你的api token 存储在本地 ~/.cargo/credentials
// 然后在cargo.toml里面添加元数据 
//cargo publish 发布   cargo publish --allow-dirty
// 发布之后一旦发布，不可覆盖，不可删除
//发布新版本  先修改version 值 参照http://semver.org   然后cargo publish

//cargo yank --vers 1.0.1 撤回版本，新的项目无法依赖，已经存在的项目可以依赖
//cargo yank --vers 1.0.1 --undo 取消撤回

// cargo 工作空间：帮助管理相互关联的create.是一套共享一个cargo.lock和输出文件的包
//1、创建文件夹  2、打开cargo.toml 配置工作空间


