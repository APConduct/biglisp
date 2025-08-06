use biglisp_core::LispExpr;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn lisp(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as LispExpr);
    let expanded = expr.to_rust();
    expanded.into()
}

// Helper macro for easier testing
#[proc_macro]
pub fn lisp_fn(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as LispExpr);
    let expanded = expr.to_rust();

    quote! {
        {
            #expanded
        }
    }
    .into()
}
