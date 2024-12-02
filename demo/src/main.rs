use fib::Recurrence;
use macro_derive_demo::HelloMacro;

mod fib;
trait HelloMacro {
    fn hello_macro();
}

#[derive(HelloMacro)]
struct MyStruct;

#[derive(HelloMacro)]
struct YourStruct;

fn main() {
    let fib = Recurrence::new();
    for e in fib.take(10) {
        println!("{}", e)
    }
}
