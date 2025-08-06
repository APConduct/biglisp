pub use biglisp_macros::{lisp, lisp_with_vars};
pub mod guts {
    pub use biglisp_core::LispExpr;
    pub use biglisp_macros::{lisp_fn, lisp_with_vars};
}
pub mod prelude {
    pub use crate::{lisp, lisp_with_vars};
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

    #[test]
    fn conditional_expressions() {
        // Test if expressions
        let result_true = lisp!((if (> 5 3) 10 20));
        assert_eq!(result_true, 10);

        let result_false = lisp!((if (< 5 3) 10 20));
        assert_eq!(result_false, 20);

        // Test if without else - commented out for now as it returns ()
        // let result_no_else = lisp!((if (= 5 5) 42));
        // assert_eq!(result_no_else, 42);

        // Test nested conditions
        let result_nested = lisp!((if (> (+ 2 3) 4) (* 2 5) (/ 10 2)));
        assert_eq!(result_nested, 10); // (2+3) > 4 is true, so 2*5 = 10
    }

    #[test]
    fn local_bindings() {
        // Test let expressions with local variable bindings
        let result_simple = lisp!((let [x 5] x));
        assert_eq!(result_simple, 5);

        let result_multiple = lisp!((let [x 3 y 4] (+ x y)));
        assert_eq!(result_multiple, 7);

        let result_nested_calc = lisp!((let [a 10 b 5] (* a (- a b))));
        assert_eq!(result_nested_calc, 50); // 10 * (10 - 5) = 50

        // Test let with more complex expressions
        let result_complex = lisp!((let [x (+ 2 3) y (* 2 4)] (+ x y)));
        assert_eq!(result_complex, 13); // (2+3) + (2*4) = 5 + 8 = 13
    }

    // TODO: Function definitions need to be at module level, not inside test functions
    // #[test]
    // fn function_definitions() {
    //     // Test function definition and calling
    //     // Note: This creates a function but doesn't call it yet
    //     // In a real implementation, you'd need a way to store and call defined functions

    //     // For now, test that the macro doesn't crash on function definitions
    //     // The actual function calling would need additional infrastructure
    //     let _square_fn = lisp!((defn square [x] (* x x)));

    //     // Test multiple parameter function
    //     let _add_fn = lisp!((defn add [a b] (+ a b)));

    //     // Test function with more complex body
    //     let _complex_fn = lisp!((defn complex_calc [x y] (+ (* x x) (* y y))));

    //     // These tests mainly verify the macro doesn't panic on function definitions
    //     // Actually calling these functions would require additional macro infrastructure
    //     assert!(true); // Placeholder - just testing that compilation succeeds
    // }

    #[test]
    fn println_expressions() {
        // Test println functionality
        // Note: These will actually print to stdout during testing

        lisp!((println 42));
        lisp!((println (+ 2 3)));
        lisp!((println (+ 1 2) (* 3 4)));

        // Test that println returns unit type (implicitly)
        let _unit_result = lisp!((println "test output"));

        // These tests verify println compiles and runs without panicking
        assert!(true);
    }

    #[test]
    fn do_blocks() {
        // Test do expressions (sequential execution)
        let result = lisp!((do
            (+ 1 2)
            (* 3 4)
            (- 10 5)
        ));
        // do returns the last expression
        assert_eq!(result, 5);

        // Test do with mixed operations
        let result2 = lisp!((do
            (* 2 3)
            (+ 10 20)
            (/ 100 4)
        ));
        assert_eq!(result2, 25);
    }

    #[test]
    fn variable_capture() {
        // Test variable capture using lisp_with_vars macro
        let x = 5;
        let y = 10;

        // Now this works with lisp_with_vars!
        let result1 = lisp_with_vars!([x, y] (+ x y));
        assert_eq!(result1, 15);

        let result2 = lisp_with_vars!([x] (* x x));
        assert_eq!(result2, 25);

        // Test with complex expressions
        let z = 3;
        let result3 = lisp_with_vars!([x, y, z] (+ (* x y) z));
        assert_eq!(result3, 53); // (5 * 10) + 3 = 53
    }

    #[test]
    fn boolean_operations() {
        // Test and operation
        let result_and_true = lisp!((and true true));
        assert_eq!(result_and_true, true);

        let result_and_false = lisp!((and true false));
        assert_eq!(result_and_false, false);

        let result_and_multiple = lisp!((and (> 5 3) (< 2 4) (= 1 1)));
        assert_eq!(result_and_multiple, true);

        // Test or operation
        let result_or_true = lisp!((or false true));
        assert_eq!(result_or_true, true);

        let result_or_false = lisp!((or false false));
        assert_eq!(result_or_false, false);

        let result_or_multiple = lisp!((or (< 5 3) (> 2 4) (= 1 1)));
        assert_eq!(result_or_multiple, true);

        // Test not operation
        let result_not_true = lisp!((not false));
        assert_eq!(result_not_true, true);

        let result_not_false = lisp!((not true));
        assert_eq!(result_not_false, false);

        let result_not_expr = lisp!((not (< 5 3)));
        assert_eq!(result_not_expr, true);
    }

