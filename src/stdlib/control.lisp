(doc "cond"
	(list)
	"`cond` returns the evaluated expression associated with the true condition, or `#void` if none are true. See `cond.lisp` or `fizzbuzz.lisp` in the `samples` folder for examples.\n\nNote `cond` is a special form."
	(table
		"cond1" "(boolean) the first conditional to be considered."
		"t1" "(any) the expression to return/evaluate if the first conditional is true."
		"cond2" "(OPTIONAL) (boolean) the second conditional to be considered."
		"t2" "(OPTIONAL) (any) the expression to return/evaluate if the second conditional is true. There may be more pairs of conditionals and associated expressions."))

(doc "define"
	(list)
	"`define` creates a binding with the name given in the current scope.\n\nNote `define` is a special form."
	(table
		"name" "(name) the name of the binding."
		"value" "(any) the value to bind to that name."))

(doc "do"
	(list)
	"`do` executes each invocation/expression given.\n\nNote `do` is a special form."
	(table
		"expr1" "(any) the first expression to be evaluated"
		"expr2" "(OPTIONAL) (any) the second expression to be evaluated. There may be more rands."))

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

(doc "if"
	(list)
	"`if` returns and evaluates one expression if the given condition is true, and the other if false. The other expression within will not be evaluated.\n\nNote `if` is a special form."
	(table
		"cond" "(boolean) the conditional."
		"t" "(any) the expression to return/evaluate if true."
		"f" "(any) the expression to return/evaluate if false."))

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
