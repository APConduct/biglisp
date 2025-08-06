; Control Flow Examples for BigLisp
; This file demonstrates conditional logic, loops, and control structures

; Basic if expressions
(if (> 5 3) "five is greater" "five is not greater")
(if (< 2 10) 42 0)
(if (= 1 1) "equal" "not equal")

; If without else clause
(if (> 10 5) "condition is true")

; Nested conditionals
(if (> (+ 2 3) 4)
    (if (< 1 2) "nested true" "nested false")
    "outer false")

; Complex conditions with boolean logic
(if (and (> 5 3) (< 2 10)) "both conditions true" "at least one false")
(if (or (< 5 3) (> 10 5)) "at least one true" "both false")
(if (not (= 3 4)) "not equal" "equal")

; Let expressions for local bindings
(let [x 10] x)
(let [x 5 y 7] (+ x y))
(let [a (+ 2 3) b (* 2 4)] (+ a b))

; Nested let expressions
(let [x 10]
  (let [y 5]
    (+ x y)))

; Let with complex expressions
(let [multiplier 3
      numbers [1 2 3 4]
      first_num (first numbers)]
  (* multiplier first_num))

; Sequential execution with do
(do
  (+ 1 2)
  (* 3 4)
  (- 10 5))

; Do with mixed operations
(do
  (println "Starting calculation")
  (let [x 5 y 10] (+ x y))
  (str "Result calculated"))

; Function definitions
(defn square [x] (* x x))
(defn add_two [a b] (+ a b))
(defn complex_calc [x y z] (+ (* x y) z))

; Function calls (would be used with call in real implementation)
; (call square 5)
; (call add_two 3 7)
; (call complex_calc 2 3 4)

; Loop constructs
(dotimes i 5 (+ i 1))
(dotimes n 3 (* (+ n 1) 2))

; Error handling with try
(try (+ 1 2) 0)
(try (/ 10 2) "error occurred")
(try
  (let [x 5] (* x 2))
  "fallback value")

; Combining control flow features
(let [condition true
      x 10
      y 5]
  (if condition
    (do
      (+ x y)
      (* x y))
    (- x y)))

; Complex nested control flow
(if (> (count [1 2 3 4]) 2)
  (let [items [1 2 3 4]
        first_item (first items)
        rest_items (rest items)]
    (if (> first_item 0)
      (str "First item " first_item " is positive")
      (str "First item is not positive")))
  "Not enough items")