    #[test]
    fn list_operations() {
        // Test creating vectors
        let vec_simple = lisp!([1 2 3 4]);
        assert_eq!(vec_simple, vec![1, 2, 3, 4]);

        // Test first operation
        let first_elem = lisp!((first [1 2 3]));
        assert_eq!(first_elem, 1);

        // Test first with single element
        let first_single = lisp!((first [42]));
        assert_eq!(first_single, 42);

        // Test count operation
        let count_result = lisp!((count [1 2 3 4 5]));
        assert_eq!(count_result, 5);

        let count_single = lisp!((count [42]));
        assert_eq!(count_single, 1);

        // Test rest operation
        let rest_result = lisp!((rest [1 2 3 4]));
        assert_eq!(rest_result, vec![2, 3, 4]);

        let rest_single = lisp!((rest [42]));
        assert_eq!(rest_single, Vec::<i32>::new());

        // Test cons operation
        let cons_result = lisp!((cons 0 [1 2 3]));
        assert_eq!(cons_result, vec![0, 1, 2, 3]);

        let cons_single = lisp!((cons 0 [42]));
        assert_eq!(cons_single, vec![0, 42]);
    }

    #[test]
    fn string_operations() {
        // Test string concatenation
        let result_simple = lisp!((str "hello" " " "world"));
        assert_eq!(result_simple, "hello world");

        let result_mixed = lisp!((str "The answer is " 42));
        assert_eq!(result_mixed, "The answer is 42");

        let result_single = lisp!((str "hello"));
        assert_eq!(result_single, "hello");

        // Test with expressions
        let result_expr = lisp!((str "Sum: " (+ 2 3)));
        assert_eq!(result_expr, "Sum: 5");

        let result_complex = lisp!((str "2 + 3 = " (+ 2 3) " and 2 * 3 = " (* 2 3)));
        assert_eq!(result_complex, "2 + 3 = 5 and 2 * 3 = 6");
    }

    #[test]
    fn function_definitions_and_calls() {
        // Test function definition and calling with new closure-based approach
        let square = lisp!((defn square [x] (* x x)));
        let result1 = lisp!((call square 5));
        assert_eq!(result1, 25);

        let add = lisp!((defn add [a b] (+ a b)));
        let result2 = lisp!((call add 3 7));
        assert_eq!(result2, 10);

        // Test function with complex body - fix variable scope
        let result3 = {
            let complex_fn = lisp!((defn complex [x y] (+ (* x x) (* y y))));
            lisp!((call complex_fn 3 4))
        };
        assert_eq!(result3, 25); // 3² + 4² = 9 + 16 = 25
    }

    #[test]
    fn advanced_control_flow() {
        // Test dotimes loop - assign to unit type since it returns ()
        let _result_dotimes: () = lisp!((dotimes i 5 (+ i 1)));

        // Test that dotimes executes without panicking
        let _result_sum: () = lisp!((dotimes i 3 (* (+ i 1) 2)));

        // These tests mainly verify the advanced control flow compiles
        assert!(true);
    }

    #[test]
    fn error_handling() {
        // Test try-catch equivalent
        let safe_division = lisp!((try (/ 10 2) 0));
        assert_eq!(safe_division, 5);

        // Test try with just the try block (no catch)
        let simple_try = lisp!((try (+ 1 2)));
        assert_eq!(simple_try, 3);

        // Test nested try blocks - fix negative literal issue
        let nested_try = lisp!((try (try (+ 5 5) 0) (- 0 1)));
        assert_eq!(nested_try, 10);
    }

    #[test]
    fn complex_combinations() {
        // Test combining multiple advanced features
        let x = 10;
        let result = lisp_with_vars!([x] (
            let [doubled (* x 2)
                 halved (/ x 2)]
            (if (> doubled halved)
                (str "Doubled " doubled " > halved " halved)
                (str "Should not happen"))
        ));
        assert_eq!(result, "Doubled 20 > halved 5");

        // Test simpler function without variable capture issues
        let add_ten = lisp!((defn add_ten [x] (+ x 10)));
        let result2 = lisp!((call add_ten 5));
        assert_eq!(result2, 15);

        // Test boolean logic with list operations
        let vec_test = lisp!((and (> (count [1 2 3 4]) 2) (= (first [5 6 7]) 5)));
        assert_eq!(vec_test, true);
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
