// In your proc-macro crate
use proc_macro::TokenStream;
use quote::quote;
use syn::{ parse_macro_input, ItemFn };

#[proc_macro_attribute]
pub fn function_tracker(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(item as ItemFn);

    let attr_name = attr.to_string();

    #[cfg(debug_assertions)]
    {
        let fn_name = function.sig.ident.to_string();

        let fn_name_literal = proc_macro2::Literal::string(&fn_name);
        let stmts = &function.block.stmts;
        let timing_code =
            quote! {{
                use std::cell::Cell;
            thread_local! {
                static RECURSION_DEPTH: Cell<u32> = Cell::new(0);
            }

            RECURSION_DEPTH.with(|depth| {
                let current_depth = depth.get();
                depth.set(current_depth + 1);

                let _____start_____ = if current_depth == 0 {
                    Some(std::time::Instant::now())
                } else {
                    None
                };

                let _____result_____ = {
                    #(#stmts)*
                };

                let _____literal_____ = if #attr_name.len() == 0 {
                        #fn_name_literal
                    } else {
                        &(#attr_name[1..#attr_name.len()-1])
                    };


                if let Some(start_time) = _____start_____ {
                    let elapsed = start_time.elapsed();

                    profiler::insert_time(_____literal_____, elapsed);
                } else {
                    profiler::increment_recursive_calls(_____literal_____)
                }

                depth.set(current_depth);

                _____result_____
            })
        }};

        let timed_block: syn::Block = match syn::parse2(timing_code) {
            Ok(block) => block,
            Err(err) => {
                return TokenStream::from(err.to_compile_error());
            }
        };
        function.block = Box::new(timed_block);

        TokenStream::from(quote! { #function })
    }

    #[cfg(not(debug_assertions))]
    TokenStream::from(quote! {
        #function
    })
}

#[proc_macro_attribute]
pub fn start(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(item as ItemFn);

    let attr: String = attr.to_string();

    #[cfg(debug_assertions)]
    {
        let stmts = &function.block.stmts;
        let timing_code =
            quote! {{
               
            let start = std::time::Instant::now();

            let result = {
                #(#stmts)*
            };

            let elapsed = start.elapsed();

            profiler::insert_time("PROGRAM", elapsed);

            if #attr != "false"  {
                profiler::show_profiler();
            }
           
            result
        }};

        let timed_block: syn::Block = match syn::parse2(timing_code) {
            Ok(block) => block,
            Err(err) => {
                return TokenStream::from(err.to_compile_error());
            }
        };

        function.block = Box::new(timed_block);

        TokenStream::from(quote! { #function })
    }

    #[cfg(not(debug_assertions))]
    TokenStream::from(quote! {
        #function
    })
}
