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

    #[test]
    fn complex_expression() {
        println!("Testing complex expression");
        let result = lisp!((/ (* (+ 10 5) (- 20 5)) 5));
        println!("Result: {}", result);
        assert_eq!(result, 45);
    }

    #[test]
    fn single_numbers() {
        // Test single argument operations
        let result_add = lisp!((+ 42));
        assert_eq!(result_add, 42);

        let result_mult = lisp!((* 7));
        assert_eq!(result_mult, 7);

        let result_neg = lisp!((-10));
        assert_eq!(result_neg, -10);
    }

    #[test]
    fn zero_and_identity() {
        // Test with zeros and identity elements
        let result_add_zero = lisp!((+ 0 5 0 3));
        assert_eq!(result_add_zero, 8);

        let result_mult_one = lisp!((* 1 7 1 3));
        assert_eq!(result_mult_one, 21);

        let result_sub_zero = lisp!((- 10 0));
        assert_eq!(result_sub_zero, 10);
    }

    #[test]
    fn large_expressions() {
        // Test expressions with many operands
        let result_many_adds = lisp!((+ 1 2 3 4 5 6 7 8 9 10));
        assert_eq!(result_many_adds, 55);

        let result_many_mults = lisp!((* 2 2 2 2 2));
        assert_eq!(result_many_mults, 32);

        let result_many_subs = lisp!((- 100 10 5 3 2));
        assert_eq!(result_many_subs, 80);
    }

    #[test]
    fn comparison_operations() {
        // Test equality
        let result_eq_true = lisp!((= 5 5));
        assert_eq!(result_eq_true, true);

        let result_eq_false = lisp!((= 3 7));
        assert_eq!(result_eq_false, false);

        // Test less than
        let result_lt_true = lisp!((< 3 7));
        assert_eq!(result_lt_true, true);

        let result_lt_false = lisp!((< 7 3));
        assert_eq!(result_lt_false, false);

        // Test greater than
        let result_gt_true = lisp!((> 7 3));
        assert_eq!(result_gt_true, true);

        let result_gt_false = lisp!((> 3 7));
        assert_eq!(result_gt_false, false);
    }

    #[test]
    fn nested_comparisons() {
        // Test comparisons with nested arithmetic
        let result_nested_eq = lisp!((= (+ 2 3) (* 1 5)));
        assert_eq!(result_nested_eq, true);

        let result_nested_lt = lisp!((< (+ 1 2) (* 2 3)));
        assert_eq!(result_nested_lt, true);

        let result_complex_comp = lisp!((> (+ (* 2 3) 1) (- 10 3)));
        assert_eq!(result_complex_comp, false); // 7 > 7 is false
    }

    #[test]
    fn deeply_nested() {
        // Test very deeply nested expressions
        let result = lisp!((+ (+ (+ 1 2) (+ 3 4)) (+ (+ 5 6) (+ 7 8))));
        assert_eq!(result, 36); // ((3 + 7) + (11 + 15)) = 10 + 26 = 36

        let result2 = lisp!((* (+ 1 (+ 2 3)) (- 10 (- 8 2))));
        assert_eq!(result2, 24); // (1 + 5) * (10 - 6) = 6 * 4 = 24
    }

    #[test]
    fn edge_case_division() {
        // Test integer division behavior
        let result_exact = lisp!((/ 15 3));
        assert_eq!(result_exact, 5);

        let result_truncated = lisp!((/ 7 2));
        assert_eq!(result_truncated, 3); // Integer division truncates

        let result_chained = lisp!((/ 60 3 2));
        assert_eq!(result_chained, 10); // 60 / 3 / 2 = 20 / 2 = 10
    }

    #[test]
    fn unary_negation() {
        // Test unary negation (single argument to minus)
        let result_neg = lisp!((-5));
        assert_eq!(result_neg, -5);

        let result_neg_zero = lisp!((-0));
        assert_eq!(result_neg_zero, 0);

        let result_neg_nested = lisp!((- (+ 3 4)));
        assert_eq!(result_neg_nested, -7);

        let result_with_negation = lisp!((+ (- 5) 10));
        assert_eq!(result_with_negation, 5); // -5 + 10 = 5
    }

    // Note: For complex macro calls that formatters keep breaking, you can use:
    //
    // Example alternative approaches:
    // 1. Break into multiple lines:
    //    let result = lisp!((
    //        * (+ 1 2) (- 5 1)
    //    ));
    //
    // 2. Use variables for complex nested expressions:
    //    let add_expr = lisp!((+ 1 2));
    //    let sub_expr = lisp!((- 5 1));
    //    let result = add_expr * sub_expr;
    //
    // 3. Configure .rustfmt.toml with format_macro_matchers = false
}
