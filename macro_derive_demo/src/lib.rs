// 引入宏相关的依赖
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse};

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse(input).unwrap();
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let content = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    content.into()
}

#[proc_macro_attribute]
pub fn custom_attribute(_input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    annotated_item
}

#[proc_macro]
pub fn custom_fn_macro(input: TokenStream) -> TokenStream {
    input
}
