(doc "put"
	(list)
	"`put` (`#void`) prints each rand concatenated, followed by a new line."
	(table
		"rand" "(OPTIONAL) (any) the rand to be printed out. There may be more rands."))

(doc "put-each"
	(list)
	"`put-each` (`#void`) puts each rand, each separated by a new line."
	(table
		"rand1" "(any) the first rand to be printed out."
		"rand2" "(OPTIONAL) (any) the second rand to be printed out. There may be more rands."))

(doc "print"
	(list)
	"`print` (`#void`) prints each rand, without any new lines."
	(table
		"rand1" "(any) the first rand to be printed out."
		"rand2" "(OPTIONAL) (any) the second rand to be printed out. There may be more rands."))

(doc "input"
	(list)
	"`input` (string) takes input from console. It has no rands."
	(table))
