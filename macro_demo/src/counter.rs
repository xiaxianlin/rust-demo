macro_rules! count_tts {
    ($($tts:tt)*) => {0usize $(+ replace_expr!($tts 1usize))*};
}

macro_rules! count_tts_recv {
    () => {0usize};
    ($_head:tt $($tail:tt)*) => {1usize + count_tts_recv!($($tail)*)};
}

macro_rules! count_tts_arr {
    ($($tts:tt)*) => {<[()]>::len(&[$(replace_expr!($tts ())),*])};
}

macro_rules! count_idents {
    ($($idents:ident),* $(,)*) => {
        {
            #[allow(dead_code, non_camel_case_types)]
            enum Idents { $($idents,)* __CountIdentsLast }
            const COUNT: u32 = Idents::__CountIdentsLast as u32;
            COUNT
        }
    };
}

macro_rules! count_tts_bit {
    () => { 0 };
    ($odd:tt $($a:tt $b:tt)*) => { (count_tts_bit!($($a)*) << 1) | 1 };
    ($($a:tt $even:tt)*) => { count_tts_bit!($($a)*) << 1 };
}

macro_rules! abacus {
    ((- $($moves:tt)*) -> (+ $($count:tt)*)) => {
        {
            println!("{} [-]{} | [+]{}", "-+1", stringify!($($moves)*), stringify!($($count)*));
            abacus!(($($moves)*) -> ($($count)*))
        }
    };
    ((- $($moves:tt)*) -> ($($count:tt)*)) => {
        {
            println!("{} [-]{} | - {}", "- 2", stringify!($($moves)*), stringify!($($count)*));
            abacus!(($($moves)*) -> (- $($count)*))
        }
    };
    ((+ $($moves:tt)*) -> (- $($count:tt)*)) => {
        {
            println!("{} [+]{} | [-]{}", "+-3", stringify!($($moves)*), stringify!($($count)*));
            abacus!(($($moves)*) -> ($($count)*))
        }
    };
    ((+ $($moves:tt)*) -> ($($count:tt)*)) => {
        {
            println!("{} [+]{} | + {}", "+ 4", stringify!($($moves)*), stringify!($($count)*));
            abacus!(($($moves)*) -> (+ $($count)*))
        }
    };

    (() -> ()) => {0};
    (() -> (- $($count:tt)*)) => {{-1 + abacus!(() -> ($($count)*)) }};
    (() -> (+ $($count:tt)*)) => {{1 + abacus!(() -> ($($count)*)) }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_abacus() {
        println!(
            "算盘游戏：左边与右边异号时抵消；非异号时，把左边的符号转移到右边；左边无符号时，游戏结束，计算右边得分"
        );
        println!(
            "图示注解：左右符号消耗情况，分支编号，[消失的符号] 左边情况 | [消失的符号] 右边情况\n"
        );

        println!("计数结果：{}\n", abacus!((++-+-+) -> (--+-+-)));
        println!("计数结果：{}\n", abacus!((++-+-+) -> (++-+-+)));
        println!("计数结果：{}\n", abacus!((---+) -> ()));
        println!("计数结果：{}\n", abacus!((++-+-+) -> ()));
        println!("计数结果：{}\n", abacus!((++-+-+++--++---++----+) -> ()));
    }

    #[test]
    fn test_count() {
        assert_eq!(count_tts_bit!(0 1 2), 3);
    }
}
