# BigLisp

A powerful Lisp-like Domain Specific Language (DSL) for Rust that provides seamless integration between Lisp syntax and Rust code through compile-time macros.

## 🚀 Quick Start

Add BigLisp to your `Cargo.toml`:

```toml
[dependencies]
biglisp = "0.1.0"
```

Use the `lisp!` macro in your Rust code:

```rust
use biglisp::prelude::*;

fn main() {
    // Basic arithmetic
    let result = lisp!((+ 1 2 3 4));
    println!("Sum: {}", result); // Sum: 10
    
    // Complex expressions
    let complex = lisp!((* (+ 1 2) (- 5 1)));
    println!("Result: {}", complex); // Result: 12
    
    // Variable capture
    let x = 10;
    let y = 5;
    let captured = lisp!([x, y] (+ (* x 2) y));
    println!("Captured: {}", captured); // Captured: 25
}
```

## ✨ Features

### 🔢 Arithmetic Operations
```rust
lisp!((+ 1 2 3 4))           // Addition: 10
lisp!((- 20 5 3))            // Subtraction: 12
lisp!((* 2 3 4))             // Multiplication: 24
lisp!((/ 24 2 3))            // Division: 4
```

### 🔍 Comparisons & Boolean Logic
```rust
lisp!((= 5 5))               // Equality: true
lisp!((< 3 7))               // Less than: true
lisp!((> 10 5))              // Greater than: true
lisp!((gte 5 5))             // Greater than or equal: true
lisp!((lte 3 7))             // Less than or equal: true
lisp!((ne 3 7))              // Not equal: true
lisp!((and true false))      // Logical AND: false
lisp!((or false true))       // Logical OR: true
lisp!((not false))           // Logical NOT: true
```

### 🎛️ Control Flow
```rust
// Conditionals
lisp!((if (> 5 3) "yes" "no"))

// Local bindings
lisp!((let [x 5 y 10] (+ x y)))

// Sequential execution
lisp!((do
    (+ 1 2)
    (* 3 4)
    (- 10 5)
))
```

### 📋 Data Structures
```rust
// Vectors
lisp!([1 2 3 4])

// List operations
lisp!((first [1 2 3]))       // First element: 1
lisp!((rest [1 2 3]))        // Rest: [2, 3]
lisp!((count [1 2 3 4]))     // Count: 4
lisp!((cons 0 [1 2 3]))      // Prepend: [0, 1, 2, 3]
```

### 🔧 Functions
```rust
// Define functions
let square = lisp!((defn square [x] (* x x)));
let result = lisp!((call square 5)); // Result: 25
```

### 🧮 Math Utilities
```rust
lisp!((min 5 3 8))           // Minimum: 3
lisp!((max 1 9 4))           // Maximum: 9
lisp!((abs (- 0 7)))         // Absolute value: 7
lisp!((modulo 10 3))         // Modulo: 1
lisp!((inc 5))               // Increment: 6
lisp!((dec 10))              // Decrement: 9
```

### 🔍 Predicates
```rust
lisp!((zero 0))              // Is zero: true
lisp!((pos 5))               // Is positive: true
lisp!((neg (- 0 3)))         // Is negative: true
lisp!((even 4))              // Is even: true
lisp!((odd 3))               // Is odd: true
```

### 🧩 Variable Capture
```rust
let rust_var = 42;
let data = vec![1, 2, 3];

// Capture Rust variables in BigLisp expressions
let result = lisp!([rust_var, data] (
    let [doubled (* rust_var 2)
         count (count data)]
    (+ doubled count)
)); // Result: 87
```

## 🖥️ Command Line Interface

BigLisp includes a powerful CLI for interactive development and file execution:

### Installation
```bash
cargo install --path biglisp-cli
```

### Interactive REPL
```bash
biglisp-cli repl

# Example session:
biglisp> (+ 1 2 3)
Result: 6

biglisp> (if (> 5 3) "yes" "no")
Result: "yes"

biglisp> examples
# Shows comprehensive syntax examples
```

### File Execution
```bash
# Run BigLisp files
biglisp-cli run examples/arithmetic.lisp

# Check syntax without execution
biglisp-cli check examples/comprehensive_demo.lisp

# Show all syntax examples
biglisp-cli examples
```

### Using Make Commands
```bash
make repl                    # Start interactive REPL
make examples               # Show syntax examples
make run FILE=path.lisp     # Execute a file
make check FILE=path.lisp   # Check syntax
```

## 📁 Project Structure

```
biglisp/
├── biglisp/              # Main library crate
├── biglisp-cli/          # Command-line interface
├── biglisp-core/         # Core parsing and AST
├── biglisp-macros/       # Procedural macros
├── examples/             # Example BigLisp files
└── README.md            # This file
```

## 🎯 Design Philosophy

BigLisp is designed with these principles:

1. **Zero Runtime Overhead** - All BigLisp expressions compile to native Rust code
2. **Seamless Integration** - Capture and use Rust variables naturally
3. **Type Safety** - Leverage Rust's type system for safety
4. **Familiar Syntax** - Classic Lisp syntax that's easy to learn
5. **Compile-Time Evaluation** - Everything happens at compile time

## 📚 Examples

