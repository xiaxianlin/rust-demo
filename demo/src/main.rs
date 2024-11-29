use macro_derive_demo::HelloMacro;

trait HelloMacro {
    fn hello_macro();
}

#[derive(HelloMacro)]
struct MyStruct;

#[derive(HelloMacro)]
struct YourStruct;

fn main() {
    println!("Hello, world!");
    MyStruct::hello_macro();
    YourStruct::hello_macro();
}
