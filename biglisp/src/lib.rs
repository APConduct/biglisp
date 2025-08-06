pub use biglisp_macros::lisp;
pub mod guts {
    pub use biglisp_core::LispExpr;
    pub use biglisp_macros::lisp_fn;
}
pub mod prelude {
    pub use crate::lisp;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_arithmetic() {
        println!("Testing basic arithmetic");
        let result = lisp!((+ 1 2 3));
        println!("Result: {}", result);
        assert_eq!(result, 6);
    }

    #[test]
    fn simple_nested() {
        // Test just one level of nesting first
        let result = lisp!((+ (+ 1 2) 3));
        assert_eq!(result, 6);
    }

    #[test]
    fn nested_expressions() {
        println!("Testing nested expressions");
        let result = lisp!((* (+ 1 2) (- 5 1)));
        println!("Result: {}", result);
        assert_eq!(result, 12);
    }
}
