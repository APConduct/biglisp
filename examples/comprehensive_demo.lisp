; Comprehensive BigLisp Demo
; This file showcases all major features of the BigLisp DSL
; Use with: biglisp run examples/comprehensive_demo.lisp

; =============================================================================
; SECTION 1: Basic Arithmetic Operations
; =============================================================================

; Simple arithmetic with multiple operands
(+ 1 2 3 4 5)                    ; Addition: 15
(- 100 20 5 3)                   ; Subtraction: 72
(* 2 3 4)                        ; Multiplication: 24
(/ 120 3 2)                      ; Division: 20

; Single operand cases
(+ 42)                           ; Identity: 42
(* 7)                            ; Identity: 7
(-5)                             ; Unary negation: -5

; Nested arithmetic expressions
(+ (* 2 3) (/ 8 2))              ; Mixed operations: 10
(* (+ 1 2 3) (- 10 4))           ; Complex nesting: 36
(/ (* (+ 10 5) (- 20 5)) 5)      ; Deep nesting: 45

; =============================================================================
; SECTION 2: Comparison and Boolean Operations
; =============================================================================

; Basic comparisons
(= 5 5)                          ; Equality: true
(= 3 7)                          ; Inequality: false
(< 3 7)                          ; Less than: true
(< 7 3)                          ; Greater than: false
(> 10 5)                         ; Greater than: true
(> 3 7)                          ; Less than: false

; Comparisons with expressions
(= (+ 2 3) (* 1 5))              ; Expression equality: true
(< (+ 1 2) (* 2 3))              ; Expression comparison: true
(> (+ (* 2 3) 1) (- 10 3))       ; Complex comparison: false

; Boolean logic operations
(and true true)                  ; AND: true
(and true false)                 ; AND: false
(and (> 5 3) (< 2 4) (= 1 1))    ; Multiple AND: true

(or false true)                  ; OR: true
(or false false)                 ; OR: false
(or (< 5 3) (> 2 4) (= 1 1))     ; Multiple OR: true

(not false)                      ; NOT: true
(not true)                       ; NOT: false
(not (< 5 3))                    ; NOT with expression: true

; =============================================================================
; SECTION 3: Control Flow and Conditionals
; =============================================================================

; Basic if expressions
(if (> 5 3) "greater" "less or equal")
(if (< 2 10) 42 0)
(if (= 1 1) "equal" "not equal")

; If expressions with complex conditions
(if (and (> 5 3) (< 2 10))
    "both conditions true"
    "at least one false")

(if (or (< 5 3) (> 10 5))
    "at least one true"
    "both false")

; Nested conditionals
(if (> (+ 2 3) 4)
    (if (< 1 2) "nested true" "nested false")
    "outer false")

; =============================================================================
; SECTION 4: Local Variable Bindings
; =============================================================================

; Simple let bindings
(let [x 10] x)
(let [x 5 y 7] (+ x y))
(let [a (+ 2 3) b (* 2 4)] (+ a b))

; Complex let expressions
(let [multiplier 3
      base_value 10
      result (* multiplier base_value)]
  (+ result 5))

; Nested let bindings
(let [x 10]
  (let [y 5
        z 3]
    (+ x y z)))

; Let with data structures
(let [numbers [1 2 3 4 5]
      first_num (first numbers)
      rest_nums (rest numbers)
      total_count (count numbers)]
  (+ first_num total_count))

; =============================================================================
; SECTION 5: Data Structures and List Operations
; =============================================================================

; Vector/list literals
[1 2 3 4 5]
[10 20 30 40]
["hello" "world" "biglisp"]
[]

; List operations - accessing elements
(first [1 2 3 4])               ; First element: 1
(first [42])                    ; Single element: 42
(rest [1 2 3 4])                ; Rest of list: [2, 3, 4]
(rest [42])                     ; Empty rest: []

; List operations - information
(count [1 2 3 4 5])             ; Count elements: 5
(count [])                      ; Empty count: 0
(count [42])                    ; Single count: 1

; List operations - building lists
(cons 0 [1 2 3])                ; Prepend: [0, 1, 2, 3]
(cons 42 [])                    ; Prepend to empty: [42]
(cons "start" ["middle" "end"])  ; Prepend string

; Nested data structures
[[1 2] [3 4] [5 6]]
[1 [2 3] 4]
(first [[1 2] [3 4]])           ; First of nested: [1, 2]
(count [[1 2] [3 4] [5 6]])     ; Count nested: 3

; Complex list operations
(count (rest (cons 0 [1 2 3 4])))
(first (rest [10 20 30 40]))
(cons (first [1 2 3]) (rest [10 20 30]))

; =============================================================================
; SECTION 6: String Operations
; =============================================================================

; Basic string concatenation
(str "hello" " " "world")
(str "The answer is " 42)
(str "Hello")

; String concatenation with expressions
(str "Sum: " (+ 2 3))
(str "Count: " (count [1 2 3 4]))
(str "First: " (first [10 20 30]))

; Complex string building
(str "2 + 3 = " (+ 2 3) " and 2 * 3 = " (* 2 3))
(str "List [1,2,3] has " (count [1 2 3]) " elements")

; Strings in conditionals
(if (> (count [1 2 3]) 2)
    (str "List has " (count [1 2 3]) " elements")
    "Small list")

; =============================================================================
; SECTION 7: Function Definitions and Calls
; =============================================================================

; Simple function definitions
(defn square [x] (* x x))
(defn add_two [a b] (+ a b))
(defn triple [n] (* n 3))

; Functions with complex bodies
(defn complex_calc [x y z]
  (+ (* x y) z))

(defn list_info [lst]
  (let [size (count lst)
        first_elem (first lst)]
    (+ size first_elem)))

