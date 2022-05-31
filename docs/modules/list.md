<!--
NOTE: This documentation is generated automatically!
Rather than editing this file, please update the associated file in stdlib!
Thanks, and have a good day!
-->
# list
This is the documentation for the `list` module.

---
## is-list?
`is-list?` checks if the given rand is a list.

Parameters:
* `val`: (any) the value to be checked if it is a list.

---
## list
`list` creates a list from the given rands.

Parameters:
* `val1`: (OPTIONAL) (any) the first value to be included in the list. There may be more rands.

---
## list-append
`list-append` returns a list with given rands appended.

Parameters:
* `list`: (list) the list to be appended to.
* `val1`: (any) the first value to be appended to the list.
* `val2`: (OPTIONAL) (any) the second value to be appended to the list. There may be more rands.

---
## list-each
`list-each` applies the value to the given procedure. Effectively the same as `list-map`, but ignores return values.

Parameters:
* `list`: (list) the list with each value that will become a rand.
* `proc`: (proc) the procedure to each value.

---
## list-empty?
`list-empty?` returns true if the list is empty.

Parameters:
* `list`: (list) the list.

---
## list-filter
`list-filter` filters the list down using the given procedure.

Parameters:
* `list`: (list) the list.
* `proc`: (proc) the procedure used for filtering. Must accept a singular value, and always return a boolean (true if the value should be included, false otherwise).

---
## list-first
`list-first` returns the first value of the given list.

Parameters:
* `list`: (list) the list.

---
## list-get
`list-get` gets an element from the list at a specific index.

Parameters:
* `index`: (natural number) the index.
* `list`: (list) the list.

---
## list-join
`list-join` combines two (or more) lists into one.

Parameters:
* `list1`: (list) the first list to be joined.
* `list2`: (list) the second list to be joined.
* `list3`: (OPTIONAL) (list) the third list to be joined. There may be more rands.

---
## list-len
`list-len` returns the length of a given list.

Parameters:
* `list`: (list) the list.

---
## list-map
`list-map` returns a new list with values transformed by the given procedure.

Parameters:
* `list`: (list) the list with each value that will become a rand.
* `proc`: (proc) the procedure to each value.

