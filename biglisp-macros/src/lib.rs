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

// Macro that allows capturing variables from outer scope
#[proc_macro]
pub fn lisp_with_vars(input: TokenStream) -> TokenStream {
    use syn::{parse::Parse, parse_macro_input, Ident};

    struct LispWithVars {
        vars: Vec<Ident>,
        expr: LispExpr,
    }

    impl Parse for LispWithVars {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            // Parse [var1, var2, var3]
            let content;
            syn::bracketed!(content in input);
            let mut vars = Vec::new();
            while !content.is_empty() {
                vars.push(content.parse::<Ident>()?);
                if !content.is_empty() {
                    content.parse::<syn::Token![,]>()?;
                }
            }

            // Parse the lisp expression
            let expr: LispExpr = input.parse()?;

            Ok(LispWithVars { vars, expr })
        }
    }

    let input = parse_macro_input!(input as LispWithVars);
    let vars = &input.vars;
    let expr_tokens = input.expr.to_rust();

    quote! {
        {
            // Capture the variables in the current scope
            #(let #vars = #vars;)*
            #expr_tokens
        }
    }
    .into()
}
