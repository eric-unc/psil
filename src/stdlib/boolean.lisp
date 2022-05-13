(doc "!="
	(list)
	"`!=` checks for inequality between all rands."
	(table
    	"val1" "(any) the first value to be checked for inequality."
    	"val2" "(any) the second value to be checked for inequality."
    	"val3" "(OPTIONAL) (any) the third value to be checked for inequality. There may be more rands."))

(doc "<"
	(list)
	"`<` checks that the first rand is less than all other rands."
	(table
        "val1" "(number) the value to be checked that it is less than the others."
        "val2" "(number) the first value to be checked that it is greater than the first rand."
        "val3" "(OPTIONAL) (number) the second value to be checked that it is greater than the first rand. There may be more rands."))

(doc "<="
	(list)
	"`<=` checks that the first rand is less than or equal to all other rands."
	(table
        "val1" "(number) the value to be checked that it is less than or equal to the others."
        "val2" "(number) the first value to be checked that it is greater or equal to than the first rand."
        "val3" "(OPTIONAL) (number) the second value to be checked that it is greater than or equal to the first rand. There may be more rands."))

(doc "=="
	(list)
	"`==` checks for equality between all rands."
	(table
    	"val1" "(any) the first value to be checked for equality."
    	"val2" "(any) the second value to be checked for equality."
    	"val3" "(OPTIONAL) (any) the third value to be checked for equality. There may be more rands."))

(doc ">"
	(list)
	"`>` checks that the first rand is greater than all other rands."
	(table
        "val1" "(number) the value to be checked that it is greater than the others."
        "val2" "(number) the first value to be checked that it is less than the first rand."
        "val3" "(OPTIONAL) (number) the second value to be checked that it is less than the first rand. There may be more rands."))

(doc ">="
	(list)
	"`>=` checks that the first rand is greater than or equal to all other rands."
	(table
        "val1" "(number) the value to be checked that it is greater than or equal to the others."
        "val2" "(number) the first value to be checked that it is less than or equal to the first rand."
        "val3" "(OPTIONAL) (number) the second value to be checked that it is less than or equal to the first rand. There may be more rands."))

(doc "and"
	(list)
	"`and` ANDs each rand together. Always short-circuited.\n\nNote `and` is a special form."
	(table
		"bool1" "(boolean) the first boolean to be ANDed."
		"bool2" "(boolean) the second boolean to be ANDed."
		"bool3" "(OPTIONAL) (boolean) the third boolean to be ANDed. There may be more rands."))

(doc "is-bool?"
	(list)
	"`is-bool?` checks if the given rand is a boolean."
	(table
    	"val" "(any) the value to be checked if it is a boolean."))

(doc "not"
	(list)
	"`not` negates the given rand."
	(table
		"bool" "(boolean) the boolean to be negated."))

(doc "or"
	(list)
	"`or` ORs each rand together. Always short-circuited.\n\nNote `or` is a special form."
	(table
    	"bool1" "(boolean) the first boolean to be ORed."
    	"bool2" "(boolean) the second boolean to be ORed."
    	"bool3" "(OPTIONAL) (boolean) the third boolean to be ORed. There may be more rands."))

(doc "xor"
	(list)
	"`xor` XORs each rand together."
	(table
        "bool1" "(boolean) the first boolean to be XORed."
        "bool2" "(boolean) the second boolean to be XORed."
        "bool3" "(OPTIONAL) (boolean) the third boolean to be XORed. There may be more rands."))