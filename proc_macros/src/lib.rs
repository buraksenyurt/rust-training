extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn work_time_effort(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_block = &input.block;

    let expanded = quote! {
        fn #fn_name() {
            let start = std::time::Instant::now();
            #fn_block
            let duration = start.elapsed();
            println!("Total execution time {}: {:?}", stringify!(#fn_name), duration);
        }
    };

    expanded.into()
}