; Function calls (in actual usage with call)
; (call square 5)                ; Result: 25
; (call add_two 3 7)             ; Result: 10
; (call complex_calc 2 3 4)      ; Result: 10

; Functions with conditionals
(defn absolute [x]
  (if (< x 0) (- x) x))

(defn max_of_two [a b]
  (if (> a b) a b))

; =============================================================================
; SECTION 8: Sequential Execution and Control Flow
; =============================================================================

; Sequential execution with do
(do
  (+ 1 2)
  (* 3 4)
  (- 10 5))

; Do with mixed operations
(do
  (count [1 2 3])
  (str "calculating")
  (+ 10 20))

; Do with let bindings
(do
  (let [x 5] (* x 2))
  (let [y 10] (+ y 5))
  42)

; =============================================================================
; SECTION 9: Loop Constructs and Iteration
; =============================================================================

; Simple loops with dotimes
(dotimes i 5 (+ i 1))
(dotimes n 3 (* (+ n 1) 2))

; Dotimes with complex bodies
(dotimes counter 4
  (let [doubled (* counter 2)]
    (+ doubled 1)))

; =============================================================================
; SECTION 10: Error Handling
; =============================================================================

; Basic try expressions
(try (+ 1 2) 0)
(try (/ 10 2) "error")
(try (* 5 5) -1)

; Try with complex expressions
(try
  (let [x 5 y 10] (* x y))
  "fallback value")

; Try with data operations
(try (first [1 2 3]) 0)
(try (count []) -1)

; Nested try expressions
(try
  (try (+ 5 5) 0)
  (- 0 1))

; =============================================================================
; SECTION 11: Print and Debug Operations
; =============================================================================

; Simple printing
(println 42)
(println "hello world")
(println (+ 2 3))

; Printing multiple values
(println (+ 1 2) (* 3 4))
(println "Result:" (+ 10 20))

; Printing in sequences
(do
  (println "Starting calculation")
  (println "Step 1:" (+ 1 2))
  (println "Step 2:" (* 3 4))
  (println "Final result:" (+ (+ 1 2) (* 3 4))))

; =============================================================================
; SECTION 12: Complex Combined Examples
; =============================================================================

; Comprehensive data processing
(let [data [10 20 30 40 50]
      first_elem (first data)
      rest_data (rest data)
      data_size (count data)
      doubled_first (* first_elem 2)]
  (if (> data_size 3)
    (str "Large dataset with " data_size " elements, first: " first_elem)
    (str "Small dataset")))

; Mathematical sequence processing
(let [sequence [1 2 3 4 5]
      sum (+ (first sequence) (first (rest sequence)))
      product (* (first sequence) (count sequence))
      final_calc (+ sum product)]
  (if (> final_calc 10)
    final_calc
    0))

; Complex conditional logic
(let [condition1 (> 10 5)
      condition2 (= (+ 2 3) 5)
      condition3 (< (count [1 2]) 5)]
  (if (and condition1 condition2 condition3)
    (do
      (println "All conditions met")
      (str "Success: " (+ 1 2 3)))
    (str "Conditions not met")))

; Nested function-like operations
(let [calculator
      (let [base 10
            multiplier 3]
        (* base multiplier))]
  (+ calculator (count [1 2 3 4])))

; String processing with data
(let [items ["apple" "banana" "cherry"]
      item_count (count items)
      first_item (first items)]
  (str "Shopping list has " item_count " items, starting with " first_item))

; Error handling in complex expressions
(try
  (let [numbers [1 2 3 4 5]
        operations (+ (first numbers) (count numbers))
        final_result (* operations 2)]
    (if (> final_result 10)
      final_result
      "too small"))
  "calculation failed")

; =============================================================================
; SECTION 13: Advanced Patterns and Edge Cases
; =============================================================================

; Empty and single-element operations
(+ )                             ; Empty addition
(*)                              ; Empty multiplication
(first [])                      ; Empty first
(rest [])                       ; Empty rest
(count [])                      ; Empty count

; Deeply nested expressions
(+ (+ (+ 1 2) (+ 3 4)) (+ (+ 5 6) (+ 7 8)))
(* (+ 1 (+ 2 3)) (- 10 (- 8 2)))

; Mixed data type operations (conceptual)
(str (+ 1 2) " plus " (+ 3 4) " equals " (+ (+ 1 2) (+ 3 4)))

; Boolean combinations with data
(and (> (count [1 2 3 4]) 2) (= (first [5 6 7]) 5))
(or (= (count []) 0) (> (first [10 20]) 5))

; Conditional data construction
(if (> (+ 2 3) 4)
    (cons 1 [2 3 4])
    [0])

; =============================================================================
; END OF COMPREHENSIVE DEMO
; =============================================================================

; This file demonstrates the full power and flexibility of BigLisp DSL
; Features covered:
; ✅ Arithmetic operations (all operators, multiple operands)
; ✅ Comparison operations (=, <, >)
; ✅ Boolean logic (and, or, not)
; ✅ Control flow (if, conditionals)
; ✅ Local bindings (let expressions)
; ✅ Data structures (vectors, lists)
; ✅ List operations (first, rest, count, cons)
; ✅ String operations (str concatenation)
; ✅ Function definitions (defn) and calls
; ✅ Sequential execution (do blocks)
; ✅ Loop constructs (dotimes)
; ✅ Error handling (try expressions)
; ✅ Print operations (println)
; ✅ Complex nested expressions
; ✅ Variable capture patterns
; ✅ Edge cases and advanced patterns

; To use this file:
; 1. Save as examples/comprehensive_demo.lisp
; 2. Run: biglisp run examples/comprehensive_demo.lisp
; 3. Or test individual expressions in REPL: biglisp repl
