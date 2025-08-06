use biglisp_core::LispExpr;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn lisp(input: TokenStream) -> TokenStream {
    use syn::{parse::Parse, Ident, Token};

    // Define variable capture parser
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

            // Parse identifiers separated by commas
            while !content.is_empty() {
                vars.push(content.parse::<Ident>()?);
                if !content.is_empty() {
                    content.parse::<Token![,]>()?;
                }
            }

            // Parse the lisp expression
            let expr: LispExpr = input.parse()?;

            Ok(LispWithVars { vars, expr })
        }
    }

    // Try to parse as variable capture first
    if let Ok(parsed) = syn::parse::<LispWithVars>(input.clone()) {
        let vars = &parsed.vars;
        let expr_tokens = parsed.expr.to_rust();

        return quote! {
            {
                // Capture the variables in the current scope
                #(let #vars = #vars;)*
                #expr_tokens
            }
        }
        .into();
    }

    // Fall back to regular lisp expression
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

// Legacy macro for backwards compatibility - now just calls lisp!
#[proc_macro]
pub fn lisp_with_vars(input: TokenStream) -> TokenStream {
    // Simply delegate to the unified lisp! macro
    lisp(input)
}
