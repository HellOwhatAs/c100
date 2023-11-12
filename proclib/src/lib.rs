use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn proc_func(_item: TokenStream) -> TokenStream {
    let path = parse_macro_input!(_item as LitStr).value();
    let binding = std::fs::read_to_string(path).unwrap();
    let s: Vec<&str> = binding
        .lines()
        .filter(|x| !x.contains(['-', ' ', '=']) && x.chars().map(|c| ((c as u8).wrapping_add(1)).wrapping_sub('a' as u8) as usize).sum::<usize>() == 100)
        .collect();
    quote!{
        {
            let result: Vec<&str> = vec![#(#s),*];
            result
        }
    }.into()
}