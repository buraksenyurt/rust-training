extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, ItemFn, LitStr};

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

#[proc_macro_attribute]
pub fn work_time_effort_log(attr: TokenStream, item: TokenStream) -> TokenStream {
    let log_level = parse_macro_input!(attr as LitStr).value();

    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_block = &input.block;

    let expanded = quote! {
        fn #fn_name() {
            let start = std::time::Instant::now();
            #fn_block
            let duration = start.elapsed();
            match #log_level {
                "info" => println!("[INFO] Total execution time {}: {:?}", stringify!(#fn_name), duration),
                "debug" => println!("[DEBUG] Total execution time {}: {:?}", stringify!(#fn_name), duration),
                "error" => eprintln!("[ERROR] Total execution time {}: {:?}", stringify!(#fn_name), duration),
                _ => println!("Unknown log level: {}", #log_level),
            }
        }
    };

    expanded.into()
}

#[proc_macro_attribute]
pub fn log_execution(attr: TokenStream, item: TokenStream) -> TokenStream {
    let logger_expr = parse_macro_input!(attr as Expr);

    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_block = &input.block;

    let expanded = quote! {
        fn #fn_name() {
            let start = std::time::Instant::now();
            #fn_block
            let duration = start.elapsed();
            #logger_expr.log(&format!("Execution time for {}: {:?}", stringify!(#fn_name), duration));
        }
    };

    expanded.into()
}