### Basic Arithmetic and Logic
```rust
use biglisp::prelude::*;

fn main() {
    // Complex nested expressions
    let result = lisp!((+ (* 2 3) (/ 8 2) (- 10 5)));
    assert_eq!(result, 15); // (6 + 4 + 5)
    
    // Extended comparisons and boolean logic
    let check = lisp!((and (gte 10 5) (lte 3 7) (ne 4 5)));
    assert_eq!(check, true);
    
    // Math utilities
    let math_result = lisp!((max (min 10 5) (abs (- 0 3))));
    assert_eq!(math_result, 5);
    
    // Predicates
    let pred_check = lisp!((and (even 4) (pos 5) (zero 0)));
    assert_eq!(pred_check, true);
    
    // Conditional expressions
    let msg = lisp!((if (> (count [1 2 3 4]) 2) "many" "few"));
    assert_eq!(msg, "many");
}
```

### Data Processing with New Features
```rust
use biglisp::prelude::*;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let threshold = 3;
    
    let result = lisp!([numbers, threshold] (
        let [first_num (first numbers)
             total_count (count numbers)
             is_valid (and (gte total_count threshold) (pos first_num))]
        (if is_valid
            (+ (* first_num 10) total_count)
            0)
    ));
    
    assert_eq!(result, 15); // (1 * 10) + 5
    
    // Advanced math operations
    let math_result = lisp!([numbers] (
        let [first_val (first numbers)
             max_val (inc first_val)
             is_even_max (even max_val)]
        (if is_even_max max_val (dec max_val))
    ));
    
    assert_eq!(math_result, 2); // inc(1) = 2, even(2) = true, so 2
}
```

### String Processing
```rust
use biglisp::prelude::*;

fn main() {
    let items = vec!["apple", "banana", "cherry"];
    let multiplier = 3;
    
    let message = lisp!([items, multiplier] (
        str "Shopping list has " (count items) 
            " items, multiplied by " multiplier 
            " equals " (* (count items) multiplier)
    ));
    
    println!("{}", message);
    // Output: "Shopping list has 3 items, multiplied by 3 equals 9"
}
```

## 🧪 Testing

Run the comprehensive test suite:

```bash
# Test all components
make test

# Test just the main library
cargo test -p biglisp

# Test specific functionality
cargo test -p biglisp comprehensive_functionality_audit
```

All BigLisp features are thoroughly tested with 43 comprehensive test cases covering:
- Arithmetic operations (`+`, `-`, `*`, `/`)
- Extended comparisons (`=`, `<`, `>`, `gte`, `lte`, `ne`)
- Boolean logic (`and`, `or`, `not`)
- Control flow (`if`, `let`, `do`)
- Data structures (vectors, `first`, `rest`, `count`, `cons`)
- String operations (`str`)
- Function definitions (`defn`, `call`)
- Math utilities (`min`, `max`, `abs`, `modulo`, `inc`, `dec`)
- Predicates (`zero`, `pos`, `neg`, `even`, `odd`)
- Variable capture (`[vars]` syntax)
- Error handling (`try`)
- Complex nested expressions
- Real-world integration examples

## 🛠️ Development

### Building
```bash
make build              # Debug build
make build-release      # Release build
make clean             # Clean artifacts
```

### Code Quality
```bash
make format            # Format code
make dev-check         # Run clippy and checks
cargo clippy           # Lint code
```

### Documentation
```bash
make doc               # Build and open documentation
```

## 📖 Documentation

- [CLI Documentation](CLI_README.md) - Detailed CLI usage and examples
- [API Documentation](https://docs.rs/biglisp) - Complete API reference
- [Examples Directory](examples/) - Sample BigLisp files

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🎨 Features Overview

| Feature | Status | Description |
|---------|--------|-------------|
| ✅ Arithmetic | Complete | `+`, `-`, `*`, `/` with multiple operands |
| ✅ Comparisons | Complete | `=`, `<`, `>`, `gte`, `lte`, `ne` operators |
| ✅ Boolean Logic | Complete | `and`, `or`, `not` operations |
| ✅ Control Flow | Complete | `if`, `let`, `do` constructs |
| ✅ Data Structures | Complete | Vectors, `first`, `rest`, `count`, `cons` |
| ✅ String Operations | Complete | `str` concatenation |
| ✅ Functions | Complete | `defn` definition, `call` invocation |
| ✅ Math Utilities | Complete | `min`, `max`, `abs`, `modulo`, `inc`, `dec` |
| ✅ Predicates | Complete | `zero`, `pos`, `neg`, `even`, `odd` |
| ✅ Variable Capture | Complete | `[vars]` syntax for Rust integration |
| ✅ Error Handling | Complete | `try` expressions |
| ✅ Loops | Complete | `dotimes` iteration |
| ✅ CLI Tool | Complete | Interactive REPL and file execution |
| ✅ Comprehensive Tests | Complete | Full test coverage (43 tests) |

## 🌟 Why BigLisp?

BigLisp bridges the gap between Lisp's expressive syntax and Rust's performance and safety. Unlike runtime Lisp interpreters, BigLisp expressions compile directly to optimized Rust code, giving you:

- **Performance**: No interpretation overhead
- **Safety**: Rust's compile-time guarantees
- **Integration**: Seamless variable capture from Rust
- **Expressiveness**: Clean, functional syntax for complex operations
- **Tooling**: Full IDE support through Rust toolchain

Perfect for:
- Domain-specific languages in Rust applications
- Configuration and scripting within Rust programs
- Functional programming patterns in systems code
- Mathematical computations and data validation
- Business logic and rule engines
- Teaching functional programming concepts
- Rapid prototyping of algorithmic code
- Complex conditional logic and calculations

## 🚀 Get Started Today!

```bash
# Clone the repository
git clone https://github.com/APConduct/biglisp
cd biglisp

# Try the interactive REPL
make repl

# Run example files
make demo-arithmetic
make demo-comprehensive

# Add to your project
cargo add biglisp
```

Happy coding with BigLisp! 🎉