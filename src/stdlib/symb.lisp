(doc "str2symb"
	(list)
	"`str2symb` (symbol) converts a given string to a symbol."
	(table
		"str" "(string) the string."))

(doc "symb2str"
	(list)
	"`symb2str` (string) converts a given symbol to a string."
	(table
		"symb" "(symbol) the string."))

(doc "is-symb?"
	(list)
	"`is-symb?` (boolean) checks if the rand is a symbol."
	(table
		"val" "(any) the value to be checked if it is a symbol."))

; TODO: obviously this can be pure
(doc "is-void?"
	(list)
	"`is-void?` (boolean) checks if the rand is the `#void` symbol."
	(table
		"val" "(any) the value to be checked if it is `#void`."))
