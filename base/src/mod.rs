// 模块
// 默认私有
// 可以用来将代码分层地拆分成逻辑单元（模块）
// 并管理它们之间的可见性（公有/私有）
//
// 一个模块是一系列项的集合
// - 函数 fn
// - 结构体 struct
// - trait
// - impl 块
// - 其他模块

// pub 修饰外部可调用

#[allow(dead_code)]
mod database {
    pub struct Database {}

    // 私有函数
    fn db() -> Database {
        println!("db()");
    }

    // 外部可调用
    pub fn run() {
        println!("database::run()");
    }

    // 嵌套模块
    pub mod sql {
        pub fn query() {
            println!("database::sql::query()");
        }

        // 使用 pub(in path) 语法声明的函数只在给定的路径中可见
        // path 必须是父模块或祖先模块
        pub(in crate::database) fn exec() {
            println!("database::sql::exec()");
        }

        // pub(self) 只能当前模块可见
        pub(self) fn cur_mod() {
            println!("cur_mod()");
        }

        // pub(super) 仅在父模块中可见
        pub(super) fn pa_mod() {
            println!("sql::cur_mod()");
        }
    }
}