(doc "is-list?"
	(list)
	"`is-list?` checks if the given rand is a list."
	(table
    	"val" "(any) the value to be checked if it is a list."))

(doc "list"
	(list)
	"`list` creates a list from the given rands"
	(table
    	"val1" "(OPTIONAL) (any) the first value to be included in the list. There may be more rands."))

(define list-first {|list| (list-get list 0)})

(define list-second {|list| (list-get list 1)})
