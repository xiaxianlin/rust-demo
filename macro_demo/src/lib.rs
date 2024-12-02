#![feature(macro_metavar_expr)]
#![allow(unused_variables, unused_macros)]
mod expr;
mod helper;
mod recurrence;

#[macro_export]
macro_rules! vec_string {
    (
        // 开始反复捕获
        $(
            // 每个反复必须包含一个表达式
            $element:expr
        )
        // 由逗号分隔
        ,
        // 0 或多次
        *
    ) => {
        // 在这个块内用大括号括起来，然后在里面写多条语句
        {
            let mut v = Vec::new();

            // 开始反复捕获
            $(
                // 每个反复会展开成下面表达式，其中 $element 被换成相应被捕获的表达式
                v.push(format!("{}", $element));
            )*

            v
        }
    };
}

macro_rules! using_a {
    ($a:ident, $e:expr) => {{
        let $a = 42;
        $e
    }};
}

#[cfg(test)]
mod tests {
    use crate::{call_foo, helped};

    #[test]
    fn test_fn() {
        let four = using_a!(a, a / 10);
    }

    #[test]
    fn unit() {
        helped!();
        call_foo!();
    }
}
