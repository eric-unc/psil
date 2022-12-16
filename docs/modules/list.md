<!--
NOTE: This documentation is generated automatically!
Rather than editing this file, please update the associated file in stdlib!
Thanks, and have a good day!
-->
# list
This is the documentation for the `list` module.

---
## is-list?
`is-list?` (boolean) checks if the given rand is a list.

Parameters:
* `val`: (any) the value to be checked if it is a list.

---
## list
`list` (list) creates a list from the given rands.

Parameters:
* `val1`: (OPTIONAL) (any) the first value to be included in the list. There may be more rands.

---
## list-append
`list-append` (list) returns a list with given rands appended.

Parameters:
* `list`: (list) the list to be appended to.
* `val1`: (any) the first value to be appended to the list.
* `val2`: (OPTIONAL) (any) the second value to be appended to the list. There may be more rands.

---
## list-count
`list-count` (natural number) returns the number of items in a list equal to the given rands.

Parameters:
* `list`: (list) the list to be counted.
* `val1`: (any) the first value to be checked for.
* `val2`: (OPTIONAL) (any) the second value to be checked for. There may be more rands.

---
## list-each
`list-each` (`#void`) applies the value to the given procedure. Effectively the same as `list-map`, but ignores return values.

Parameters:
* `list`: (list) the list with each value that will become a rand.
* `proc`: (proc) the procedure to each value.

---
## list-empty?
`list-empty?` (boolean) returns true if the list is empty.

Parameters:
* `list`: (list) the list.

---
## list-filter
`list-filter` (list) filters the list down using the given procedure.

Parameters:
* `list`: (list) the list.
* `proc`: (proc) the procedure used for filtering. Must accept a singular value, and always return a boolean (true if the value should be included, false otherwise).

---
## list-find
`list-find` (integer) returns the first index where the given value was found, or -1 if the value was not found.

Parameters:
* `list`: (list) the list.
* `val`: (any) the value to be found.

---
## list-first
`list-first` (any) returns the first value of the given list.

Parameters:
* `list`: (list) the list.

---
## list-flatten
`list-flatten` (list) returns a flatten list. That is, each list inside the list is expanded in the new list produced, being replaced by its elements.

Parameters:
* `level`: (OPTIONAL) (integer) the level of recursion. Default is 1.
* `list`: (list) the list.

---
## list-fold
`list-fold` (any) combines all elements of a list into one cumulative value, given a procedure and a base value. With each iteration, the procedure is passed firstly the accumulator, then a value from the list.

Parameters:
* `base`: (any) the initial value.
* `list`: (list) the list.
* `proc`: (proc) the procedure used for accumulation. Must accept two arguments.

---
## list-foldr
`list-foldr` (any) is similar to `list-fold`, except the procedure is passed firstly a value from the list, and _then_ the accumulator.

Parameters:
* `base`: (any) the initial value.
* `list`: (list) the list.
* `proc`: (proc) the procedure used for accumulation. Must accept two arguments.

---
## list-get
`list-get` (any) gets an element from the list at a specific index.

Parameters:
* `index`: (natural number) the index.
* `list`: (list) the list.

---
## list-join
`list-join` (list) combines two (or more) lists into one.

Parameters:
* `list1`: (list) the first list to be joined.
* `list2`: (list) the second list to be joined.
* `list3`: (OPTIONAL) (list) the third list to be joined. There may be more rands.

---
## list-len
`list-len` (natural number) returns the length of a given list.

Parameters:
* `list`: (list) the list.

---
## list-map
`list-map` (list) returns a new list with values transformed by the given procedure.

Parameters:
* `list`: (list) the list with each value that will become a rand.
* `proc`: (proc) the procedure to each value.

---
## list-range
`list-range` (list) creates a list of numbers from a given range, with an optional step-size.

Parameters:
* `end`: (integer) the end of the range. Obviously, must be larger than the start
* `start`: (integer) the start of the range.
* `step-size`: (OPTIONAL) (integer) an alternative step size to choose from (the default is 1)

---
## list-remove
`list-remove` (list) removes an element from the list at a specific index.

Parameters:
* `index`: (natural number) the index.
* `list`: (list) the list.

---
## list-reverse
`list-reverse` (list) returns a reversed list.

Parameters:
* `list`: (list) the list.

---
## list-second
`list-second` (any) returns the second value of the given list.

Parameters:
* `list`: (list) the list.

---
## list-swap
`list-swap` (list) swaps two elements in the list.

Parameters:
* `index1`: (natural number) the first index.
* `index2`: (natural number) the second index.
* `list`: (list) the list.

---
## list-third
`list-third` (any) returns the third value of the given list.

Parameters:
* `list`: (list) the list.

