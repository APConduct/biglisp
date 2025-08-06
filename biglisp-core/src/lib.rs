use std::fmt::Debug;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    parse::Parse,
    token::{Bracket, Paren},
    Ident, Lit, Token,
};

/// Represents a Lisp expression in the BigLisp language.
///
/// This enum is used to model various types of expressions that can appear
/// in Lisp-like syntax, including symbols, literals, lists, vectors, and operators.
pub enum LispExpr {
    /// A symbol, represented by an identifier.
    Symbol(Ident),

    /// A literal value, such as a number or string.
    Literal(Lit),

    /// A list of Lisp expressions, typically used for function calls or grouping.
    List(Vec<LispExpr>),

    /// A vector of Lisp expressions, used for collections.
    Vector(Vec<LispExpr>),

    /// An operator, represented as a string (e.g., "+", "-", "*").
    Operator(String),
}

impl Debug for LispExpr {
    /// Formats the `LispExpr` enum for debugging purposes.
    ///
    /// This implementation provides a human-readable representation of each variant
    /// of the `LispExpr` enum, including its associated data.
    ///
    /// # Parameters
    /// - `f`: A mutable reference to the formatter used to write the output.
    ///
    /// # Returns
    /// - `std::fmt::Result`: Indicates whether the formatting was successful.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Formats the `Symbol` variant with its identifier.
            LispExpr::Symbol(ident) => write!(f, "Symbol({})", ident),

            // Formats the `Literal` variant with its span information.
            LispExpr::Literal(lit) => write!(f, "Literal({:?})", lit.span()),

            // Formats the `Operator` variant with its operator string.
            LispExpr::Operator(op) => write!(f, "Operator({})", op),

            // Formats the `List` variant by iterating over its elements.
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

