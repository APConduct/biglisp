// New Features Demo - BigLisp Extended Functionality
// This example demonstrates all the new operators and utility functions added to BigLisp

use biglisp::prelude::*;

fn main() {
    println!("🎉 BigLisp New Features Demo\n");

    // =================================================================
    // EXTENDED COMPARISON OPERATORS
    // =================================================================
    println!("🔍 Extended Comparison Operators:");

    // Greater than or equal (gte)
    let gte_result1 = lisp!((gte 10 5));
    let gte_result2 = lisp!((gte 5 5));
    let gte_result3 = lisp!((gte 3 7));

    println!("  (gte 10 5) = {} (greater)", gte_result1);
    println!("  (gte 5 5) = {} (equal)", gte_result2);
    println!("  (gte 3 7) = {} (less)", gte_result3);

    // Less than or equal (lte)
    let lte_result1 = lisp!((lte 3 7));
    let lte_result2 = lisp!((lte 5 5));
    let lte_result3 = lisp!((lte 10 5));

    println!("  (lte 3 7) = {} (less)", lte_result1);
    println!("  (lte 5 5) = {} (equal)", lte_result2);
    println!("  (lte 10 5) = {} (greater)", lte_result3);

    // Not equal (ne)
    let ne_result1 = lisp!((ne 5 3));
    let ne_result2 = lisp!((ne 7 7));

    println!("  (ne 5 3) = {} (different)", ne_result1);
    println!("  (ne 7 7) = {} (same)", ne_result2);

    // Complex comparison expressions
    let complex_comp = lisp!((and (gte 10 5) (lte 3 7) (ne 4 5)));
    println!(
        "  Complex: (and (gte 10 5) (lte 3 7) (ne 4 5)) = {}",
        complex_comp
    );
    println!();

    // =================================================================
    // MATH UTILITY FUNCTIONS
    // =================================================================
    println!("🧮 Math Utility Functions:");

    // Min and Max
    let min_result = lisp!((min 5 3 8 1));
    let max_result = lisp!((max 5 3 8 15));
    println!("  (min 5 3 8 1) = {}", min_result);
    println!("  (max 5 3 8 15) = {}", max_result);

    // Absolute value
    let abs_pos = lisp!((abs 7));
    let abs_neg = lisp!((abs (- 0 7)));
    println!("  (abs 7) = {}", abs_pos);
    println!("  (abs -7) = {}", abs_neg);

    // Modulo operations
    let mod_result1 = lisp!((% 10 3));
    let mod_result2 = lisp!((modulo 15 4));
    println!("  (% 10 3) = {}", mod_result1);
    println!("  (modulo 15 4) = {}", mod_result2);

    // Increment and decrement
    let inc_result = lisp!((inc 5));
    let dec_result = lisp!((dec 10));
    println!("  (inc 5) = {}", inc_result);
    println!("  (dec 10) = {}", dec_result);

    // Nested math operations
    let nested_math = lisp!((max (min 10 5) (abs (- 0 3))));
    println!("  Nested: (max (min 10 5) (abs -3)) = {}", nested_math);
    println!();

    // =================================================================
    // PREDICATE FUNCTIONS
    // =================================================================
    println!("❓ Predicate Functions:");

    // Zero checking
    let zero_true = lisp!((zero 0));
    let zero_false = lisp!((zero 5));
    println!("  (zero 0) = {}", zero_true);
    println!("  (zero 5) = {}", zero_false);

    // Positive/negative checking
    let pos_result = lisp!((pos 5));
    let neg_result = lisp!((neg (- 0 3)));
    println!("  (pos 5) = {}", pos_result);
    println!("  (neg -3) = {}", neg_result);

    // Even/odd checking
    let even_result = lisp!((even 4));
    let odd_result = lisp!((odd 3));
    println!("  (even 4) = {}", even_result);
    println!("  (odd 3) = {}", odd_result);

    // Combined predicates
    let combined_pred = lisp!((and (pos 5) (even 4) (ne 3 7)));
    println!(
        "  Combined: (and (pos 5) (even 4) (ne 3 7)) = {}",
        combined_pred
    );
    println!();

    // =================================================================
    // VARIABLE CAPTURE WITH NEW FEATURES
    // =================================================================
    println!("🧩 Variable Capture with New Features:");

    let age = 25;
    let score = 87;
    let threshold = 85;

    let age_check = lisp!([age] (and (gte age 18) (lte age 65)));
    let score_check = lisp!([score, threshold] (gte score threshold));
    let bonus_check = lisp!([score] (and (even score) (gte score 80)));

    println!("  Age {}: working age = {}", age, age_check);
    println!(
        "  Score {}: above threshold {} = {}",
        score, threshold, score_check
    );
    println!("  Score {}: bonus eligible = {}", score, bonus_check);

    let numbers = vec![12, 7, 23, 8, 15];
    let analysis = lisp!([numbers] (
        let [first_num (first numbers)
             count_val (count numbers)
             is_even_first (even first_num)
             min_threshold 10]
        (and (gte count_val 3) is_even_first (gte first_num min_threshold))
    ));

    println!("  Numbers [12, 7, 23, 8, 15]: analysis = {}", analysis);
    println!();

    // =================================================================
    // REAL-WORLD USE CASES
    // =================================================================
    println!("🌍 Real-World Examples:");

    // Example 1: Grade calculation with new operators
    let exam_score = 78;
    let homework_avg = 85;
    let participation = 90;

    let final_grade = lisp!([exam_score, homework_avg, participation] (
        let [weighted_score (+ (* exam_score 50) (* homework_avg 30) (* participation 20))
             final_score (/ weighted_score 100)]
        final_score
    ));

    let letter_grade = if final_grade >= 90 {
        "A"
    } else if final_grade >= 80 {
        "B"
    } else if final_grade >= 70 {
        "C"
    } else {
        "F"
    };

    println!("  Grade Calculation:");
    println!(
        "    Exam: {}, Homework: {}, Participation: {}",
        exam_score, homework_avg, participation
    );
    println!("    Final Score: {} ({})", final_grade, letter_grade);

    // Example 2: Data validation
    let user_input = 42;
    let validation = lisp!([user_input] (
        and (gte user_input 1)
            (lte user_input 100)
            (ne user_input 13)
            (pos user_input)
    ));

    println!("  Input Validation:");
    println!("    Value {}: valid = {}", user_input, validation);

    // Example 3: Mathematical properties
    let number = 24;
    let properties = lisp!([number] (
        let [is_even_num (even number)
             is_divisible_by_3 (zero (% number 3))
             is_divisible_by_4 (zero (% number 4))]
        (and is_even_num is_divisible_by_3 is_divisible_by_4)
    ));

    println!("  Number Properties:");
    println!("    {} properties check = {}", number, properties);

    // Example 4: Range and bounds checking
    let temperature = 72;
    let humidity = 45;
    let comfort_zone = lisp!([temperature, humidity] (
        and (gte temperature 68)
            (lte temperature 78)
            (gte humidity 30)
            (lte humidity 60)
    ));

    println!("  Comfort Zone:");
    println!("    Temp: {}°F, Humidity: {}%", temperature, humidity);
    println!("    In comfort zone: {}", comfort_zone);
    println!();

    // =================================================================
    // COMPLEX NESTED EXAMPLES
    // =================================================================
    println!("🏗️ Complex Nested Examples:");

    // Multi-step calculation with new functions
    let complex_calc = lisp!((
        let [a (max 5 (min 10 15))
             b (abs (- 3 8))
             c (inc (% 17 7))
             result (+ (* a b) c)]
        result
    ));
    println!("  Complex calculation result: {}", complex_calc);

    // Conditional branching with predicates
    let x = 16;
    let conditional_result = lisp!([x] (
        if (even x)
            (if (gte x 10)
                (+ (inc x) (abs (- 0 2)))
                (dec x))
            (if (odd x)
                (* x 2)
                x)
    ));
    println!("  Conditional result for {}: {}", x, conditional_result);

    // Simple sequence validation
    let sample_numbers = vec![2, 4, 6, 8];
    let first_num = sample_numbers[0];
    let count_nums = sample_numbers.len() as i32;
    let sequence_valid = lisp!([first_num, count_nums] (
        and (even first_num) (gte count_nums 3)
    ));
    println!(
        "  Sequence {:?} validation: {}",
        sample_numbers, sequence_valid
    );
    println!();

    println!("✅ All new features demonstrated successfully!");
    println!("\n🎯 New Feature Summary:");
    println!("  • Extended comparisons: gte, lte, ne");
    println!("  • Math utilities: min, max, abs, modulo, inc, dec");
    println!("  • Predicates: zero, pos, neg, even, odd");
    println!("  • Perfect integration with variable capture");
    println!("  • Zero runtime overhead - all compile to native Rust");
    println!("  • Seamless composition with existing BigLisp features");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extended_comparisons() {
        assert_eq!(lisp!((gte 10 5)), true);
        assert_eq!(lisp!((gte 5 5)), true);
        assert_eq!(lisp!((gte 3 7)), false);

        assert_eq!(lisp!((lte 3 7)), true);
        assert_eq!(lisp!((lte 5 5)), true);
        assert_eq!(lisp!((lte 10 5)), false);

        assert_eq!(lisp!((ne 5 3)), true);
        assert_eq!(lisp!((ne 7 7)), false);
    }

    #[test]
    fn test_math_utilities() {
        assert_eq!(lisp!((min 5 3 8)), 3);
        assert_eq!(lisp!((max 5 3 8)), 8);
        assert_eq!(lisp!((abs 5)), 5);
        assert_eq!(lisp!((abs (- 0 7))), 7);
        assert_eq!(lisp!((% 10 3)), 1);
        assert_eq!(lisp!((modulo 15 4)), 3);
        assert_eq!(lisp!((inc 5)), 6);
        assert_eq!(lisp!((dec 10)), 9);
    }

    #[test]
    fn test_predicates() {
        assert_eq!(lisp!((zero 0)), true);
        assert_eq!(lisp!((zero 5)), false);
        assert_eq!(lisp!((pos 5)), true);
        assert_eq!(lisp!((pos 0)), false);
        assert_eq!(lisp!((neg (- 0 5))), true);
        assert_eq!(lisp!((neg 5)), false);
        assert_eq!(lisp!((even 4)), true);
        assert_eq!(lisp!((even 5)), false);
        assert_eq!(lisp!((odd 3)), true);
        assert_eq!(lisp!((odd 4)), false);
    }

    #[test]
    fn test_variable_capture_with_new_features() {
        let x = 15;
        let y = 4;

        assert_eq!(lisp!([x] (gte x 10)), true);
        assert_eq!(lisp!([x, y] (ne x y)), true);
        assert_eq!(lisp!([x] (odd x)), true);
        assert_eq!(lisp!([y] (even y)), true);
        assert_eq!(lisp!([x, y] (max x y)), 15);
        assert_eq!(lisp!([x, y] (min x y)), 4);
    }

    #[test]
    fn test_complex_expressions() {
        let result1 = lisp!((and (gte 10 5) (lte 3 7) (ne 4 5)));
        assert_eq!(result1, true);

        let result2 = lisp!((max (min 10 5) (abs (- 0 3))));
        assert_eq!(result2, 5);

        let result3 = lisp!((if (even 6) (inc 10) (dec 10)));
        assert_eq!(result3, 11);

        // Test with variable capture
        let x = 15;
        let y = 4;
        let result4 = lisp!([x, y] (and (gte x 10) (ne (% x y) 0)));
        assert_eq!(result4, true); // 15 >= 10 is true, and 15 % 4 = 3 != 0
    }
}
