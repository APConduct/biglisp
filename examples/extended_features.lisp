; Extended Features Examples for BigLisp
; This file demonstrates the new operators and utility functions added to BigLisp

; =============================================================================
; SECTION 1: Extended Comparison Operators
; =============================================================================

; Greater than or equal (gte)
(gte 10 5)                       ; Greater than or equal: true
(gte 5 5)                        ; Equal case: true
(gte 3 7)                        ; Less than case: false

; Less than or equal (lte)
(lte 3 7)                        ; Less than or equal: true
(lte 5 5)                        ; Equal case: true
(lte 10 5)                       ; Greater than case: false

; Not equal (ne)
(ne 5 3)                         ; Not equal: true
(ne 7 7)                         ; Equal case: false
(ne "hello" "world")             ; String comparison: true

; Complex expressions with new operators
(and (gte 10 5) (lte 3 7))       ; Both conditions true
(or (ne 5 5) (gte 8 3))          ; Mixed conditions
(not (lte 10 5))                 ; Negated comparison

; =============================================================================
; SECTION 2: Math Utility Functions
; =============================================================================

; Minimum values
(min 5 3)                        ; Two arguments: 3
(min 10 2 8 1 9)                 ; Multiple arguments: 1
(min (+ 2 3) (* 2 4))            ; With expressions: 5

; Maximum values
(max 5 3)                        ; Two arguments: 5
(max 10 2 8 15 9)                ; Multiple arguments: 15
(max (- 10 3) (/ 20 4))          ; With expressions: 7

; Absolute values
(abs 5)                          ; Positive number: 5
(abs (- 0 7))                    ; Negative number: 7
(abs 0)                          ; Zero: 0

; Modulo operations
(% 10 3)                         ; Standard modulo: 1
(modulo 15 4)                    ; Named modulo: 3
(% 20 5)                         ; Even division: 0

; Increment and decrement
(inc 5)                          ; Increment: 6
(inc 0)                          ; From zero: 1
(dec 10)                         ; Decrement: 9
(dec 1)                          ; To zero: 0

; =============================================================================
; SECTION 3: Predicate Functions
; =============================================================================

; Zero checking
(zero 0)                         ; Is zero: true
(zero 5)                         ; Not zero: false
(zero (- 5 5))                   ; Expression result: true

; Positive checking
(pos 5)                          ; Positive: true
(pos 0)                          ; Zero: false
(pos (- 0 3))                    ; Negative: false

; Negative checking
(neg (- 0 5))                    ; Negative: true
(neg 0)                          ; Zero: false
(neg 3)                          ; Positive: false

; Even checking
(even 4)                         ; Even: true
(even 5)                         ; Odd: false
(even 0)                         ; Zero is even: true

; Odd checking
(odd 3)                          ; Odd: true
(odd 4)                          ; Even: false
(odd 1)                          ; One is odd: true

; =============================================================================
; SECTION 4: Complex Combinations
; =============================================================================

; Mathematical sequences
(min (max 5 3) (abs (- 0 8)))    ; Nested min/max/abs: 5
(+ (inc 5) (dec 10))             ; Increment and decrement: 15
(% (+ 10 5) (- 8 3))             ; Modulo with expressions: 0

; Conditional logic with new operators
(if (gte 10 5) (max 1 2 3) (min 1 2 3))     ; Conditional math: 3
(if (even 6) (inc 10) (dec 10))             ; Even/odd branching: 11
(if (zero (% 15 3)) "divisible" "remainder") ; Divisibility check

; Predicate combinations
(and (pos 5) (even 4))           ; Multiple predicates: true
(or (zero 0) (neg 3))            ; Mixed predicates: true
(not (and (odd 4) (pos (- 0 3)))) ; Complex negation: true

; Variable-style usage (for demonstration)
; Note: These would work with variable capture in actual Rust code
; let x = 15, y = 4, z = 0
; (and (gte x 10) (ne (% x y) z))  ; x >= 10 and x % y != 0

; =============================================================================
; SECTION 5: Practical Use Cases
; =============================================================================

; Range checking
(and (gte 25 18) (lte 25 65))    ; Age in working range: true
(and (gte 15 1) (lte 15 12))     ; Month validation: false

; Mathematical properties
(and (even 8) (pos 8))           ; Positive even number: true
(and (odd 7) (ne 7 0))           ; Non-zero odd number: true

; Bounds checking
(or (lte 5 0) (gte 5 100))       ; Outside bounds 0-100: false
(and (gte 50 0) (lte 50 100))    ; Within bounds 0-100: true

; Calculation helpers
(abs (- 10 15))                  ; Distance between numbers: 5
(min (abs (- 10 7)) (abs (- 10 13))) ; Closer distance: 3
(max (inc 5) (dec 8))            ; Maximum adjusted value: 7

; =============================================================================
; SECTION 6: Advanced Examples
; =============================================================================

; Nested mathematical operations
(+ (min 10 5) (max 3 7) (abs (- 0 2))) ; Complex calculation: 14
(* (inc (max 2 4)) (dec (min 8 6)))     ; Chained operations: 25

; Complex conditionals
(if (and (even 6) (gte 6 4))
    (+ (inc 5) (abs (- 0 3)))
    (- (dec 10) (min 2 4)))      ; Multi-step conditional: 9

; Predicate chains
(and (pos (abs (- 0 5)))         ; abs(-5) > 0: true
     (even (+ 2 4))              ; 2 + 4 is even: true
     (ne (% 7 3) 0))             ; 7 % 3 != 0: true

; Mathematical validation
(and (gte 100 50)                ; Range check
     (zero (% 100 10))           ; Divisibility check
     (even (/ 100 10)))          ; Result property check

; =============================================================================
; SUMMARY OF NEW FEATURES
; =============================================================================

; Extended Comparison Operators:
; - gte (>=)  : Greater than or equal
; - lte (<=)  : Less than or equal
; - ne (!=)   : Not equal

; Math Utility Functions:
; - min       : Minimum of multiple values
; - max       : Maximum of multiple values
; - abs       : Absolute value
; - % / modulo: Modulo operation
; - inc       : Increment by 1
; - dec       : Decrement by 1

; Predicate Functions:
; - zero      : Check if value is zero
; - pos       : Check if value is positive
; - neg       : Check if value is negative
; - even      : Check if value is even
; - odd       : Check if value is odd

; All functions work seamlessly with:
; - Variable capture: lisp!([x, y] (gte x y))
; - Complex expressions: (min (+ 1 2) (* 3 4))
; - Conditional logic: (if (even x) (inc y) (dec y))
; - Nested operations: (abs (min -5 -10))

; These extensions make BigLisp more powerful for:
; - Mathematical computations
; - Data validation
; - Conditional logic
; - Range checking
; - Property testing
