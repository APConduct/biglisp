use biglisp_core::LispExpr;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

/// A procedural macro that allows embedding Lisp-like expressions in Rust code.
///
/// # Syntax
/// - Supports variable capture by parsing a list of variables followed by a Lisp expression.
/// - If variable capture is not used, it falls back to parsing a regular Lisp expression.
///
/// # Example
/// ```rust
/// use biglisp_macros::lisp;
/// let x = 10;
/// let y = 20;
/// lisp!([x, y] (+ x y));
/// lisp!((+ 1 2));
/// ```
#[proc_macro]
pub fn lisp(input: TokenStream) -> TokenStream {
    use syn::{parse::Parse, Ident, Token};

    // Define a structure to parse Lisp expressions with variable capture.
    struct LispWithVars {
        vars: Vec<Ident>, // List of variable identifiers.
        expr: LispExpr,   // The Lisp expression.
    }

    // Implement the `Parse` trait for `LispWithVars` to define how it is parsed.
    impl Parse for LispWithVars {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            // Parse a bracketed list of variables (e.g., [var1, var2, var3]).
            let content;
            syn::bracketed!(content in input);
            let mut vars = Vec::new();

            // Parse identifiers separated by commas.
            while !content.is_empty() {
                vars.push(content.parse::<Ident>()?);
                if !content.is_empty() {
                    content.parse::<Token![,]>()?;
                }
            }

            // Parse the Lisp expression following the variable list.
            let expr: LispExpr = input.parse()?;

            Ok(LispWithVars { vars, expr })
        }
    }

    // Attempt to parse the input as a `LispWithVars` structure.
    if let Ok(parsed) = syn::parse::<LispWithVars>(input.clone()) {
        let vars = &parsed.vars; // Extract the parsed variables.
        let expr_tokens = parsed.expr.to_rust(); // Convert the Lisp expression to Rust code.

        // Generate Rust code that captures the variables and evaluates the expression.
        return quote! {
            {
                // Capture the variables in the current scope.
                #(let #vars = #vars;)*
                #expr_tokens
            }
        }
            .into();
    }

    // If parsing as `LispWithVars` fails, fall back to parsing a regular Lisp expression.
    let expr = parse_macro_input!(input as LispExpr);
    let expanded = expr.to_rust(); // Convert the Lisp expression to Rust code.
    expanded.into()
}

/// A helper procedural macro for testing Lisp expressions.
///
/// # Example
/// ```rust
/// use biglisp_macros::lisp_fn;
/// lisp_fn!((+ 1 2));
/// ```
#[proc_macro]
pub fn lisp_fn(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as LispExpr); // Parse the input as a Lisp expression.
    let expanded = expr.to_rust(); // Convert the Lisp expression to Rust code.

    // Wrap the generated Rust code in a block.
    quote! {
        {
            #expanded
        }
    }
        .into()
}