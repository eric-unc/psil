(doc "and"
	(list)
	"`and` ANDs each rand together. Always short-circuited.\n\nNote `and` is a special form."
	(table
		"bool1" "(boolean) the first boolean to be ANDed."
		"bool2" "(boolean) the second boolean to be ANDed."
		"bool3" "(OPTIONAL) (boolean) the third boolean to be ANDed. There may be more rands."))

(doc "not"
	(list)
	"`not` negates the given rand."
	(table
		"bool" "(boolean) the boolean to be negated."))
