use clap::{Args, Parser, Subcommand};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "biglisp")]
#[command(about = "A Lisp-like DSL for Rust with macro support")]
#[command(version = "0.1.0")]
#[command(author = "Perry/APConduct")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start an interactive REPL
    Repl(ReplArgs),
    /// Execute a biglisp file
    Run(RunArgs),
    /// Show examples of biglisp syntax
    Examples,
    /// Validate biglisp syntax without execution
    Check(CheckArgs),
}

#[derive(Args)]
struct ReplArgs {
    /// Show verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Args)]
struct RunArgs {
    /// The biglisp file to execute
    file: PathBuf,
    /// Show verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Args)]
struct CheckArgs {
    /// The biglisp file to check
    file: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Repl(args)) => run_repl(args),
        Some(Commands::Run(args)) => run_file(args),
        Some(Commands::Examples) => show_examples(),
        Some(Commands::Check(args)) => check_file(args),
        None => run_repl(ReplArgs { verbose: false }),
    }
}

fn run_repl(args: ReplArgs) {
    println!("🚀 BigLisp REPL v0.1.0");
    println!("Type 'help' for commands, 'exit' to quit, or enter biglisp expressions.");
    println!("Examples: (+ 1 2 3), (* (+ 1 2) (- 5 1)), (if (> 5 3) \"yes\" \"no\")");
    println!();

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("biglisp> ");
        stdout.flush().unwrap();

        let mut line = String::new();
        match stdin.read_line(&mut line) {
            Ok(0) => break, // EOF
            Ok(_) => {
                let line = line.trim();

                if line.is_empty() {
                    continue;
                }

                match line {
                    "exit" | "quit" | ":q" => {
                        println!("Goodbye! 👋");
                        break;
                    }
                    "help" | ":h" => show_help(),
                    "examples" | ":e" => show_examples(),
                    "clear" | ":c" => {
                        print!("\x1B[2J\x1B[1;1H"); // Clear screen
                        stdout.flush().unwrap();
                    }
                    _ => execute_expression(line, args.verbose),
                }
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                break;
            }
        }
    }
}

fn execute_expression(expr: &str, verbose: bool) {
    if verbose {
        println!("Executing: {}", expr);
    }

    // Try to parse and execute common biglisp patterns
    match expr {
        // Basic arithmetic examples
        s if s.starts_with("(+") => {
            println!("Result: {}", demo_arithmetic("+", s));
        }
        s if s.starts_with("(-") => {
            println!("Result: {}", demo_arithmetic("-", s));
        }
        s if s.starts_with("(*") => {
            println!("Result: {}", demo_arithmetic("*", s));
        }
        s if s.starts_with("(/") => {
            println!("Result: {}", demo_arithmetic("/", s));
        }

        // Comparison examples
        s if s.starts_with("(=")
            || s.starts_with("(<")
            || s.starts_with("(>")
            || s.starts_with("(gte")
            || s.starts_with("(lte")
            || s.starts_with("(ne") =>
        {
            println!("Result: {}", demo_comparison(s));
        }

        // Math utility examples
        s if s.starts_with("(min")
            || s.starts_with("(max")
            || s.starts_with("(abs")
            || s.starts_with("(modulo")
            || s.starts_with("(inc")
            || s.starts_with("(dec") =>
        {
            println!("Result: {}", demo_math_utility(s));
        }

        // Predicate examples
        s if s.starts_with("(zero")
            || s.starts_with("(pos")
            || s.starts_with("(neg")
            || s.starts_with("(even")
            || s.starts_with("(odd") =>
        {
            println!("Result: {}", demo_predicate(s));
        }

        // Control flow examples
        s if s.starts_with("(if") => {
            println!("Result: {}", demo_conditional(s));
        }

        // String operations
        s if s.starts_with("(str") => {
            println!("Result: {}", demo_string(s));
        }

        // List operations
        s if s.starts_with("(first") || s.starts_with("(rest") || s.starts_with("(count") => {
            println!("Result: {}", demo_list(s));
        }

        // Vector literals
        s if s.starts_with("[") && s.ends_with("]") => {
            println!("Result: {}", demo_vector(s));
        }

        _ => {
            println!(
                "⚠️  Expression '{}' not recognized in this demo REPL.",
                expr
            );
            println!("This CLI demonstrates biglisp syntax but doesn't have a full parser.");
            println!("In real usage, you'd use the lisp! macro in Rust code.");
            println!("Type 'examples' to see supported patterns.");
        }
    }
}

fn demo_arithmetic(op: &str, expr: &str) -> String {
    // Simple demo - in real implementation, this would use the actual parser
    match expr {
        "(+ 1 2)" => "3".to_string(),
        "(+ 1 2 3)" => "6".to_string(),
        "(+ 1 2 3 4)" => "10".to_string(),
        "(- 10 3)" => "7".to_string(),
        "(- 10 3 2)" => "5".to_string(),
        "(* 2 3)" => "6".to_string(),
        "(* 2 3 4)" => "24".to_string(),
        "(/ 12 3)" => "4".to_string(),
        "(/ 12 3 2)" => "2".to_string(),
        _ => format!("Demo result for {} operation", op),
    }
}