            // Formats the `Vector` variant by iterating over its elements.
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
    /// Parses a `LispExpr` from a token stream.
    ///
    /// This implementation supports various Lisp-like syntax constructs, including:
    /// - Parenthesized lists of expressions (e.g., `(expr1 expr2 ...)`).
    /// - Bracketed vectors of expressions (e.g., `[expr1 expr2 ...]`).
    /// - Operators (`+`, `-`, `*`, `/`, `%`, etc.).
    /// - Literals (e.g., numbers, strings).
    /// - Symbols (e.g., `if`, `let`, `do`, etc.).
    ///
    /// # Parameters
    /// - `input`: The token stream to parse.
    ///
    /// # Returns
    /// - `Ok(LispExpr)`: The parsed Lisp expression.
    /// - `Err(syn::Error)`: If the input cannot be parsed as a valid `LispExpr`.
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Paren) {
            // Parse a parenthesized list of expressions.
            let content;
            syn::parenthesized!(content in input);
            let mut exprs = Vec::new();
            while !content.is_empty() {
                exprs.push(content.parse::<LispExpr>()?);
            }
            Ok(LispExpr::List(exprs))
        } else if input.peek(Bracket) {
            // Parse a bracketed vector of expressions.
            let content;
            syn::bracketed!(content in input);
            let mut exprs = Vec::new();
            while !content.is_empty() {
                exprs.push(content.parse::<LispExpr>()?);
            }
            Ok(LispExpr::Vector(exprs))
        } else if input.peek(Token![+]) {
            // Parse the `+` operator.
            input.parse::<Token![+]>()?;
            Ok(LispExpr::Operator("+".to_string()))
        } else if input.peek(Token![-]) {
            // Parse the `-` operator.
            input.parse::<Token![-]>()?;
            Ok(LispExpr::Operator("-".to_string()))
        } else if input.peek(Token![*]) {
            // Parse the `*` operator.
            input.parse::<Token![*]>()?;
            Ok(LispExpr::Operator("*".to_string()))
        } else if input.peek(Token![/]) {
            // Parse the `/` operator.
            input.parse::<Token![/]>()?;
            Ok(LispExpr::Operator("/".to_string()))
        } else if input.peek(Token![=]) {
            // Parse the `=` operator.
            input.parse::<Token![=]>()?;
            Ok(LispExpr::Operator("=".to_string()))
        } else if input.peek(Token![<]) {
            // Parse the `<` operator.
            input.parse::<Token![<]>()?;
            Ok(LispExpr::Operator("<".to_string()))
        } else if input.peek(Token![>]) {
            // Parse the `>` operator.
            input.parse::<Token![>]>()?;
            Ok(LispExpr::Operator(">".to_string()))
        } else if input.peek(Token![%]) {
            // Parse the `%` operator.
            input.parse::<Token![%]>()?;
            Ok(LispExpr::Operator("%".to_string()))
        } else if input.peek(Lit) {
            // Parse a literal value.
            Ok(LispExpr::Literal(input.parse()?))
        } else {
            // Attempt to parse as a symbol or identifier.
            let lookahead = input.lookahead1();
            if lookahead.peek(syn::Token![if]) {
                // Parse the `if` symbol.
                input.parse::<syn::Token![if]>()?;
                Ok(LispExpr::Symbol(Ident::new("if", Span::call_site())))
            } else if lookahead.peek(syn::Token![let]) {
                // Parse the `let` symbol.
                input.parse::<syn::Token![let]>()?;
                Ok(LispExpr::Symbol(Ident::new("let", Span::call_site())))
            } else if lookahead.peek(syn::Token![do]) {
                // Parse the `do` symbol.
                input.parse::<syn::Token![do]>()?;
                Ok(LispExpr::Symbol(Ident::new("do", Span::call_site())))
            } else if lookahead.peek(syn::Token![while]) {
                // Parse the `while` symbol.
                input.parse::<syn::Token![while]>()?;
                Ok(LispExpr::Symbol(Ident::new("while", Span::call_site())))
            } else if lookahead.peek(syn::Token![try]) {
                // Parse the `try` symbol.
                input.parse::<syn::Token![try]>()?;
                Ok(LispExpr::Symbol(Ident::new("try", Span::call_site())))
            } else if lookahead.peek(Ident) {
                // Parse an identifier or special symbol.
                let ident: Ident = input.parse()?;
                let ident_str = ident.to_string();
                if ident_str == "defn"
                    || ident_str == "println"
                    || ident_str == "dotimes"
                    || ident_str == "call"
                    || ident_str == "gte"
                    || ident_str == "lte"
                    || ident_str == "ne"
                    || ident_str == "min"
                    || ident_str == "max"
                    || ident_str == "abs"
                    || ident_str == "modulo"
                    || ident_str == "inc"
                    || ident_str == "dec"
                    || ident_str == "zero"
                    || ident_str == "pos"
                    || ident_str == "neg"
                    || ident_str == "even"
                    || ident_str == "odd"
                {
                    Ok(LispExpr::Symbol(ident))
                } else {
                    Ok(LispExpr::Symbol(ident))
                }
            } else {
                // Return an error if no valid syntax is found.
                Err(lookahead.error())
            }
        }
    }
}
impl LispExpr {
    /// Converts a `LispExpr` into a Rust `TokenStream`.
    ///
    /// This method recursively transforms Lisp expressions into equivalent Rust code
    /// that can be compiled and executed at compile-time. It handles all major
    /// expression types including symbols, literals, operators, vectors, and lists.
    ///
    /// # Returns
    /// - `TokenStream`: The generated Rust code tokens representing the equivalent
    ///   Rust expression for the given Lisp expression.
    ///
    /// # Expression Type Handling
    /// - **Symbols**: Direct identifier conversion (e.g., `x` becomes `x`)
    /// - **Literals**: Direct literal conversion (e.g., `42` becomes `42`)
    /// - **Operators**: Converted to prefixed identifiers (e.g., `+` becomes `op_plus`)
    /// - **Vectors**: Converted to Rust `vec!` macros with element expansion
    /// - **Lists**: Processed as function calls or special operations
    ///
    /// # List Processing
    /// - Empty lists return the unit type `()`
    /// - Lists with symbols or operators as the first element are expanded using
    ///   `expand_operation()` to handle special forms like arithmetic, control flow, etc.
    /// - Other lists are treated as function calls with the first element as the
    ///   function and remaining elements as arguments
    ///
    /// # Example Transformations
    /// ```text
    /// Symbol: x                -> x
    /// Literal: 42              -> 42
    /// Vector: [1, 2, 3]        -> vec![1, 2, 3]
    /// List: (+ 1 2)            -> Expanded arithmetic operation
    /// List: (func arg1 arg2)   -> func(arg1, arg2)
    /// ```
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
                            .replace(">=", "gte")
                            .replace("<=", "lte")
                            .replace("!=", "ne")
                            .replace("%", "mod")
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

