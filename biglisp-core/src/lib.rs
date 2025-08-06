use std::fmt::Debug;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    parse::Parse,
    token::{Bracket, Paren},
    Ident, Lit, Token,
};

pub fn hello() {
    println!("Hello, world!");
}
pub enum LispExpr {
    Symbol(Ident),
    Literal(Lit),
    List(Vec<LispExpr>),
    Vector(Vec<LispExpr>),
    Operator(String),
}

impl Debug for LispExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LispExpr::Symbol(ident) => write!(f, "Symbol({})", ident),
            LispExpr::Literal(lit) => write!(f, "Literal({:?})", lit.span()),
            LispExpr::Operator(op) => write!(f, "Operator({})", op),
            LispExpr::List(exprs) => {
                write!(f, "List(")?;
                for (i, expr) in exprs.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{:?}", expr)?;
                }
                write!(f, ")")
            }
            LispExpr::Vector(exprs) => {
                write!(f, "Vector(")?;
                for (i, expr) in exprs.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{:?}", expr)?;
                }
                write!(f, ")")
            }
        }
    }
}

impl Parse for LispExpr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Paren) {
            let content;
            syn::parenthesized!(content in input);
            let mut exprs = Vec::new();
            while !content.is_empty() {
                exprs.push(content.parse::<LispExpr>()?);
            }
            Ok(LispExpr::List(exprs))
        } else if input.peek(Bracket) {
            let content;
            syn::bracketed!(content in input);
            let mut exprs = Vec::new();
            while !content.is_empty() {
                exprs.push(content.parse::<LispExpr>()?);
            }
            Ok(LispExpr::Vector(exprs))
        } else if input.peek(Token![+]) {
            input.parse::<Token![+]>()?;
            Ok(LispExpr::Operator("+".to_string()))
        } else if input.peek(Token![-]) {
            input.parse::<Token![-]>()?;
            Ok(LispExpr::Operator("-".to_string()))
        } else if input.peek(Token![*]) {
            input.parse::<Token![*]>()?;
            Ok(LispExpr::Operator("*".to_string()))
        } else if input.peek(Token![/]) {
            input.parse::<Token![/]>()?;
            Ok(LispExpr::Operator("/".to_string()))
        } else if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            Ok(LispExpr::Operator("=".to_string()))
        } else if input.peek(Token![<]) {
            input.parse::<Token![<]>()?;
            Ok(LispExpr::Operator("<".to_string()))
        } else if input.peek(Token![>]) {
            input.parse::<Token![>]>()?;
            Ok(LispExpr::Operator(">".to_string()))
        } else if input.peek(Lit) {
            Ok(LispExpr::Literal(input.parse()?))
        } else {
            Ok(LispExpr::Symbol(input.parse()?))
        }
    }
}

