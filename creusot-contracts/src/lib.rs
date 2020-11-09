#![feature(stmt_expr_attributes)]
#![feature(register_tool)]
#![register_tool(creusot)]

extern crate proc_macro;

use std::collections::HashSet;

use proc_macro2::TokenStream;

use syn::*;
use syn::visit::Visit;

use quote::quote;


#[proc_macro_attribute]
pub fn requires(
    attr: proc_macro::TokenStream,
    tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let p: syn::Term = parse_macro_input!(attr);

    let f: ItemFn = parse_macro_input!(tokens);

    let req_toks = format!("{}", quote!{#p});
    // TODO: Parse and pass down all the function's arguments.
    proc_macro::TokenStream::from(quote! {
      #[creusot::spec::requires=#req_toks]
      #f
    })
}

#[proc_macro_attribute]
pub fn ensures(
    attr: proc_macro::TokenStream,
    tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let p: syn::Term = parse_macro_input!(attr);

    let f: ItemFn = parse_macro_input!(tokens);

    let req_toks = format!("{}", quote!{#p});
    // TODO: Parse and pass down all the function's arguments.
    proc_macro::TokenStream::from(quote! {
      #[creusot::spec::ensures=#req_toks]
      #f
    })
}

#[proc_macro]
pub fn invariant(invariant: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let p: syn::Term = parse_macro_input!(invariant);
    let inv_toks = format!("{}", quote!{#p});

    proc_macro::TokenStream::from(quote! {
        #[creusot::spec::invariant=#inv_toks]
        ||{};
    })
}
