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
    
    // Boolean logic with comparisons
    let check = lisp!((and (> 10 5) (< 3 7) (= 4 4)));
    assert_eq!(check, true);
    
    // Conditional expressions
    let msg = lisp!((if (> (count [1 2 3 4]) 2) "many" "few"));
    assert_eq!(msg, "many");
}
```

### Data Processing
```rust
use biglisp::prelude::*;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    let result = lisp!([numbers] (
        let [first_num (first numbers)
             rest_nums (rest numbers)
             total_count (count numbers)]
        (+ (* first_num 10) total_count)
    ));
    
    assert_eq!(result, 15); // (1 * 10) + 5
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

All BigLisp features are thoroughly tested with over 25 test cases covering:
- Arithmetic operations
- Boolean logic
- Control flow
- Data structures
- String operations
- Function definitions
- Variable capture
- Error handling
- Complex nested expressions

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
| ✅ Comparisons | Complete | `=`, `<`, `>` operators |
| ✅ Boolean Logic | Complete | `and`, `or`, `not` operations |
| ✅ Control Flow | Complete | `if`, `let`, `do` constructs |
| ✅ Data Structures | Complete | Vectors, `first`, `rest`, `count`, `cons` |
| ✅ String Operations | Complete | `str` concatenation |
| ✅ Functions | Complete | `defn` definition, `call` invocation |
| ✅ Variable Capture | Complete | `[vars]` syntax for Rust integration |
| ✅ Error Handling | Complete | `try` expressions |
| ✅ Loops | Complete | `dotimes` iteration |
| ✅ CLI Tool | Complete | Interactive REPL and file execution |
| ✅ Comprehensive Tests | Complete | Full test coverage |

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
- Teaching functional programming concepts
- Rapid prototyping of algorithmic code

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