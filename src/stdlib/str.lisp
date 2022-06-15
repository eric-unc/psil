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

(doc "str-insert"
	(list)
	"`str-insert` (string) inserts the second string into the first at the given index."
	(table
		"str1" "(string) the string to be insert into."
		"index" "(natural number) the index of the first string where the second is inserted."
		"str2" "(string) the string that is inserted."))

(doc "str-len"
	(list)
	"`str-len` (natural number) returns the length of a given string."
	(table
		"str" "(string) the string."))

(doc "str-low"
	(list)
	"`str-low` (string) returns the lowercase version of the given string."
	(table
		"str" "(string) the string."))

(doc "str-repeat"
	(list)
	"`str-repeat` (string) returns the string repeated a certain number of times."
	(table
		"str" "(string) the string to be repeated."
		"count" "(natural number) the number of times to repeat the string."))

(doc "str-replace"
	(list)
	"`str-replace` (string) returns the first string with all instances of the second (the \"old text\") replaced by the third (the \"new text\")."
	(table
		"str" "(string) the string to be modified."
		"old-text" "(string) the old text to be replaced."
		"new-text" "(string) the new text that the old text is replaced with"))

(doc "str-starts-with?"
	(list)
	"`str-starts-with?` (boolean) checks if the first rand starts with the second rand."
	(table
		"str1" "(string) the string."
		"str2" "(string) the testing string."))

(doc "str-strip"
	(list)
	"`str-strip` (string) removes leading/trailing whitespace from the given string."
	(table
		"str" "(string) the string."))

(doc "str-trunc"
	(list)
	"`str-trunc` (string) shortens the given string to a given length."
	(table
		"str" "(string) the string."
		"new-length" "(natural number) the new length. Cannot be larger than the current length."))

(doc "str-up"
	(list)
	"`str-up` (string) returns the uppercase version of the given string."
	(table
		"str" "(string) the string."))