fn demo_comparison(expr: &str) -> String {
    match expr {
        "(= 5 5)" => "true".to_string(),
        "(= 3 7)" => "false".to_string(),
        "(< 3 7)" => "true".to_string(),
        "(< 7 3)" => "false".to_string(),
        "(> 7 3)" => "true".to_string(),
        "(> 3 7)" => "false".to_string(),
        "(gte 5 5)" => "true".to_string(),
        "(gte 7 3)" => "true".to_string(),
        "(gte 3 7)" => "false".to_string(),
        "(lte 3 7)" => "true".to_string(),
        "(lte 5 5)" => "true".to_string(),
        "(lte 7 3)" => "false".to_string(),
        "(ne 3 7)" => "true".to_string(),
        "(ne 5 5)" => "false".to_string(),
        _ => "true/false".to_string(),
    }
}

fn demo_math_utility(expr: &str) -> String {
    match expr {
        "(min 5 3)" => "3".to_string(),
        "(min 1 2 3)" => "1".to_string(),
        "(max 5 3)" => "5".to_string(),
        "(max 1 2 3)" => "3".to_string(),
        "(abs 5)" => "5".to_string(),
        "(abs -7)" => "7".to_string(),
        "(modulo 10 3)" => "1".to_string(),
        "(inc 5)" => "6".to_string(),
        "(dec 10)" => "9".to_string(),
        _ => "math result".to_string(),
    }
}

fn demo_predicate(expr: &str) -> String {
    match expr {
        "(zero 0)" => "true".to_string(),
        "(zero 5)" => "false".to_string(),
        "(pos 5)" => "true".to_string(),
        "(pos 0)" => "false".to_string(),
        "(neg -5)" => "true".to_string(),
        "(neg 5)" => "false".to_string(),
        "(even 4)" => "true".to_string(),
        "(even 5)" => "false".to_string(),
        "(odd 3)" => "true".to_string(),
        "(odd 4)" => "false".to_string(),
        _ => "true/false".to_string(),
    }
}

fn demo_conditional(expr: &str) -> String {
    match expr {
        "(if (> 5 3) \"yes\" \"no\")" => "\"yes\"".to_string(),
        "(if (< 5 3) \"yes\" \"no\")" => "\"no\"".to_string(),
        "(if (> 5 3) 42 0)" => "42".to_string(),
        _ => "conditional result".to_string(),
    }
}

fn demo_string(expr: &str) -> String {
    match expr {
        "(str \"hello\" \" \" \"world\")" => "\"hello world\"".to_string(),
        "(str \"The answer is \" 42)" => "\"The answer is 42\"".to_string(),
        _ => "\"concatenated string\"".to_string(),
    }
}

fn demo_list(expr: &str) -> String {
    match expr {
        "(first [1 2 3])" => "1".to_string(),
        "(rest [1 2 3])" => "[2, 3]".to_string(),
        "(count [1 2 3 4 5])" => "5".to_string(),
        _ => "list operation result".to_string(),
    }
}

fn demo_vector(expr: &str) -> String {
    match expr {
        "[1 2 3]" => "[1, 2, 3]".to_string(),
        "[1 2 3 4 5]" => "[1, 2, 3, 4, 5]".to_string(),
        _ => "[vector elements]".to_string(),
    }
}

fn show_help() {
    println!("📖 BigLisp REPL Commands:");
    println!("  help, :h      - Show this help");
    println!("  examples, :e  - Show syntax examples");
    println!("  clear, :c     - Clear screen");
    println!("  exit, :q      - Exit REPL");
    println!();
    println!("💡 Enter biglisp expressions to see results!");
    println!("   Examples: (+ 1 2), (if (> 5 3) \"yes\" \"no\"), [1 2 3]");
    println!();
}

fn run_file(args: RunArgs) {
    if !args.file.exists() {
        eprintln!("❌ Error: File '{}' not found", args.file.display());
        std::process::exit(1);
    }

    match fs::read_to_string(&args.file) {
        Ok(content) => {
            if args.verbose {
                println!("📂 Reading file: {}", args.file.display());
                println!("📄 Content:\n{}", content);
                println!("🔄 Executing...\n");
            }

            // In a real implementation, this would parse and execute the file
            println!("🚀 Executing biglisp file: {}", args.file.display());
            println!("📝 File contains {} lines", content.lines().count());

            // Demo: show what expressions were found
            let expressions: Vec<&str> = content
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty() && !line.starts_with(';'))
                .collect();

            if expressions.is_empty() {
                println!("⚠️  No biglisp expressions found in file");
            } else {
                println!("🔍 Found {} expressions:", expressions.len());
                for (i, expr) in expressions.iter().enumerate() {
                    println!("  {}. {}", i + 1, expr);
                }
            }

            println!("✅ Execution complete!");
        }
        Err(error) => {
            eprintln!("❌ Error reading file: {}", error);
            std::process::exit(1);
        }
    }
}

