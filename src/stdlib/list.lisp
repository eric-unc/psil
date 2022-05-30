(doc "is-list?"
	(list)
	"`is-list?` checks if the given rand is a list."
	(table
    	"val" "(any) the value to be checked if it is a list."))

(doc "list"
	(list)
	"`list` creates a list from the given rands."
	(table
    	"val1" "(OPTIONAL) (any) the first value to be included in the list. There may be more rands."))

(doc "list-append"
	(list)
	"`list-append` returns a list with given rands appended."
	(table
		"list" "(list) the list to be appended to."
		"val1" "(any) the first value to be appended to the list."
		"val2" "(OPTIONAL) (any) the second value to be appended to the list. There may be more rands."))

(doc "list-each"
	(list)
	"`list-each` applies the value to the given procedure. Effectively the same as `list-map`, but ignores return values."
	(table
		"list" "(list) the list with each value that will become a rand."
		"proc" "(proc) the procedure to each value."))

(doc "list-empty?"
	(list)
	"`list-empty?` returns true if the list is empty."
	(table
		"list" "(list) the list."))

(doc "list-filter"
	(list)
	"`list-filter` filters the list down using the given procedure."
	(table
		"list" "(list) the list."
		"proc" "(proc) the procedure used for filtering. Must accept a singular value, and always return a boolean (true if the value should be included, false otherwise)."))

(define list-first {|list| (list-get list 0)})
(define first list-first)
(doc "list-first"
	(list "first")
	"`list-first` returns the first value of the given list."
	(table
		"list" "(list) the list."))

(doc "list-get"
	(list)
	"`list-get` gets an element from the list at a specific index."
	(table
		"list" "(list) the list."
		"index" "(natural number) the index."))

(doc "list-join"
	(list)
	"`list-join` combines two (or more) lists into one."
	(table
		"list1" "(list) the first list to be joined."
		"list2" "(list) the second list to be joined."
		"list3" "(OPTIONAL) (list) the third list to be joined. There may be more rands."

(doc "list-len"
	(list)
	"`list-len` returns the length of a given list."
	(table
		"list" "(list) the list."))

(doc "list-map"
	(list)
	"`list-map` returns a new list with values transformed by the given procedure."
	(table
		"list" "(list) the list with each value that will become a rand."
		"proc" "(proc) the procedure to each value."))

(define list-second {|list| (list-get list 1)})
