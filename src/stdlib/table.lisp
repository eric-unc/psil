(doc "is-table?"
	(list)
	"`is-table?` (boolean) checks if the given rand is a table."
	(table
		"val" "(any) the value to be checked if it is a table."))

(doc "table"
	(list)
	"`table` (table) creates a table from the given rands."
	(table
		"key1" "(OPTIONAL) (any) the first key, mapped to the first value. There may be more keys, as long as there are corresponding values. Internally, the keys will converted into strings, so be careful; `1` and `\"1\"` might map to the same value."
		"val1" "(OPTIONAL) (any) the first value, as mapped by the first key. There may be more values, as long as there are corresponding keys"))
