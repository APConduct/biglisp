# BigLisp CLI

A command-line interface for the BigLisp Rust DSL that provides an interactive REPL and file execution capabilities.

## Overview

BigLisp CLI allows you to:
- **Interactive REPL**: Experiment with BigLisp expressions in real-time
- **File Execution**: Run BigLisp scripts from files
- **Syntax Checking**: Validate BigLisp syntax without execution
- **Examples**: View comprehensive syntax examples

## Installation

From the project root:

```bash
cargo build --release -p biglisp-cli
```

The binary will be available at `target/release/biglisp-cli` (or just `biglisp-cli` if added to PATH).

## Usage

### Interactive REPL

Start an interactive session to experiment with BigLisp expressions:

```bash
biglisp-cli repl
# or simply
biglisp-cli
```

**REPL Commands:**
- `help` or `:h` - Show help
- `examples` or `:e` - Display syntax examples
- `clear` or `:c` - Clear screen
- `exit`, `quit`, or `:q` - Exit REPL

**Example REPL Session:**
```
🚀 BigLisp REPL v0.1.0
biglisp> (+ 1 2 3)
Result: 6

biglisp> (if (> 5 3) "yes" "no")
Result: "yes"

biglisp> [1 2 3 4]
Result: [1, 2, 3, 4]

biglisp> help
📖 BigLisp REPL Commands:
  help, :h      - Show this help
  examples, :e  - Show syntax examples
  ...
```

### File Execution

Execute BigLisp expressions from a file:

```bash
biglisp-cli run examples/arithmetic.lisp
biglisp-cli run examples/comprehensive_demo.lisp --verbose
```

### Syntax Checking

Validate BigLisp syntax without execution:

```bash
biglisp-cli check examples/arithmetic.lisp
```

### Examples

View comprehensive syntax examples:

```bash
biglisp-cli examples
```

## BigLisp Syntax Reference

### Arithmetic Operations
```lisp
(+ 1 2 3 4)          ; Addition: 10
(- 20 5 3)           ; Subtraction: 12
(* 2 3 4)            ; Multiplication: 24
(/ 24 2 3)           ; Division: 4
```

### Comparison Operations
```lisp
(= 5 5)              ; Equality: true
(< 3 7)              ; Less than: true
(> 10 5)             ; Greater than: true
```

### Boolean Logic
```lisp
(and true false)     ; Logical AND: false
(or false true)      ; Logical OR: true
(not false)          ; Logical NOT: true
```

### Control Flow
```lisp
(if (> 5 3) "yes" "no")                    ; Conditional
(let [x 5 y 10] (+ x y))                   ; Local bindings
```

### Data Structures
```lisp
[1 2 3 4]            ; Vector literal
(first [1 2 3])      ; First element: 1
(rest [1 2 3])       ; Rest: [2, 3]
(count [1 2 3 4])    ; Count: 4
(cons 0 [1 2 3])     ; Prepend: [0, 1, 2, 3]
```

### String Operations
```lisp
(str "hello" " " "world")     ; Concatenation: "hello world"
(str "Answer: " 42)           ; Mixed types: "Answer: 42"
```

### Functions
```lisp
(defn square [x] (* x x))     ; Define function
(call square 5)               ; Call function: 25
```

### Advanced Features
```lisp
(do                           ; Sequential execution
  (+ 1 2)
  (* 3 4)
  (- 10 5))

(println "Hello World")       ; Print output

(try (+ 1 2) 0)              ; Error handling

(dotimes i 5 (+ i 1))        ; Loops
```

## Example Files

The `examples/` directory contains sample BigLisp files:

- **`arithmetic.lisp`** - Basic mathematical operations
- **`control_flow.lisp`** - Conditionals, loops, and control structures
- **`data_structures.lisp`** - Working with vectors and lists
- **`comprehensive_demo.lisp`** - Complete feature demonstration

## Integration with Rust

BigLisp is designed to be embedded in Rust applications using the `lisp!` macro:

```rust
use biglisp::prelude::*;

fn main() {
    // Basic expressions
    let result = lisp!((+ 1 2 3));
    println!("Result: {}", result); // Result: 6
    
    // Variable capture
    let x = 10;
    let y = 5;
    let captured = lisp!([x, y] (+ (* x 2) y));
    println!("Captured: {}", captured); // Captured: 25
    
    // Complex expressions
    let complex = lisp!((
        let [a 5 b 10] 
        (if (> a 3) 
            (str "a is " a) 
            "a is small")
    ));
}
```

## CLI Limitations

**Important Note**: The CLI provides a demonstration interface and syntax validation. For full BigLisp functionality, use the `lisp!` macro directly in Rust code. The CLI shows what expressions would look like and validates syntax, but doesn't execute a full BigLisp interpreter.

The real power of BigLisp comes from:
1. **Compile-time macro expansion** in Rust
2. **Zero runtime overhead** - expands to native Rust code
3. **Type safety** - leverages Rust's type system
4. **Variable capture** - seamless integration with Rust variables

## Command Line Options

```
biglisp-cli [COMMAND]

Commands:
  repl      Start an interactive REPL
  run       Execute a biglisp file
  examples  Show examples of biglisp syntax
  check     Validate biglisp syntax without execution
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Subcommand Options

**REPL:**
```bash
biglisp-cli repl [OPTIONS]
  -v, --verbose    Show verbose output
```

**Run:**
```bash
biglisp-cli run [OPTIONS] <FILE>
  -v, --verbose    Show verbose output
```

**Check:**
```bash
biglisp-cli check <FILE>
```

## Development

To contribute to BigLisp CLI:

1. **Build**: `cargo build -p biglisp-cli`
2. **Test**: `cargo test -p biglisp-cli`
3. **Run**: `cargo run -p biglisp-cli -- [args]`

### Project Structure
```
biglisp-cli/
├── src/main.rs          # CLI implementation
├── Cargo.toml           # Dependencies
└── examples/            # Example BigLisp files
```

## License

Same as the main BigLisp project.

## See Also

- [Main BigLisp Documentation](../README.md)
- [BigLisp Core](../biglisp-core/)
- [BigLisp Macros](../biglisp-macros/)