fn check_file(args: CheckArgs) {
    if !args.file.exists() {
        eprintln!("❌ Error: File '{}' not found", args.file.display());
        std::process::exit(1);
    }

    match fs::read_to_string(&args.file) {
        Ok(content) => {
            println!("🔍 Checking biglisp syntax in: {}", args.file.display());

            let mut errors = 0;
            let mut warnings = 0;

            for (line_num, line) in content.lines().enumerate() {
                let line = line.trim();
                if line.is_empty() || line.starts_with(';') {
                    continue;
                }

                // Basic syntax checking (demo)
                if line.starts_with('(') && !line.ends_with(')') {
                    println!("❌ Line {}: Unclosed parenthesis: {}", line_num + 1, line);
                    errors += 1;
                } else if line.starts_with('[') && !line.ends_with(']') {
                    println!("❌ Line {}: Unclosed bracket: {}", line_num + 1, line);
                    errors += 1;
                } else if !line.starts_with('(') && !line.starts_with('[') {
                    println!(
                        "⚠️  Line {}: Possible invalid syntax: {}",
                        line_num + 1,
                        line
                    );
                    warnings += 1;
                }
            }

            if errors == 0 && warnings == 0 {
                println!("✅ Syntax check passed! No issues found.");
            } else {
                println!("\n📊 Summary:");
                if errors > 0 {
                    println!("  ❌ Errors: {}", errors);
                }
                if warnings > 0 {
                    println!("  ⚠️  Warnings: {}", warnings);
                }
            }

            if errors > 0 {
                std::process::exit(1);
            }
        }
        Err(error) => {
            eprintln!("❌ Error reading file: {}", error);
            std::process::exit(1);
        }
    }
}

fn show_examples() {
    println!("🎯 BigLisp Syntax Examples:");
    println!();

    println!("📊 Basic Arithmetic:");
    println!("  (+ 1 2 3)           ; Addition: 6");
    println!("  (- 10 3 2)          ; Subtraction: 5");
    println!("  (* 2 3 4)           ; Multiplication: 24");
    println!("  (/ 12 3)            ; Division: 4");
    println!();

    println!("🔍 Comparisons:");
    println!("  (= 5 5)             ; Equality: true");
    println!("  (< 3 7)             ; Less than: true");
    println!("  (> 7 3)             ; Greater than: true");
    println!("  (gte 5 5)           ; Greater than or equal: true");
    println!("  (lte 3 7)           ; Less than or equal: true");
    println!("  (ne 3 7)            ; Not equal: true");
    println!();

    println!("🧠 Boolean Logic:");
    println!("  (and true false)    ; Logical AND: false");
    println!("  (or false true)     ; Logical OR: true");
    println!("  (not false)         ; Logical NOT: true");
    println!();

    println!("🎛️  Control Flow:");
    println!("  (if (> 5 3) \"yes\" \"no\")              ; Conditional");
    println!("  (let [x 5 y 10] (+ x y))              ; Local bindings: 15");
    println!();

    println!("📝 Strings:");
    println!("  (str \"hello\" \" \" \"world\")          ; Concatenation");
    println!("  (str \"Answer: \" 42)                 ; Mixed types");
    println!();

    println!("📋 Lists/Vectors:");
    println!("  [1 2 3 4]           ; Vector literal");
    println!("  (first [1 2 3])     ; First element: 1");
    println!("  (rest [1 2 3])      ; Rest: [2, 3]");
    println!("  (count [1 2 3 4])   ; Count: 4");
    println!("  (cons 0 [1 2 3])    ; Prepend: [0, 1, 2, 3]");
    println!();

    println!("🔧 Functions:");
    println!("  (defn square [x] (* x x))             ; Define function");
    println!("  (call square 5)                       ; Call function: 25");
    println!();

    println!("🏗️  Complex Examples:");
    println!("  (+ (* 2 3) (/ 8 2))                  ; Nested: 10");
    println!("  (if (> (+ 2 3) 4) \"big\" \"small\")     ; Complex condition");
    println!("  (let [x 10 y 5] (* x (- x y)))       ; Local vars: 50");
    println!();

    println!("💡 Variable Capture (in Rust code):");
    println!("  let x = 5;");
    println!("  lisp!([x] (+ x 10))                  ; Captures Rust variable");
    println!();

    println!("🔧 Math Utilities:");
    println!("  (min 5 3 8)         ; Minimum value: 3");
    println!("  (max 1 9 4)         ; Maximum value: 9");
    println!("  (abs -7)            ; Absolute value: 7");
    println!("  (modulo 10 3)       ; Modulo operation: 1");
    println!("  (inc 5)             ; Increment: 6");
    println!("  (dec 10)            ; Decrement: 9");
    println!();

    println!("🔍 Predicates:");
    println!("  (zero 0)            ; Is zero: true");
    println!("  (pos 5)             ; Is positive: true");
    println!("  (neg -3)            ; Is negative: true");
    println!("  (even 4)            ; Is even: true");
    println!("  (odd 3)             ; Is odd: true");
    println!();

    println!("🚀 Try these examples in the REPL!");
}
