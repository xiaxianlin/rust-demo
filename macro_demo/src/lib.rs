#![feature(macro_metavar_expr)]
#![allow(unused_variables, unused_macros, dead_code)]
#![feature(trace_macros)]

mod counter;
mod expr;
mod helper;
mod recurrence;
mod tt;

#[macro_export]
macro_rules! vec_string {
    ( $($element:expr),*) => {
        {
            let mut v = Vec::new();
            $( v.push(format!("{}", $element));)*
            v
        }
    };
}

macro_rules! call_with_larch {
    ($callback:ident) => {
        $callback!(larch)
    };
}

macro_rules! expand_to_larch {
    () => {
        larch
    };
}

macro_rules! recognize_tree {
    (larch) => {
        println!("#1, the Larch.")
    };
    (redwood) => {
        println!("#2, the Mighty Redwood.")
    };
    (fir) => {
        println!("#3, the Fir.")
    };
    (chestnut) => {
        println!("#4, the Horse Chestnut.")
    };
    (pine) => {
        println!("#5, the Scots Pine.")
    };
    ($($other:tt)*) => {
        println!("I don't know; some kind of birch maybe?")
    };
}

macro_rules! callback {
    ($callback:ident( $($args:tt)* )) => {
        $callback!( $($args)* )
    };
}

#[macro_export]
macro_rules! as_expr {
    ($e:expr) => {
        $e
    };
}

macro_rules! init_array {
    (@accum (0, $_e:expr) -> ($($body:tt)*)) => {
        init_array!(@as_expr [$($body)*])
    };
    (@accum (1, $e:expr) -> ($($body:tt)*)) => {
        init_array!(@accum (0, $e) -> ($($body)* $e,))
    };
    (@accum (2, $e:expr) -> ($($body:tt)*)) => {
        init_array!(@accum (1, $e) -> ($($body)* $e,))
    };
    (@accum (3, $e:expr) -> ($($body:tt)*)) => {
        init_array!(@accum (2, $e) -> ($($body)* $e,))
    };
    (@as_expr $e:expr) => {$e};
    [$e:expr; $n:tt] => {
        {
            let e = $e;
            init_array!(@accum ($n, e.clone()) -> ())
        }
    };
}

macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {
        $sub
    };
}

macro_rules! tuple_default {
    ($($tup_tys:ty),*) => {
        (
            $(
                replace_expr!(
                    ($tup_tys)
                    Default::default()
                ),
            )*
        )
    };
}

macro_rules! call_a_or_b_on_tail {
    ((a: $a:ident, b: $b:ident), call a: $($tail:tt)*) => {
        $a(stringify!($($tail)*))
    };

    ((a: $a:ident, b: $b:ident), call b: $($tail:tt)*) => {
        $b(stringify!($($tail)*))
    };

    ($ab:tt, $_skip:tt $($tail:tt)*) => {
        call_a_or_b_on_tail!($ab, $($tail)*)
    };
}

macro_rules! as_expr {
    ($e:expr) => {
        $e
    };
}
macro_rules! as_item {
    ($i:item) => {
        $i
    };
}
macro_rules! as_pat {
    ($p:pat) => {
        $p
    };
}
macro_rules! as_stmt {
    ($s:stmt) => {
        $s
    };
}
macro_rules! as_ty {
    ($t:ty) => {
        $t
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_ast_coercion() {
        as_item! {struct Dummy;}

        as_stmt!(let as_pat!(_): as_ty!(_) = as_expr!(42));
    }

    fn compute_len(s: &str) -> Option<usize> {
        Some(s.len())
    }

    fn show_tail(s: &str) -> Option<usize> {
        println!("tail: {:?}", s);
        None
    }

    #[test]
    fn test_call_a_or_b_on_tail() {
        assert_eq!(
            call_a_or_b_on_tail!(
                (a: compute_len, b: show_tail),
                the recursive part that skips over all these
                tokens doesnt much care whether we will call a
                or call b: only the terminal rules care.
            ),
            None
        );
        assert_eq!(
            call_a_or_b_on_tail!(
                (a: compute_len, b: show_tail),
                and now, to justify the existence of two paths
                we will also call a: its input should somehow
                be self-referential, so lets make it return
                some eighty-six!
            ),
            Some(85)
        );
    }
}
