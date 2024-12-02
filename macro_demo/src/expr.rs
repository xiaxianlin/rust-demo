macro_rules! foo {
    ( $( $outer:ident ( $( $inner:ident ),* ) ; )* ) => {
        println!("count(outer, 0): $outer repeats {} times", ${count($outer)});
        println!("count(inner, 0): The $inner repetition repeats {} times in the outer repetition", ${count($inner, 0)});
        println!("count(inner, 1): $inner repeats {} times in the inner repetitions", ${count($inner, 1)});
    };
}

macro_rules! attach_iteration_counts {
    ( $( ( $( $inner:ident ),* ) ; )* ) => {
        ( $(
            $((
                stringify!($inner),
                ${index(1)}, // 这指的是外层反复
                ${index()}  // 这指的是内层反复，等价于 `index(0)`
            ),)*
        )* )
    };
}

macro_rules! lets_count {
    ( $( $outer:ident ( $( $inner:ident ),* ) ; )* ) => {
        $(
            $(
                println!(
                    "'{}' in inner iteration {}/{} with '{}' in outer iteration {}/{} ",
                    stringify!($inner), ${index()}, ${len()},
                    stringify!($outer), ${index(1)}, ${len(1)},
                );
            )*
        )*
    };
}

macro_rules! repetition_tuples {
    ( $( ( $( $inner:ident ),* ) ; )* ) => {
        ($(
            $(
                (
                    ${index()},
                    ${index(1)}
                    ${ignore($inner)} // without this metavariable expression, compilation would fail
                ),
            )*
        )*)
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_foo() {
        foo! {
            outer () ;
            outer ( inner , inner ) ;
            outer () ;
            outer ( inner ) ;
        };
    }

    #[test]
    fn test_attach_iteration_counts() {
        let v = attach_iteration_counts! {
            ( hello ) ;
            ( indices , of ) ;
            () ;
            ( these, repetitions ) ;
        };
        println!("{v:?}");
    }

    #[test]
    fn test_lets_count() {
        lets_count!(
            many (small , things) ;
            none () ;
            exactly ( one ) ;
        );
    }

    #[test]
    fn test_repetition_tuples() {
        let tuple = repetition_tuples!(
            ( one, two ) ;
            () ;
            ( one ) ;
            ( one, two, three ) ;
        );
        println!("{tuple:?}");
    }
}
