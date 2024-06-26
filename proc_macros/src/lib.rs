use std::{ fmt::Debug, str::FromStr };

use proc_macro::{ TokenStream, TokenTree };
use quote::{ quote, ToTokens };
use syn::{ parse::{ Parse, ParseStream }, parse_macro_input, punctuated::Punctuated, token };

struct Values {
    _name: syn::Ident,
    _brace_token: token::Brace,
    entries: Vec<Entry>,
}

struct Entry {
    ident: syn::Ident,
    _paren_token: token::Paren,
    ty: syn::Type,
}

impl Debug for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} , {:?}", self.ident, self.ty.to_token_stream().to_string())
    }
}

impl Parse for Entry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Entry {
            ident: input.parse()?,
            _paren_token: syn::parenthesized!(content in input),
            ty: content.parse()?,
        })
    }
}

impl Parse for Values {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let _name: syn::Ident = input.parse()?;
        let _brace_token = syn::braced!(content in input);
        let entries = content
            .parse_terminated(Entry::parse, token::Comma)?
            .into_iter()
            .collect::<Vec<_>>();
        Ok(Values {
            _name,
            _brace_token,
            entries,
        })
    }
}

#[proc_macro]
pub fn make_answer(input: TokenStream) -> TokenStream {
    let Values { entries, .. } = parse_macro_input!(input as Values);

    let value_variants = entries.iter().map(|Entry { ident, ty, .. }| {
        let ty = match &ty {
            syn::Type::Path(type_path) if type_path.path.is_ident("Self") => quote! { Box<#ty> },
            _ => quote! { #ty },
        };
        quote! {
            #ident(#ty),
        }
    });

    let value_type_variants = entries.iter().map(|Entry { ident, ty, .. }| {
        let ty = match &ty {
            syn::Type::Path(type_path) if type_path.path.is_ident("Self") => quote! { Box<#ty> },
            _ => quote! {},
        };
        quote! {
            #ident #ty,
        }
    });

    let expanded =
        quote! {
        pub enum Value2 {
            #(#value_variants)*
        }

        // pub enum ValueType {
        //     #(#value_type_variants)*,
        // }
    };

    expanded.into()
}

// message: TokenStream [
//     Ident {
//         ident: "Values",
//         span: #0 bytes(882..888),
//     },
//     Group {
//         delimiter: Brace,
//         stream: TokenStream [
//             Ident {
//                 ident: "Int",
//                 span: #0 bytes(895..898),
//             },
//             Group {
//                 delimiter: Parenthesis,
//                 stream: TokenStream [
//                     Ident {
//                         ident: "isize",
//                         span: #0 bytes(899..904),
//                     },
//                 ],
//                 span: #0 bytes(898..905),
//             },
//             Punct {
//                 ch: ',',
//                 spacing: Alone,
//                 span: #0 bytes(905..906),
//             },
//             Ident {
//                 ident: "Bool",
//                 span: #0 bytes(911..915),
//             },
//             Group {
//                 delimiter: Parenthesis,
//                 stream: TokenStream [
//                     Ident {
//                         ident: "bool",
//                         span: #0 bytes(916..920),
//                     },
//                 ],
//                 span: #0 bytes(915..921),
//             },
//             Punct {
//                 ch: ',',
//                 spacing: Alone,
//                 span: #0 bytes(921..922),
//             },
//         ],
//         span: #0 bytes(889..924),
//     },
// ]
