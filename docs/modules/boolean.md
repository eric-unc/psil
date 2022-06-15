<!--
NOTE: This documentation is generated automatically!
Rather than editing this file, please update the associated file in stdlib!
Thanks, and have a good day!
-->
# boolean
This is the documentation for the `boolean` module.

---
## !=
`!=` (boolean) checks for inequality between all rands.

Parameters:
* `val1`: (any) the first value to be checked for inequality.
* `val2`: (any) the second value to be checked for inequality.
* `val3`: (OPTIONAL) (any) the third value to be checked for inequality. There may be more rands.

---
## <
`<` (boolean) checks that the first rand is less than all other rands.

Parameters:
* `val1`: (number) the value to be checked that it is less than the others.
* `val2`: (number) the first value to be checked that it is greater than the first rand.
* `val3`: (OPTIONAL) (number) the second value to be checked that it is greater than the first rand. There may be more rands.

---
## <=
`<=` (boolean) checks that the first rand is less than or equal to all other rands.

Parameters:
* `val1`: (number) the value to be checked that it is less than or equal to the others.
* `val2`: (number) the first value to be checked that it is greater or equal to than the first rand.
* `val3`: (OPTIONAL) (number) the second value to be checked that it is greater than or equal to the first rand. There may be more rands.

---
## ==
`==` (boolean) checks for equality between all rands.

Parameters:
* `val1`: (any) the first value to be checked for equality.
* `val2`: (any) the second value to be checked for equality.
* `val3`: (OPTIONAL) (any) the third value to be checked for equality. There may be more rands.

---
## >
`>` (boolean) checks that the first rand is greater than all other rands.

Parameters:
* `val1`: (number) the value to be checked that it is greater than the others.
* `val2`: (number) the first value to be checked that it is less than the first rand.
* `val3`: (OPTIONAL) (number) the second value to be checked that it is less than the first rand. There may be more rands.

---
## >=
`>=` (boolean) checks that the first rand is greater than or equal to all other rands.

Parameters:
* `val1`: (number) the value to be checked that it is greater than or equal to the others.
* `val2`: (number) the first value to be checked that it is less than or equal to the first rand.
* `val3`: (OPTIONAL) (number) the second value to be checked that it is less than or equal to the first rand. There may be more rands.

---
## and
`and` (boolean) ANDs each rand together. Always short-circuited.

Note `and` is a special form.

Parameters:
* `bool1`: (boolean) the first boolean to be ANDed.
* `bool2`: (boolean) the second boolean to be ANDed.
* `bool3`: (OPTIONAL) (boolean) the third boolean to be ANDed. There may be more rands.

---
## is-bool?
`is-bool?` (boolean) checks if the given rand is a boolean.

Parameters:
* `val`: (any) the value to be checked if it is a boolean.

---
## not
`not` (boolean) negates the given rand.

Parameters:
* `bool`: (boolean) the boolean to be negated.

---
## or
`or` (boolean) ORs each rand together. Always short-circuited.

Note `or` is a special form.

Parameters:
* `bool1`: (boolean) the first boolean to be ORed.
* `bool2`: (boolean) the second boolean to be ORed.
* `bool3`: (OPTIONAL) (boolean) the third boolean to be ORed. There may be more rands.

---
## xor
`xor` (boolean) XORs each rand together.

Parameters:
* `bool1`: (boolean) the first boolean to be XORed.
* `bool2`: (boolean) the second boolean to be XORed.
* `bool3`: (OPTIONAL) (boolean) the third boolean to be XORed. There may be more rands.

