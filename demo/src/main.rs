macro_rules! foo {
    ($($tts:tt),*) => {
        $(println!("{}",$tts);)*
    };
}

fn main() {
    foo!(1, 2, 3, 4, "test");
}
