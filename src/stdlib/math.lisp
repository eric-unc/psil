(doc "+"
	(list)
	"`+` adds all rands given."
	(table
		"val1" "(number) the first value to be added."
		"val2" "(number) the second value to be added."
		"val3" "(OPTIONAL) (number) the third value to be added. There may be more rands."))

(doc "-"
	(list)
	"`-` subtracts the first rand by the remaining rands."
	(table
		"val1" "(number) the value that is subtracted from (the \"minuend\")."
		"val2" "(number) the first value to be subtracted by (or the first \"subtrahend\")."
		"val3" "(OPTIONAL) (number) the second value to be subtracted by (or second \"subtrahend\"). There may be more rands."))

(doc "*"
	(list)
	"`*` multiplies all rands given."
	(table
		"val1" "(number) the first value to be multiplied."
		"val2" "(number) the second value to be multiplied."
		"val3" "(OPTIONAL) (number) the third value to be multiplied. There may be more rands."))

(doc "/"
	(list)
	"`/` divides the first rand by the remaining rands."
	(table
		"val1" "(number) the value that is divided from (the \"dividend\" or \"numerator\")."
		"val2" "(number) the first value to divide by (or the first \"divisor\" or \"denominator\")."
		"val3" "(OPTIONAL) (number) the second value to divide by (or the second \"divisor\" or \"denominator\"). There may be more rands."))

(doc "%"
	(list)
	"`%` divides the first rand by the remaining rands, and gives the remainder. It is the modulo operator."
	(table
		"val1" "(number) the value that is modulo from (the \"dividend\" or \"numerator\")."
		"val2" "(number) the first value to by modulo by (or the first \"divisor\" or \"denominator\")."
		"val3" "(OPTIONAL) (number) the second value to modulo by (or the second \"divisor\" or \"denominator\"). There may be more rands."))

(doc "is-num?"
	(list)
	"`is-num?` checks if the given rand is a number."
	(table
    	"val" "(any) the value to be checked if it is a number."))