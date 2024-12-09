//// 在 `helper` crate 里定义 `helped!` 和 `helper!` 宏
#[macro_export]
macro_rules! helped {
    // () => { helper!() } // 这行可能导致 `helper` 不在作用域的错误
    () => {
        $crate::helper!()
    };
}

#[macro_export]
macro_rules! helper {
    () => {
        ()
    };
}

pub mod inner {

    #[macro_export]
    macro_rules! call_foo {
        () => {
            $crate::helper::inner::foo()
        };
    }

    pub fn foo() {
        println!("call foo");
    }
}
