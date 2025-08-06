// Example Usage of BigLisp in Real Rust Applications
// This file demonstrates how to integrate BigLisp into actual Rust projects
// Using only the features that are currently implemented

use biglisp::prelude::*;

fn main() {
    println!("🚀 BigLisp Integration Examples\n");

    // =================================================================
    // EXAMPLE 1: Basic Mathematical Calculations
    // =================================================================
    println!("📊 Example 1: Mathematical Calculations");

    // Simple arithmetic - compiles to native Rust operations
    let sum = lisp!((+ 10 20 30 40));
    let product = lisp!((* 3 4 5));
    let complex = lisp!((+ (* 2 3) (/ 8 2) (- 10 3)));

    println!("  Sum: {}", sum); // 100
    println!("  Product: {}", product); // 60
    println!("  Complex: {}", complex); // 13
    println!();

    // =================================================================
    // EXAMPLE 2: Variable Capture from Rust
    // =================================================================
    println!("🧩 Example 2: Variable Capture");

    let base_price = 100;
    let tax_rate = 8; // 8%
    let discount = 15; // 15%

    let final_price = lisp!([base_price, tax_rate, discount] (
        let [with_tax (+ base_price (/ (* base_price tax_rate) 100))
             discount_amount (/ (* with_tax discount) 100)]
        (- with_tax discount_amount)
    ));

    println!("  Base price: ${}", base_price);
    println!("  Tax rate: {}%", tax_rate);
    println!("  Discount: {}%", discount);
    println!("  Final price: ${}", final_price);
    println!();

    // =================================================================
    // EXAMPLE 3: Data Processing with Lists
    // =================================================================
    println!("📋 Example 3: Data Processing");

    let scores = vec![85, 92, 78, 96, 88];
    let weights = vec![2, 3, 1, 4, 2];

    let analysis = lisp!([scores, weights] (
        let [total_scores (count scores)
             first_score (first scores)
             rest_scores (rest scores)
             total_weights (count weights)]
        (if (= total_scores total_weights)
            (+ first_score total_scores)
            0)
    ));

    println!("  Scores: {:?}", scores);
    println!("  Weights: {:?}", weights);
    println!("  Analysis result: {}", analysis);
    println!();

    // =================================================================
    // EXAMPLE 4: Conditional Logic for Business Rules
    // =================================================================
    println!("🎛️ Example 4: Business Logic");

    let user_age = 25;
    let account_balance = 1500;
    let min_age = 18;
    let min_balance = 1000;

    let age_check = lisp!([user_age, min_age] (> user_age min_age));
    let balance_check = lisp!([account_balance, min_balance] (> account_balance min_balance));
    let loan_approved = lisp!([age_check, balance_check] (and age_check balance_check));

    let loan_amount = lisp!([loan_approved, account_balance] (
        if loan_approved
            (* account_balance 3)
            0
    ));

    println!("  User age: {}", user_age);
    println!("  Account balance: ${}", account_balance);
    println!("  Age check: {}", age_check);
    println!("  Balance check: {}", balance_check);
    println!("  Loan approved: {}", loan_approved);
    println!("  Max loan amount: ${}", loan_amount);
    println!();

    // =================================================================
    // EXAMPLE 5: String Processing and Formatting
    // =================================================================
    println!("📝 Example 5: String Processing");

    let username = "alice";
    let login_count = 42;
    let data = vec![1, 2, 3, 4, 5];

    let welcome_message = lisp!([username, login_count, data] (
        str "Welcome back, " username "! "
            "You've logged in " login_count " times. "
            "You have " (count data) " items in your dashboard."
    ));

    println!("  Username: {}", username);
    println!("  Login count: {}", login_count);
    println!("  Data items: {:?}", data);
    println!("  Message: {}", welcome_message);
    println!();

    // =================================================================
    // EXAMPLE 6: Error Handling and Validation
    // =================================================================
    println!("⚠️ Example 6: Error Handling");

    let input_data = vec![10, 2, 5];
    let safe_division = lisp!([input_data] (
        try (/ (first input_data) (first (rest input_data)))
            999
    ));

    let validation_result = lisp!([input_data] (
        let [count_val (count input_data)
             first_val (first input_data)]
        (if (and (> count_val 0) (> first_val 0))
            (str "Valid data with " count_val " items")
            (str "Invalid data"))
    ));

    println!("  Input data: {:?}", input_data);
    println!("  Safe division result: {}", safe_division);
    println!("  Validation: {}", validation_result);
    println!();

    // =================================================================
    // EXAMPLE 7: Function Definitions and Reuse
    // =================================================================
    println!("🔧 Example 7: Function Definitions");

    // Define reusable functions
    let calculate_area = lisp!((defn area [width height] (* width height)));
    let calculate_perimeter = lisp!((defn perimeter [width height]
        (* 2 (+ width height))));

    let room_width = 12;
    let room_height = 8;

    let area_result = lisp!((call calculate_area 12 8));
    let perimeter_result = lisp!((call calculate_perimeter 12 8));

    println!("  Room dimensions: {}x{}", room_width, room_height);
    println!("  Area: {} sq ft", area_result);
    println!("  Perimeter: {} ft", perimeter_result);
    println!();

    // =================================================================
    // EXAMPLE 8: Configuration and Settings Processing
    // =================================================================
    println!("⚙️ Example 8: Configuration Processing");

    let config_enabled = true;
    let max_connections = 100;
    let timeout_seconds = 30;
    let features = vec!["auth", "logging", "cache"];

    let connection_ratio = lisp!([max_connections] (/ max_connections 10));
    let feature_count = lisp!([features] (count features));
    let has_enough_features = lisp!([feature_count] (> feature_count 2));

    let system_status = lisp!([config_enabled, has_enough_features, connection_ratio, timeout_seconds] (
        if config_enabled
            (if has_enough_features
                (str "System ready with " connection_ratio " connection pools, " timeout_seconds "s timeout")
                (str "System partially ready"))
            (str "System disabled")
    ));

    println!("  Config enabled: {}", config_enabled);
    println!("  Max connections: {}", max_connections);
    println!("  Timeout: {}s", timeout_seconds);
    println!("  Features: {:?}", features);
    println!("  Status: {}", system_status);
    println!();

    // =================================================================
    // EXAMPLE 9: Real-Time Calculations
    // =================================================================
    println!("⏱️ Example 9: Real-Time Calculations");

    let sensor_readings = vec![23, 25, 24, 26, 22];
    let threshold = 24;
    let alert_count = 2;

    let monitoring_result = lisp!([sensor_readings, threshold, alert_count] (
        let [total_readings (count sensor_readings)
             first_reading (first sensor_readings)
             second_reading (first (rest sensor_readings))
             avg_estimate (/ (+ first_reading second_reading) 2)]
        (if (> total_readings alert_count)
            (if (> avg_estimate threshold)
                (str "ALERT: Average " avg_estimate " exceeds threshold " threshold)
                (str "OK: Average " avg_estimate " within limits"))
            (str "Insufficient data"))
    ));

    println!("  Sensor readings: {:?}", sensor_readings);
    println!("  Threshold: {}", threshold);
    println!("  Monitoring: {}", monitoring_result);
    println!();

    // =================================================================
    // EXAMPLE 10: Complex Nested Logic
    // =================================================================
    println!("🏗️ Example 10: Complex Nested Logic");

    let user_roles = vec!["user", "moderator"];
    let permission_level = 5;
    let resource_cost = 100;
    let user_credits = 150;

    let has_multiple_roles = lisp!([user_roles] (> (count user_roles) 1));
    let can_afford = lisp!([user_credits, resource_cost] (> user_credits resource_cost));
    let sufficient_permissions = lisp!([permission_level] (> permission_level 3));

    let access_decision = lisp!([has_multiple_roles, sufficient_permissions, can_afford, user_credits, resource_cost] (
        if (and has_multiple_roles sufficient_permissions)
            (if can_afford
                (let [remaining_credits (- user_credits resource_cost)]
                (str "ACCESS GRANTED. Remaining credits: " remaining_credits))
                (str "ACCESS DENIED: Insufficient credits"))
            (str "ACCESS DENIED: Insufficient permissions")
    ));

    println!("  User roles: {:?}", user_roles);
    println!("  Permission level: {}", permission_level);
    println!("  Resource cost: {}", resource_cost);
    println!("  User credits: {}", user_credits);
    println!("  Decision: {}", access_decision);
    println!();

    // =================================================================
    // EXAMPLE 11: List Manipulation and Processing
    // =================================================================
    println!("📝 Example 11: Advanced List Operations");

    let original_list = vec![10, 20, 30, 40, 50];

    let list_analysis = lisp!([original_list] (
        let [first_elem (first original_list)
             rest_list (rest original_list)
             total_count (count original_list)
             new_list (cons 0 original_list)]
        (+ first_elem (count new_list))
    ));

    let string_result = lisp!([original_list] (
        str "Original list has " (count original_list) " elements, "
            "first element is " (first original_list)
    ));

    println!("  Original list: {:?}", original_list);
    println!("  Analysis result: {}", list_analysis);
    println!("  Description: {}", string_result);
    println!();

    // =================================================================
    // EXAMPLE 12: Sequential Operations
    // =================================================================
    println!("🔄 Example 12: Sequential Operations");

    let step1 = lisp!((+ 5 10));
    let step2 = lisp!([step1] (* step1 2));
    let step3 = lisp!([step2] (/ step2 3));

    let sequential_result = lisp!((do
        (+ 1 2)
        (* 3 4)
        (- 20 5)
    ));

    println!("  Step 1 (5 + 10): {}", step1);
    println!("  Step 2 (step1 * 2): {}", step2);
    println!("  Step 3 (step2 / 3): {}", step3);
    println!("  Sequential do result: {}", sequential_result);
    println!();

    println!("✅ All examples completed successfully!");
    println!("\n💡 Key Benefits Demonstrated:");
    println!("  • Zero runtime overhead - compiles to native Rust");
    println!("  • Seamless variable capture from Rust scope");
    println!("  • Type safety through Rust's type system");
    println!("  • Expressive functional syntax for complex logic");
    println!("  • Perfect for DSLs, configuration, and business rules");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_integration() {
        let x = 10;
        let y = 5;
        let result = lisp!([x, y] (+ (* x 2) y));
        assert_eq!(result, 25);
    }

    #[test]
    fn test_conditional_logic() {
        let age = 25;
        let min_age = 18;
        let can_vote = lisp!([age, min_age] (> age min_age));
        assert_eq!(can_vote, true);
    }

    #[test]
    fn test_data_processing() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = lisp!([numbers] (
            let [first_num (first numbers)
                 total_count (count numbers)]
            (+ first_num total_count)
        ));

        assert_eq!(result, 6); // 1 + 5
    }

    #[test]
    fn test_string_operations() {
        let name = "Alice";
        let score = 95;

        let message = lisp!([name, score] (
            str "Congratulations " name "! Your score: " score "%"
        ));

        assert_eq!(message, "Congratulations Alice! Your score: 95%");
    }

    #[test]
    fn test_function_definition() {
        let square = lisp!((defn square [x] (* x x)));
        let result = lisp!((call square 7));
        assert_eq!(result, 49);
    }

    #[test]
    fn test_boolean_logic() {
        let result1 = lisp!((and true true));
        let result2 = lisp!((or false true));
        let result3 = lisp!((not false));

        assert_eq!(result1, true);
        assert_eq!(result2, true);
        assert_eq!(result3, true);
    }

    #[test]
    fn test_list_operations() {
        let data = vec![10, 20, 30];

        let first_elem = lisp!([data] (first data));
        let rest_data = lisp!([data] (rest data));
        let count_data = lisp!([data] (count data));
        let cons_data = lisp!([data] (cons 0 data));

        assert_eq!(first_elem, 10);
        assert_eq!(rest_data, vec![20, 30]);
        assert_eq!(count_data, 3);
        assert_eq!(cons_data, vec![0, 10, 20, 30]);
    }

    #[test]
    fn test_error_handling() {
        let safe_result = lisp!((try (+ 1 2) 999));
        assert_eq!(safe_result, 3);

        let safe_division = lisp!((try (/ 10 2) 0));
        assert_eq!(safe_division, 5);
    }
}
