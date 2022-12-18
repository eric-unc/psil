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

(doc "list-count"
	(list)
	"`list-count` (natural number) returns the number of items in a list equal to the given rands."
	(table
		"list" "(list) the list to be counted."
		"val1" "(any) the first value to be checked for."
		"val2" "(OPTIONAL) (any) the second value to be checked for. There may be more rands."))

(doc "list-each"
	(list)
	"`list-each` (`#void`) applies the value to the given procedure. Effectively the same as `list-map`, but ignores return values."
	(table
		"list" "(list) the list with each value that will become a rand."
		"proc" "(proc) the procedure to each value."))

(define list-eighth {|list| (list-get list 7)})
(define eighth list-eighth)
(doc "list-eighth"
	(list "eighth")
	"`list-eighth` (any) returns the eighth value of the given list."
	(table
		"list" "(list) the list."))

(doc "list-empty?"
	(list)
	"`list-empty?` (boolean) returns true if the list is empty."
	(table
		"list" "(list) the list."))

(define list-fifth {|list| (list-get list 4)})
(define fifth list-fifth)
(doc "list-fifth"
	(list "fifth")
	"`list-fifth` (any) returns the fifth value of the given list."
	(table
		"list" "(list) the list."))

(doc "list-filter"
	(list)
	"`list-filter` (list) filters the list down using the given procedure."
	(table
		"list" "(list) the list."
		"proc" "(proc) the procedure used for filtering. Must accept a singular value, and always return a boolean (true if the value should be included, false otherwise)."))

(doc "list-find"
	(list)
	"`list-find` (integer) returns the first index where the given value was found, or -1 if the value was not found."
	(table
		"list" "(list) the list."
		"val" "(any) the value to be found."))

(define list-first {|list| (list-get list 0)})
(define first list-first)
(doc "list-first"
	(list "first")
	"`list-first` (any) returns the first value of the given list."
	(table
		"list" "(list) the list."))

(doc "list-flatten"
	(list)
	"`list-flatten` (list) returns a flatten list. That is, each list inside the list is expanded in the new list produced, being replaced by its elements."
	(table
		"list" "(list) the list."
		"level" "(OPTIONAL) (integer) the level of recursion. Default is 1."))

(doc "list-fold"
	(list)
	"`list-fold` (any) combines all elements of a list into one cumulative value, given a procedure and a base value. With each iteration, the procedure is passed firstly the accumulator, then a value from the list."
	(table
		"list" "(list) the list."
		"proc" "(proc) the procedure used for accumulation. Must accept two arguments."
		"base" "(any) the initial value."))

(doc "list-foldr"
	(list)
	"`list-foldr` (any) is similar to `list-fold`, except the procedure is passed firstly a value from the list, and _then_ the accumulator."
	(table
		"list" "(list) the list."
		"proc" "(proc) the procedure used for accumulation. Must accept two arguments."
		"base" "(any) the initial value."))

(define list-fourth {|list| (list-get list 3)})
(define fourth list-fourth)
(doc "list-fourth"
	(list "fourth")
	"`list-fourth` (any) returns the fourth value of the given list."
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

(define list-ninth {|list| (list-get list 8)})
(define ninth list-ninth)
(doc "list-ninth"
	(list "ninth")
	"`list-ninth` (any) returns the ninth value of the given list."
	(table
		"list" "(list) the list."))

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

(define list-seventh {|list| (list-get list 6)})
(define seventh list-seventh)
(doc "list-seventh"
	(list "seventh")
	"`list-seventh` (any) returns the seventh value of the given list."
	(table
		"list" "(list) the list."))

(define list-sixth {|list| (list-get list 5)})
(define sixth list-sixth)
(doc "list-sixth"
	(list "sixth")
	"`list-sixth` (any) returns the sixth value of the given list."
	(table
		"list" "(list) the list."))

(doc "list-swap"
	(list)
	"`list-swap` (list) swaps two elements in the list."
	(table
		"list" "(list) the list."
		"index1" "(natural number) the first index."
		"index2" "(natural number) the second index."))

(define list-third {|list| (list-get list 2)})
(define third list-third)
(doc "list-third"
	(list "third")
	"`list-third` (any) returns the third value of the given list."
	(table
		"list" "(list) the list."))

(define list-tenth {|list| (list-get list 9)})
(define tenth list-tenth)
(doc "list-tenth"
	(list "tenth")
	"`list-tenth` (any) returns the tenth value of the given list."
	(table
		"list" "(list) the list."))
