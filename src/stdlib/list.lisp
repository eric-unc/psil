(doc "is-list?"
	(list)
	"`is-list?` (boolean) checks if the given rand is a list."
	(table
    	"val" "(any) the value to be checked if it is a list."))

(doc "list"
	(list)
	"`list` (list) creates a list from the given rands."
	(table
    	"val1" "(OPTIONAL) (any) the first value to be included in the list. There may be more rands."))

(doc "list-append"
	(list)
	"`list-append` (list) returns a list with given rands appended."
	(table
		"list" "(list) the list to be appended to."
		"val1" "(any) the first value to be appended to the list."
		"val2" "(OPTIONAL) (any) the second value to be appended to the list. There may be more rands."))

(doc "list-each"
	(list)
	"`list-each` (`#void`) applies the value to the given procedure. Effectively the same as `list-map`, but ignores return values."
	(table
		"list" "(list) the list with each value that will become a rand."
		"proc" "(proc) the procedure to each value."))

(doc "list-empty?"
	(list)
	"`list-empty?` (boolean) returns true if the list is empty."
	(table
		"list" "(list) the list."))

(doc "list-filter"
	(list)
	"`list-filter` (list) filters the list down using the given procedure."
	(table
		"list" "(list) the list."
		"proc" "(proc) the procedure used for filtering. Must accept a singular value, and always return a boolean (true if the value should be included, false otherwise)."))

(define list-first {|list| (list-get list 0)})
(define first list-first)
(doc "list-first"
	(list "first")
	"`list-first` (any) returns the first value of the given list."
	(table
		"list" "(list) the list."))

(doc "list-get"
	(list)
	"`list-get` (any) gets an element from the list at a specific index."
	(table
		"list" "(list) the list."
		"index" "(natural number) the index."))

(doc "list-join"
	(list)
	"`list-join` (list) combines two (or more) lists into one."
	(table
		"list1" "(list) the first list to be joined."
		"list2" "(list) the second list to be joined."
		"list3" "(OPTIONAL) (list) the third list to be joined. There may be more rands."))

(doc "list-len"
	(list)
	"`list-len` (natural number) returns the length of a given list."
	(table
		"list" "(list) the list."))

(doc "list-map"
	(list)
	"`list-map` (list) returns a new list with values transformed by the given procedure."
	(table
		"list" "(list) the list with each value that will become a rand."
		"proc" "(proc) the procedure to each value."))

(doc "list-range"
	(list)
	"`list-range` (list) creates a list of numbers from a given range, with an optional step-size."
	(table
		"start" "(integer) the start of the range."
		"end" "(integer) the end of the range. Obviously, must be larger than the start"
		"step-size" "(OPTIONAL) (integer) an alternative step size to choose from (the default is 1)"))

(doc "list-remove"
	(list)
	"`list-remove` (list) removes an element from the list at a specific index."
	(table
		"list" "(list) the list."
		"index" "(natural number) the index."))

(doc "list-reverse"
	(list)
	"`list-reverse` (list) returns a reversed list."
	(table
		"list" "(list) the list."))

(define list-second {|list| (list-get list 1)})
(define second list-second)
(doc "list-second"
	(list "second")
	"`list-second` (any) returns the second value of the given list."
	(table
		"list" "(list) the list."))

(doc "list-swap"
	(list)
	"`list-swap` (list) swaps two elements in the list."
	(table
		"list" "(list) the list."
		"index1" "(natural number) the first index."
		"index2" "(natural number) the second index."))
