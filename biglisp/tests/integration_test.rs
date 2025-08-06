// Simple BigLisp Integration Example
// This demonstrates the core features that are actually implemented

use biglisp::prelude::*;

fn main() {
    println!("🚀 BigLisp Simple Integration Examples\n");

    // =================================================================
    // EXAMPLE 1: Basic Arithmetic
    // =================================================================
    println!("📊 Basic Arithmetic:");

    let add_result = lisp!((+ 1 2 3 4 5));
    let sub_result = lisp!((- 20 5 3));
    let mul_result = lisp!((* 2 3 4));
    let div_result = lisp!((/ 24 2 3));

    println!("  Addition: (+ 1 2 3 4 5) = {}", add_result);
    println!("  Subtraction: (- 20 5 3) = {}", sub_result);
    println!("  Multiplication: (* 2 3 4) = {}", mul_result);
    println!("  Division: (/ 24 2 3) = {}", div_result);
    println!();

    // =================================================================
    // EXAMPLE 2: Nested Expressions
    // =================================================================
    println!("🔗 Nested Expressions:");

    let nested1 = lisp!((+ (* 2 3) (/ 8 2)));
    let nested2 = lisp!((* (+ 1 2) (- 5 1)));
    let complex = lisp!((/ (* (+ 10 5) (- 20 5)) 5));

    println!("  (+ (* 2 3) (/ 8 2)) = {}", nested1);
    println!("  (* (+ 1 2) (- 5 1)) = {}", nested2);
    println!("  (/ (* (+ 10 5) (- 20 5)) 5) = {}", complex);
    println!();

    // =================================================================
    // EXAMPLE 3: Comparisons and Boolean Logic
    // =================================================================
    println!("🔍 Comparisons and Boolean Logic:");

    let eq_result = lisp!((= 5 5));
    let lt_result = lisp!((< 3 7));
    let gt_result = lisp!((> 10 5));
    let and_result = lisp!((and true false));
    let or_result = lisp!((or false true));
    let not_result = lisp!((not false));

    println!("  (= 5 5) = {}", eq_result);
    println!("  (< 3 7) = {}", lt_result);
    println!("  (> 10 5) = {}", gt_result);
    println!("  (and true false) = {}", and_result);
    println!("  (or false true) = {}", or_result);
    println!("  (not false) = {}", not_result);
    println!();

    // =================================================================
    // EXAMPLE 4: Variable Capture
    // =================================================================
    println!("🧩 Variable Capture:");

    let x = 10;
    let y = 5;
    let z = 3;

    let captured1 = lisp!([x, y] (+ x y));
    let captured2 = lisp!([x, y, z] (* x (+ y z)));
    let captured3 = lisp!([x] (if (> x 5) (* x 2) x));

    println!("  x = {}, y = {}, z = {}", x, y, z);
    println!("  (+ x y) = {}", captured1);
    println!("  (* x (+ y z)) = {}", captured2);
    println!("  (if (> x 5) (* x 2) x) = {}", captured3);
    println!();

    // =================================================================
    // EXAMPLE 5: Conditionals
    // =================================================================
    println!("🎛️ Conditionals:");

    let cond1 = lisp!((if (> 5 3) "yes" "no"));
    let cond2 = lisp!((if (= 2 3) 100 200));

    let user_age = 25;
    let adult_check = lisp!([user_age] (if (> user_age 18) "adult" "minor"));

    println!("  (if (> 5 3) \"yes\" \"no\") = {}", cond1);
    println!("  (if (= 2 3) 100 200) = {}", cond2);
    println!(
        "  Age {}: {} = {}",
        user_age, "(if (> user_age 18) \"adult\" \"minor\")", adult_check
    );
    println!();

    // =================================================================
    // EXAMPLE 6: Local Bindings (let)
    // =================================================================
    println!("📦 Local Bindings:");

    let let1 = lisp!((let [a 5] a));
    let let2 = lisp!((let [a 3 b 4] (+ a b)));
    let let3 = lisp!((let [x (+ 2 3) y (* 2 4)] (+ x y)));

    println!("  (let [a 5] a) = {}", let1);
    println!("  (let [a 3 b 4] (+ a b)) = {}", let2);
    println!("  (let [x (+ 2 3) y (* 2 4)] (+ x y)) = {}", let3);
    println!();

    // =================================================================
    // EXAMPLE 7: Data Structures (Vectors)
    // =================================================================
    println!("📋 Data Structures:");

    let vec1 = lisp!([1 2 3 4]);
    let first_elem = lisp!((first [10 20 30]));
    let rest_elems = lisp!((rest [10 20 30]));
    let count_elems = lisp!((count [1 2 3 4 5]));
    let cons_result = lisp!((cons 0 [1 2 3]));

    println!("  [1 2 3 4] = {:?}", vec1);
    println!("  (first [10 20 30]) = {}", first_elem);
    println!("  (rest [10 20 30]) = {:?}", rest_elems);
    println!("  (count [1 2 3 4 5]) = {}", count_elems);
    println!("  (cons 0 [1 2 3]) = {:?}", cons_result);
    println!();

    // =================================================================
    // EXAMPLE 8: String Operations
    // =================================================================
    println!("📝 String Operations:");

    let str1 = lisp!((str "hello" " " "world"));
    let str2 = lisp!((str "The answer is " 42));

    let name = "Alice";
    let score = 95;
    let personal_msg = lisp!([name, score] (str "Hi " name "! Your score: " score));

    println!("  (str \"hello\" \" \" \"world\") = {}", str1);
    println!("  (str \"The answer is \" 42) = {}", str2);
    println!("  Personal message: {}", personal_msg);
    println!();

    // =================================================================
    // EXAMPLE 9: Functions
    // =================================================================
    println!("🔧 Functions:");

    let square = lisp!((defn square [x] (* x x)));
    let add_nums = lisp!((defn add_nums [a b] (+ a b)));

    let square_result = lisp!((call square 6));
    let add_result = lisp!((call add_nums 10 20));

    println!("  Defined square function");
    println!("  (call square 6) = {}", square_result);
    println!("  (call add_nums 10 20) = {}", add_result);
    println!();

    // =================================================================
    // EXAMPLE 10: Sequential Execution (do)
    // =================================================================
    println!("🔄 Sequential Execution:");

    let do_result = lisp!((do
        (+ 1 2)
        (* 3 4)
        (- 10 5)
    ));

    println!("  do block result (returns last expression): {}", do_result);
    println!();

    // =================================================================
    // EXAMPLE 11: Error Handling
    // =================================================================
    println!("⚠️ Error Handling:");

    let safe1 = lisp!((try (+ 1 2) 999));
    let safe2 = lisp!((try (/ 10 2) 0));

    println!("  (try (+ 1 2) 999) = {}", safe1);
    println!("  (try (/ 10 2) 0) = {}", safe2);
    println!();

    // =================================================================
    // EXAMPLE 12: Real-World Use Case
    // =================================================================
    println!("🌍 Real-World Example - Price Calculator:");

    let base_price = 100;
    let tax_rate = 8; // 8%
    let discount = 10; // 10%

    let final_price = lisp!([base_price, tax_rate, discount] (
        let [tax_amount (/ (* base_price tax_rate) 100)
             price_with_tax (+ base_price tax_amount)
             discount_amount (/ (* price_with_tax discount) 100)
             final_amount (- price_with_tax discount_amount)]
        final_amount
    ));

    println!("  Base price: ${}", base_price);
    println!("  Tax rate: {}%", tax_rate);
    println!("  Discount: {}%", discount);
    println!("  Final price: ${}", final_price);
    println!();

    println!("✅ All examples completed successfully!");
    println!("\n💡 BigLisp Benefits:");
    println!("  • Zero runtime overhead - compiles to native Rust code");
    println!("  • Seamless Rust variable integration");
    println!("  • Type-safe functional programming");
    println!("  • Perfect for DSLs and complex calculations");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetic() {
        assert_eq!(lisp!((+ 1 2 3)), 6);
        assert_eq!(lisp!((- 10 3)), 7);
        assert_eq!(lisp!((* 2 3 4)), 24);
        assert_eq!(lisp!((/ 12 3)), 4);
    }

    #[test]
    fn test_nested_expressions() {
        assert_eq!(lisp!((+ (* 2 3) (/ 8 2))), 10);
        assert_eq!(lisp!((* (+ 1 2) (- 5 1))), 12);
    }

    #[test]
    fn test_comparisons() {
        assert_eq!(lisp!((= 5 5)), true);
        assert_eq!(lisp!((< 3 7)), true);
        assert_eq!(lisp!((> 10 5)), true);
    }

    #[test]
    fn test_boolean_logic() {
        assert_eq!(lisp!((and true true)), true);
        assert_eq!(lisp!((and true false)), false);
        assert_eq!(lisp!((or false true)), true);
        assert_eq!(lisp!((not false)), true);
    }

    #[test]
    fn test_variable_capture() {
        let x = 10;
        let y = 5;
        assert_eq!(lisp!([x, y] (+ x y)), 15);
        assert_eq!(lisp!([x] (* x 2)), 20);
    }

    #[test]
    fn test_conditionals() {
        assert_eq!(lisp!((if (> 5 3) "yes" "no")), "yes");
        assert_eq!(lisp!((if (< 5 3) 100 200)), 200);

        let age = 25;
        assert_eq!(lisp!([age] (if (> age 18) "adult" "minor")), "adult");
    }

    #[test]
    fn test_let_bindings() {
        assert_eq!(lisp!((let [x 5] x)), 5);
        assert_eq!(lisp!((let [a 3 b 4] (+ a b))), 7);
        assert_eq!(lisp!((let [x (+ 2 3) y (* 2 4)] (+ x y))), 13);
    }

    #[test]
    fn test_data_structures() {
        assert_eq!(lisp!([1 2 3]), vec![1, 2, 3]);
        assert_eq!(lisp!((first [10 20 30])), 10);
        assert_eq!(lisp!((rest [10 20 30])), vec![20, 30]);
        assert_eq!(lisp!((count [1 2 3 4 5])), 5);
        assert_eq!(lisp!((cons 0 [1 2 3])), vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_string_operations() {
        assert_eq!(lisp!((str "hello" " " "world")), "hello world");
        assert_eq!(lisp!((str "Answer: " 42)), "Answer: 42");

        let name = "Alice";
        assert_eq!(lisp!([name] (str "Hi " name "!")), "Hi Alice!");
    }

    #[test]
    fn test_functions() {
        let square = lisp!((defn square [x] (* x x)));
        assert_eq!(lisp!((call square 5)), 25);

        let add = lisp!((defn add [a b] (+ a b)));
        assert_eq!(lisp!((call add 3 7)), 10);
    }

    #[test]
    fn test_sequential_execution() {
        assert_eq!(lisp!((do (+ 1 2) (* 3 4) (- 10 5))), 5);
    }

    #[test]
    fn test_error_handling() {
        assert_eq!(lisp!((try (+ 1 2) 999)), 3);
        assert_eq!(lisp!((try (/ 10 2) 0)), 5);
    }

    #[test]
    fn test_complex_example() {
        let base = 100;
        let rate = 10;

        let result = lisp!([base, rate] (
            let [amount (/ (* base rate) 100)
                 total (+ base amount)]
            total
        ));

        assert_eq!(result, 110); // 100 + (100 * 10 / 100) = 110
    }
}
