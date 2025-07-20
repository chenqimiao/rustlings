// TODO: Fix the compiler error without taking the macro definition out of this
// module.
mod macros {
    
    ///Rust 1.30 及以上版本，模块内的宏如果要在模块外使用，必须用 #[macro_export] 或放在 crate 根部。
    // pub(crate) 只能用于函数/类型，不能让宏跨模块可见。
    // 高版本 Rust 不支持 use 导入模块内宏，只有 #[macro_export] 才能让 main 访问到宏。
    // 所以，推荐用 #[macro_export] 注解，或者把宏定义移到模块外。
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
