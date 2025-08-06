; Data Structures Examples for BigLisp
; This file demonstrates working with vectors, lists, and data manipulation

; Creating vectors/lists
[1 2 3 4 5]
[10 20 30]
["hello" "world" "biglisp"]
[true false true]

; Empty vectors
[]

; Mixed type vectors (in a real implementation)
; [1 "hello" true 3.14]

; List operations - first element
(first [1 2 3 4])
(first [42])
(first ["a" "b" "c"])

; List operations - rest (all but first)
(rest [1 2 3 4])
(rest [42])
(rest ["a" "b" "c"])

; List operations - count/length
(count [1 2 3 4 5])
(count [42])
(count [])
(count ["hello" "world"])

; Prepending elements with cons
(cons 0 [1 2 3])
(cons "start" ["middle" "end"])
(cons 42 [])

; Nested data structures
[[1 2] [3 4] [5 6]]
[1 [2 3] 4]
[[["nested"] ["deeply"]] ["structure"]]

; Operations on nested structures
(first [[1 2] [3 4]])
(rest [[1 2] [3 4] [5 6]])
(count [[1 2] [3 4] [5 6]])

; Complex data manipulation
(let [numbers [1 2 3 4 5]
      first_num (first numbers)
      rest_nums (rest numbers)
      total_count (count numbers)]
  (+ first_num total_count))

; Building lists incrementally
(cons 1 (cons 2 (cons 3 [])))

; Working with strings as data
(str "Item count: " (count [1 2 3 4]))
(str "First element: " (first [10 20 30]))

; Conditional operations on data
(if (> (count [1 2 3]) 2)
  (first [1 2 3])
  0)

; Data transformations in let bindings
(let [original [1 2 3 4]
      without_first (rest original)
      with_zero (cons 0 original)
      size (count original)]
  (+ size (first with_zero)))

; Combining data operations
(count (rest (cons 0 [1 2 3 4])))

; Boolean operations on data predicates
(and (> (count [1 2 3]) 0) (= (first [5 6 7]) 5))
(or (= (count []) 0) (> (count [1 2]) 5))

; Data-driven conditionals
(if (= (count [1 2 3]) 3)
  (str "List has " (count [1 2 3]) " elements")
  "Unexpected count")

; Working with data in functions
(defn get_info [lst]
  (let [size (count lst)
        first_elem (first lst)]
    (str "Size: " size ", First: " first_elem)))

; Function that works with list structure
(defn process_list [data]
  (if (> (count data) 0)
    (+ (first data) (count (rest data)))
    0))

; Nested list processing
(let [matrix [[1 2] [3 4] [5 6]]
      first_row (first matrix)
      first_element (first first_row)
      row_count (count matrix)]
  (+ first_element row_count))

; Sequential data operations
(do
  [1 2 3]
  (cons 0 [1 2 3])
  (count [0 1 2 3]))

; Error handling with data operations
(try (first [1 2 3]) 0)
(try (rest []) [])
(try (count [1 2 3 4 5]) 0)

; Complex data structure example
(let [data [[1 2 3] [4 5 6] [7 8 9]]
      first_sublist (first data)
      rest_sublists (rest data)
      total_sublists (count data)
      first_of_first (first first_sublist)]
  (if (> total_sublists 2)
    (str "Matrix " total_sublists "x" (count first_sublist)
         " starting with " first_of_first)
    "Small matrix"))

; Building complex structures
(cons [1 2] (cons [3 4] []))
(cons
  (cons 1 [2 3])
  (cons
    (cons 4 [5 6])
    []))
