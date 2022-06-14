(doc "2str"
	(list)
	"`2str` (string) converts any value to a string."
	(table
		"val" "(any) the value to be converted into a string."))

(doc "is-str?"
	(list)
	"`is-str?` (boolean) checks if the given rand is a string."
	(table
    	"val" "(any) the value to be checked if it is a string."))

(doc "str-cat"
	(list)
	"`str-cat` (string) concatenates two (or more) strings (or other values) into one. If a rand is not a string, it will be converted into one."
	(table
		"val1" "(any) the first, or starting, string/value to be concatenated."
		"val2" "(any) the second string/value to be concatenated."
		"val3" "(OPTIONAL) (list) the third/value string to be concatenated. There may be more rands."))

(doc "str-contains?"
	(list)
	"`str-contains?` (boolean) checks if the second rand is contained in the first rand."
	(table
		"str" "(string) the string to be searched."
		"pattern" "(string) the pattern to be found."))

(doc "str-empty?"
	(list)
	"`str-empty?` (boolean) checks if the given string is the empty string (`\"\"`)."
	(table
		"str" "(string) the string to be checked."))
