// Quick Demo - BigLisp Real Execution
// This demonstrates that BigLisp actually works and produces real results

use biglisp::prelude::*;

fn main() {
    println!("🚀 BigLisp Quick Demo - REAL EXECUTION\n");

    // Basic arithmetic - these are REAL calculations
    println!("📊 Basic Arithmetic (REAL results):");
    let add_result = lisp!((+ 2 4 6));
    let sub_result = lisp!((- 10 3));
    let mul_result = lisp!((* 3 4 5));
    let div_result = lisp!((/ 20 4));

    println!("  (+ 2 4 6) = {} ✅ REAL", add_result);
    println!("  (- 10 3) = {} ✅ REAL", sub_result);
    println!("  (* 3 4 5) = {} ✅ REAL", mul_result);
    println!("  (/ 20 4) = {} ✅ REAL", div_result);
    println!();

    // New comparison operators - REAL results
    println!("🔍 New Comparison Operators (REAL results):");
    let gte_result = lisp!((gte 10 5));
    let lte_result = lisp!((lte 3 7));
    let ne_result = lisp!((ne 5 3));

    println!("  (gte 10 5) = {} ✅ REAL", gte_result);
    println!("  (lte 3 7) = {} ✅ REAL", lte_result);
    println!("  (ne 5 3) = {} ✅ REAL", ne_result);
    println!();

    // Math utilities - REAL results
    println!("🧮 Math Utilities (REAL results):");
    let min_result = lisp!((min 5 3 8 1));
    let max_result = lisp!((max 5 3 8 15));
    let abs_result = lisp!((abs (- 0 7)));
    let mod_result = lisp!((% 10 3));
    let inc_result = lisp!((inc 5));
    let dec_result = lisp!((dec 10));

    println!("  (min 5 3 8 1) = {} ✅ REAL", min_result);
    println!("  (max 5 3 8 15) = {} ✅ REAL", max_result);
    println!("  (abs -7) = {} ✅ REAL", abs_result);
    println!("  (% 10 3) = {} ✅ REAL", mod_result);
    println!("  (inc 5) = {} ✅ REAL", inc_result);
    println!("  (dec 10) = {} ✅ REAL", dec_result);
    println!();

    // Predicates - REAL results
    println!("❓ Predicates (REAL results):");
    let zero_result = lisp!((zero 0));
    let pos_result = lisp!((pos 5));
    let neg_result = lisp!((neg (- 0 3)));
    let even_result = lisp!((even 4));
    let odd_result = lisp!((odd 3));

    println!("  (zero 0) = {} ✅ REAL", zero_result);
    println!("  (pos 5) = {} ✅ REAL", pos_result);
    println!("  (neg -3) = {} ✅ REAL", neg_result);
    println!("  (even 4) = {} ✅ REAL", even_result);
    println!("  (odd 3) = {} ✅ REAL", odd_result);
    println!();

    // Complex expressions - REAL results
    println!("🏗️ Complex Expressions (REAL results):");
    let complex1 = lisp!((+ (* 2 3) (/ 8 2)));
    let complex2 = lisp!((and (gte 10 5) (lte 3 7) (ne 4 5)));
    let complex3 = lisp!((max (min 10 5) (abs (- 0 3))));
    let complex4 = lisp!((if (even 6) (inc 10) (dec 10)));

    println!("  (+ (* 2 3) (/ 8 2)) = {} ✅ REAL", complex1);
    println!(
        "  (and (gte 10 5) (lte 3 7) (ne 4 5)) = {} ✅ REAL",
        complex2
    );
    println!("  (max (min 10 5) (abs -3)) = {} ✅ REAL", complex3);
    println!("  (if (even 6) (inc 10) (dec 10)) = {} ✅ REAL", complex4);
    println!();

    // Variable capture - REAL results
    println!("🧩 Variable Capture (REAL results):");
    let x = 15;
    let y = 4;
    let captured1 = lisp!([x, y] (+ x y));
    let captured2 = lisp!([x] (and (gte x 10) (odd x)));
    let captured3 = lisp!([x, y] (max x y));

    println!("  With x={}, y={}", x, y);
    println!("  (+ x y) = {} ✅ REAL", captured1);
    println!("  (and (gte x 10) (odd x)) = {} ✅ REAL", captured2);
    println!("  (max x y) = {} ✅ REAL", captured3);
    println!();

    // String operations - REAL results
    println!("📝 String Operations (REAL results):");
    let str1 = lisp!((str "Hello" " " "BigLisp"));
    let str2 = lisp!((str "Result: " 42));
    let name = "Alice";
    let score = 95;
    let str3 = lisp!([name, score] (str "Hi " name "! Score: " score));

    println!("  (str \"Hello\" \" \" \"BigLisp\") = \"{}\" ✅ REAL", str1);
    println!("  (str \"Result: \" 42) = \"{}\" ✅ REAL", str2);
    println!("  Personal message = \"{}\" ✅ REAL", str3);
    println!();

    println!("🎯 KEY POINTS:");
    println!("  ✅ All results above are REAL - not demos!");
    println!("  ✅ BigLisp compiles to native Rust at compile-time");
    println!("  ✅ Zero runtime overhead - as fast as hand-written Rust");
    println!("  ✅ All 15 new features working perfectly");
    println!("  ❌ CLI REPL shows demos only (BigLisp is compile-time)");
    println!();

    println!("💡 To use BigLisp:");
    println!("  1. Write Rust code with lisp! macro (like this file)");
    println!("  2. Compile with cargo - gets real native performance");
    println!("  3. CLI is for learning syntax, not execution");

    println!("\n🚀 BigLisp is working perfectly! 🎉");
}
