(doc "doc"
	(list)
	"`doc` adds a new entry to the built-in documentation."
	(table
		"proc" "(string) The procedure to be documented."
		"aliases" "(list of strings) All aliases for the procedure"
		"desc" "(string) A description of the procedure"
		"params" "(list) A list of the param, with descriptions of each."))

(doc "exit"
	(list)
	"`exit` exits the program with a status code."
	(table
		"code" "(OPTIONAL) (integer) exit code. Defaults is 0."))