impl LispExpr {
    pub fn to_rust(&self) -> TokenStream {
        match self {
            LispExpr::Symbol(ident) => {
                quote::quote! { #ident }
            }
            LispExpr::Literal(lit) => {
                quote::quote! { #lit }
            }
            LispExpr::Operator(op) => {
                let ident = Ident::new(
                    &format!(
                        "op_{}",
                        op.replace("+", "plus")
                            .replace("-", "minus")
                            .replace("*", "mul")
                            .replace("/", "div")
                            .replace("=", "eq")
                            .replace("<", "lt")
                            .replace(">", "gt")
                    ),
                    Span::call_site(),
                );
                quote::quote! { #ident }
            }
            LispExpr::Vector(exprs) => {
                let elements = exprs.iter().map(|e| e.to_rust());
                quote::quote! { vec![#(#elements),*] }
            }
            LispExpr::List(exprs) => {
                if exprs.is_empty() {
                    return quote::quote! { () };
                }
                let first = &exprs[0];
                let rest = &exprs[1..];

                match first {
                    LispExpr::Symbol(op) => self.expand_operation(&op.to_string(), rest),
                    LispExpr::Operator(op) => self.expand_operation(op, rest),
                    _ => {
                        let funct = first.to_rust();
                        let args = rest.iter().map(|e| e.to_rust());
                        quote::quote! {
                            #funct(#(#args),*)
                        }
                    }
                }
            }
        }
    }

    fn expand_operation(&self, op_str: &str, args: &[LispExpr]) -> TokenStream {
        match op_str {
            "+" => {
                if args.is_empty() {
                    quote! { 0 }
                } else if args.len() == 1 {
                    args[0].to_rust()
                } else {
                    let terms = args.iter().map(|e| e.to_rust());
                    let mut result = quote! { 0 };
                    for term in terms {
                        result = quote! { #result + (#term) };
                    }
                    result
                }
            }
            "-" => {
                if args.len() == 1 {
                    let arg = args[0].to_rust();
                    quote! { -(#arg) }
                } else if args.len() >= 2 {
                    let first = args[0].to_rust();
                    let rest = args[1..].iter().map(|e| e.to_rust());
                    let mut result = quote! { (#first) };
                    for term in rest {
                        result = quote! { #result - (#term) };
                    }
                    result
                } else {
                    quote! { compile_error!("Subtraction requires at least 1 argument") }
                }
            }
            "*" => {
                if args.is_empty() {
                    quote! { 1 }
                } else if args.len() == 1 {
                    args[0].to_rust()
                } else {
                    let terms = args.iter().map(|e| e.to_rust());
                    let mut result = quote! { 1 };
                    for term in terms {
                        result = quote! { #result * (#term) };
                    }
                    result
                }
            }
            "/" => {
                if args.len() >= 2 {
                    let first = args[0].to_rust();
                    let rest = args[1..].iter().map(|e| e.to_rust());
                    let mut result = quote! { (#first) };
                    for term in rest {
                        result = quote! { #result / (#term) };
                    }
                    result
                } else {
                    quote! { compile_error!("Division requires at least 2 arguments") }
                }
            }
            // Comparison operators
            "=" | "eq" => {
                if args.len() == 2 {
                    let left = args[0].to_rust();
                    let right = args[1].to_rust();
                    quote! { (#left) == (#right) }
                } else {
                    quote! { compile_error!("Equality requires exactly 2 arguments") }
                }
            }
            "<" => {
                if args.len() == 2 {
                    let left = args[0].to_rust();
                    let right = args[1].to_rust();
                    quote! { (#left) < (#right) }
                } else {
                    quote! { compile_error!("Less-than requires exactly 2 arguments") }
                }
            }
            ">" => {
                if args.len() == 2 {
                    let left = args[0].to_rust();
                    let right = args[1].to_rust();
                    quote! { (#left) > (#right) }
                } else {
                    quote! { compile_error!("Greater-than requires exactly 2 arguments") }
                }
            }
            // Control flow
            "if" => match args.len() {
                2 => {
                    let cond = args[0].to_rust();
                    let then_branch = args[1].to_rust();
                    quote! { if (#cond) { #then_branch } }
                }
                3 => {
                    let cond = args[0].to_rust();
                    let then_branch = args[1].to_rust();
                    let else_branch = args[2].to_rust();
                    quote! { if (#cond) { #then_branch } else { #else_branch } }
                }
                _ => quote! { compile_error!("If requires 2 or 3 arguments") },
            },
            // Let bindings
            "let" => {
                if args.len() >= 2 {
                    if let LispExpr::Vector(bindings) = &args[0] {
                        let body = args[1].to_rust();
                        let mut lets = TokenStream::new();

                        for binding in bindings.chunks(2) {
                            if binding.len() == 2 {
                                if let (LispExpr::Symbol(name), value) = (&binding[0], &binding[1])
                                {
                                    let value_tokens = value.to_rust();
                                    lets.extend(quote! { let #name = #value_tokens; });
                                }
                            }
                        }

                        quote! { { #lets #body } }
                    } else {
                        quote! { compile_error!("Let requires vector of bindings") }
                    }
                } else {
                    quote! { compile_error!("Let requires bindings and body") }
                }
            }

            // Function definition
            "defn" => {
                if args.len() >= 3 {
                    if let (LispExpr::Symbol(name), LispExpr::Vector(params), body) =
                        (&args[0], &args[1], &args[2])
                    {
                        let param_names: Vec<_> = params
                            .iter()
                            .filter_map(|p| {
                                if let LispExpr::Symbol(s) = p {
                                    Some(s)
                                } else {
                                    None
                                }
                            })
                            .collect();
                        let body_tokens = body.to_rust();

                        quote! {
                            fn #name(#(#param_names: i32),*) -> i32 {
                                #body_tokens
                            }
                        }
                    } else {
                        quote! { compile_error!("Function definition format: (defn name [params] body)") }
                    }
                } else {
                    quote! { compile_error!("Function definition requires name, params, and body") }
                }
            }
            // Block/do
            "do" => {
                let statements = args.iter().map(|e| e.to_rust());
                quote! { { #(#statements);* } }
            }

            // Print/debug
            "println" => {
                if args.len() == 1 {
                    let arg = args[0].to_rust();
                    quote! { println!("{:?}", #arg) }
                } else {
                    let args_tokens = args.iter().map(|e| e.to_rust());
                    quote! { println!("{:?}", (#(#args_tokens),*)) }
                }
            }
            // Default: treat as function call
            _ => {
                let op_ident = Ident::new(op_str, Span::call_site());
                let args_tokens = args.iter().map(|e| e.to_rust());
                quote! { #op_ident(#(#args_tokens),*) }
            }
        }
    }
}
