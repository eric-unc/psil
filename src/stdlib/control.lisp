(doc "doc"
	(list)
	"`doc` adds a new entry to the built-in documentation."
	(table
		"proc" "(string) the procedure to be documented."
		"aliases" "(list of strings) all aliases for the procedure."
		"desc" "(string) a description of the procedure."
		"params" "(list) a list of the param, with descriptions of each."))

(doc "exit"
	(list)
	"`exit` exits the program with a status code."
	(table
		"code" "(OPTIONAL) (integer) the exit code. Defaults is 0."))

(doc "fail"
	(list)
	"`fail` generates an error, with an optional error message. In REPL mode, Psil will just output the error message to standard error, and continue to accept input, while in load mode, Psil will terminate execution of the given script after the error message is printed."
	(table
		"message" "(OPTIONAL) (string) the error message."))

(doc "help"
	(list)
	"`help` shares the documentation for a given procedure or constant."
	(table
		"binding" "(string) the binding."))

(doc "load"
	(list)
	"`load` loads and runs a given Psil script."
	(table
		"path" "(string) the path of the Psil script."))

(doc "type"
	(list)
	"`type` give the type of the given rand as a symbol."
	(table
		"rand" "(any) the given rand."))