    /// Expands BigLisp operations into equivalent Rust code.
    ///
    /// This method is the core of the BigLisp DSL, transforming Lisp-style function calls
    /// and special forms into native Rust expressions. It handles all built-in operations
    /// including arithmetic, comparisons, control flow, data structures, and utility functions.
    ///
    /// # Parameters
    /// - `op_str`: The operation name as a string (e.g., "+", "if", "let", "defn")
    /// - `args`: A slice of `LispExpr` arguments to the operation
    ///
    /// # Returns
    /// - `TokenStream`: Generated Rust code that implements the operation
    ///
    /// # Supported Operations
    ///
    /// ## Arithmetic Operations
    /// - `+`: Addition with identity element 0, supports single argument
    /// - `-`: Subtraction and unary negation
    /// - `*`: Multiplication with identity element 1, supports single argument
    /// - `/`: Division (requires at least 2 arguments)
    /// - `%`/`modulo`: Modulo operation
    ///
    /// ## Comparison Operations
    /// - `=`/`eq`: Equality comparison
    /// - `<`, `>`: Less than, greater than
    /// - `gte`, `lte`: Greater/less than or equal
    /// - `ne`: Not equal
    ///
    /// ## Boolean Operations
    /// - `and`: Logical AND (requires at least 2 arguments)
    /// - `or`: Logical OR (requires at least 2 arguments)
    /// - `not`: Logical NOT (exactly 1 argument)
    ///
    /// ## Control Flow
    /// - `if`: Conditional with optional else branch
    /// - `let`: Local variable bindings with vector syntax
    /// - `do`: Sequential execution block
    /// - `while`: While loop with condition and body
    /// - `dotimes`: For-like loop with variable, count, and body
    ///
    /// ## Function Operations
    /// - `defn`: Function definition creating closures
    /// - `call`: Function invocation
    ///
    /// ## Data Structure Operations
    /// - `first`: Get first element of collection
    /// - `rest`: Get all but first element
    /// - `cons`: Prepend element to collection
    /// - `count`: Get collection length
    ///
    /// ## String Operations
    /// - `str`: String concatenation of multiple arguments
    ///
    /// ## Math Utility Functions
    /// - `min`, `max`: Minimum/maximum of multiple values
    /// - `abs`: Absolute value
    /// - `inc`, `dec`: Increment/decrement by 1
    ///
    /// ## Predicate Functions
    /// - `zero`: Test if value equals zero
    /// - `pos`, `neg`: Test if value is positive/negative
    /// - `even`, `odd`: Test if value is even/odd
    ///
    /// ## Error Handling
    /// - `try`: Panic-safe execution with optional fallback
    ///
    /// ## Variable Capture
    /// - `with-vars`: Capture external variables in scope
    ///
    /// ## Debug Operations
    /// - `println`: Debug printing
    ///
    /// # Examples
    /// ```ignore
    /// // Arithmetic: (+ 1 2 3) -> 0 + 1 + 2 + 3
    /// // Control flow: (if (> x 0) "pos" "neg") -> if x > 0 { "pos" } else { "neg" }
    /// // Functions: (defn sq [x] (* x x)) -> closure |x: i32| -> i32 { x * x }
    /// ```
    ///
    /// # Error Handling
    /// Invalid argument counts or malformed expressions generate compile-time errors
    /// using `compile_error!` macro, ensuring type safety and preventing runtime errors.
    fn expand_operation(&self, op_str: &str, args: &[LispExpr]) -> TokenStream {
        match op_str {
            // Arithmetic Operations
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
            "gte" => {
                if args.len() == 2 {
                    let left = args[0].to_rust();
                    let right = args[1].to_rust();
                    quote! { (#left) >= (#right) }
                } else {
                    quote! { compile_error!("Greater-than-or-equal requires exactly 2 arguments") }
                }
            }
            "lte" => {
                if args.len() == 2 {
                    let left = args[0].to_rust();
                    let right = args[1].to_rust();
                    quote! { (#left) <= (#right) }
                } else {
                    quote! { compile_error!("Less-than-or-equal requires exactly 2 arguments") }
                }
            }
            "ne" => {
                if args.len() == 2 {
                    let left = args[0].to_rust();
                    let right = args[1].to_rust();
                    quote! { (#left) != (#right) }
                } else {
                    quote! { compile_error!("Not-equal requires exactly 2 arguments") }
                }
            }
            "%" | "modulo" => {
                if args.len() == 2 {
                    let left = args[0].to_rust();
                    let right = args[1].to_rust();
                    quote! { (#left) % (#right) }
                } else {
                    quote! { compile_error!("Modulo requires exactly 2 arguments") }
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

            // Function definition - now creates a closure that can be called
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
                            {
                                let #name = |#(#param_names: i32),*| -> i32 {
                                    #body_tokens
                                };
                                #name
                            }
                        }
                    } else {
                        quote! { compile_error!("Function definition format: (defn name [params] body)") }
                    }
                } else {
                    quote! { compile_error!("Function definition requires name, params, and body") }
                }
            }

            // Function call
            "call" => {
                if args.len() >= 1 {
                    let func = args[0].to_rust();
                    let func_args = args[1..].iter().map(|e| e.to_rust());
                    quote! { (#func)(#(#func_args),*) }
                } else {
                    quote! { compile_error!("call requires at least a function") }
                }
            }

            // Error handling - try/catch equivalent
            "try" => {
                if args.len() >= 1 {
                    let try_body = args[0].to_rust();
                    if args.len() >= 2 {
                        let catch_body = args[1].to_rust();
                        quote! {
                            {
                                let result = std::panic::catch_unwind(|| {
                                    #try_body
                                });
                                match result {
                                    Ok(val) => val,
                                    Err(_) => #catch_body,
                                }
                            }
                        }
                    } else {
                        quote! {
                            {
                                let result = std::panic::catch_unwind(|| {
                                    #try_body
                                });
                                match result {
                                    Ok(val) => val,
                                    Err(_) => panic!("Unhandled error in try block"),
                                }
                            }
                        }
                    }
                } else {
                    quote! { compile_error!("try requires at least a body") }
                }
            }
            // Block/do
            "do" => {
                let statements = args.iter().map(|e| e.to_rust());
                quote! { { #(#statements);* } }
            }

            // Variable capture - with-vars syntax
            "with-vars" => {
                if args.len() >= 2 {
                    if let LispExpr::Vector(vars) = &args[0] {
                        let body = &args[1];
                        let var_captures: Vec<_> = vars
                            .iter()
                            .filter_map(|v| {
                                if let LispExpr::Symbol(name) = v {
                                    Some(name)
                                } else {
                                    None
                                }
                            })
                            .collect();

                        let body_tokens = body.to_rust();
                        quote! {
                            {
                                #(let #var_captures = #var_captures;)*
                                #body_tokens
                            }
                        }
                    } else {
                        quote! { compile_error!("with-vars requires vector of variable names") }
                    }
                } else {
                    quote! { compile_error!("with-vars requires variables and body") }
                }
            }

            // While loop
            "while" => {
                if args.len() == 2 {
                    let condition = args[0].to_rust();
                    let body = args[1].to_rust();
                    quote! {
                        {
                            let mut result = ();
                            while (#condition) {
                                result = #body;
                            }
                            result
                        }
                    }
                } else {
                    quote! { compile_error!("While requires condition and body") }
                }
            }

            // For-like loop (dotimes)
            "dotimes" => {
                if args.len() == 3 {
                    if let LispExpr::Symbol(var) = &args[0] {
                        let count = args[1].to_rust();
                        let body = args[2].to_rust();
                        quote! {
                            {
                                for #var in 0..(#count) {
                                    let _ = #body;
                                }
                                ()
                            }
                        }
                    } else {
                        quote! { compile_error!("dotimes requires variable name") }
                    }
                } else {
                    quote! { compile_error!("dotimes requires var, count, and body") }
                }
            }

            // Boolean operations
            "and" => {
                if args.len() >= 2 {
                    let terms = args.iter().map(|e| e.to_rust());
                    let mut result = quote! { true };
                    for term in terms {
                        result = quote! { (#result) && (#term) };
                    }
                    result
                } else {
                    quote! { compile_error!("And requires at least 2 arguments") }
                }
            }
            "or" => {
                if args.len() >= 2 {
                    let terms = args.iter().map(|e| e.to_rust());
                    let mut result = quote! { false };
                    for term in terms {
                        result = quote! { (#result) || (#term) };
                    }
                    result
                } else {
                    quote! { compile_error!("Or requires at least 2 arguments") }
                }
            }
            "not" => {
                if args.len() == 1 {
                    let arg = args[0].to_rust();
                    quote! { !(#arg) }
                } else {
                    quote! { compile_error!("Not requires exactly 1 argument") }
                }
            }

            // List/Vector operations
            "first" => {
                if args.len() == 1 {
                    let arg = args[0].to_rust();
                    quote! { (#arg).first().copied().unwrap_or_default() }
                } else {
                    quote! { compile_error!("First requires exactly 1 argument") }
                }
            }
            "rest" => {
                if args.len() == 1 {
                    let arg = args[0].to_rust();
                    quote! { { let v = #arg; if v.len() > 1 { v[1..].to_vec() } else { vec![] } } }
                } else {
                    quote! { compile_error!("Rest requires exactly 1 argument") }
                }
            }
            "cons" => {
                if args.len() == 2 {
                    let elem = args[0].to_rust();
                    let list = args[1].to_rust();
                    quote! { { let mut result = vec![(#elem)]; result.extend(#list); result } }
                } else {
                    quote! { compile_error!("Cons requires exactly 2 arguments") }
                }
            }
            "count" => {
                if args.len() == 1 {
                    let arg = args[0].to_rust();
                    quote! { (#arg).len() }
                } else {
                    quote! { compile_error!("Count requires exactly 1 argument") }
                }
            }

            // String operations
            "str" => {
                if args.len() >= 1 {
                    let string_parts = args.iter().map(|e| {
                        let arg = e.to_rust();
                        quote! { (#arg).to_string() }
                    });
                    quote! { [#(#string_parts),*].join("") }
                } else {
                    quote! { String::new() }
                }
            }

            // Math utility functions
            "min" => {
                if args.len() >= 2 {
                    let first = args[0].to_rust();
                    let rest = args[1..].iter().map(|e| e.to_rust());
                    let mut result = quote! { (#first) };
                    for term in rest {
                        result = quote! { std::cmp::min(#result, #term) };
                    }
                    result
                } else {
                    quote! { compile_error!("min requires at least 2 arguments") }
                }
            }
            "max" => {
                if args.len() >= 2 {
                    let first = args[0].to_rust();
                    let rest = args[1..].iter().map(|e| e.to_rust());
                    let mut result = quote! { (#first) };
                    for term in rest {
                        result = quote! { std::cmp::max(#result, #term) };
                    }
                    result
                } else {
                    quote! { compile_error!("max requires at least 2 arguments") }
                }
            }
            "abs" => {
                if args.len() == 1 {
                    let arg = args[0].to_rust();
                    quote! { ((#arg) as i32).abs() }
                } else {
                    quote! { compile_error!("abs requires exactly 1 argument") }
                }
            }

            // Additional utility functions
            "inc" => {
                if args.len() == 1 {
                    let arg = args[0].to_rust();
                    quote! { (#arg) + 1 }
                } else {
                    quote! { compile_error!("inc requires exactly 1 argument") }
                }
            }
            "dec" => {
                if args.len() == 1 {
                    let arg = args[0].to_rust();
                    quote! { (#arg) - 1 }
                } else {
                    quote! { compile_error!("dec requires exactly 1 argument") }
                }
            }
            "zero" => {
                if args.len() == 1 {
                    let arg = args[0].to_rust();
                    quote! { (#arg) == 0 }
                } else {
                    quote! { compile_error!("zero requires exactly 1 argument") }
                }
            }
            "pos" => {
                if args.len() == 1 {
                    let arg = args[0].to_rust();
                    quote! { (#arg) > 0 }
                } else {
                    quote! { compile_error!("pos requires exactly 1 argument") }
                }
            }
            "neg" => {
                if args.len() == 1 {
                    let arg = args[0].to_rust();
                    quote! { (#arg) < 0 }
                } else {
                    quote! { compile_error!("neg requires exactly 1 argument") }
                }
            }
            "even" => {
                if args.len() == 1 {
                    let arg = args[0].to_rust();
                    quote! { (#arg) % 2 == 0 }
                } else {
                    quote! { compile_error!("even requires exactly 1 argument") }
                }
            }
            "odd" => {
                if args.len() == 1 {
                    let arg = args[0].to_rust();
                    quote! { (#arg) % 2 != 0 }
                } else {
                    quote! { compile_error!("odd requires exactly 1 argument") }
                }
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
