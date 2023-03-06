pub mod hosting{  //默认私有，父级模块不可访问子模块私有条目。子模块可以访问所有祖先条目
    pub fn add_to_waitlist(){}
    fn some_function(){}
}