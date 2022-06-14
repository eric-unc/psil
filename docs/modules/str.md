<!--
NOTE: This documentation is generated automatically!
Rather than editing this file, please update the associated file in stdlib!
Thanks, and have a good day!
-->
# str
This is the documentation for the `str` module.

---
## 2str
`2str` (string) converts any value to a string.

Parameters:
* `val`: (any) the value to be converted into a string.

---
## is-str?
`is-str?` (boolean) checks if the given rand is a string.

Parameters:
* `val`: (any) the value to be checked if it is a string.

---
## str-cat
`str-cat` (string) concatenates two (or more) strings (or other values) into one. If a rand is not a string, it will be converted into one.

Parameters:
* `val1`: (any) the first, or starting, string/value to be concatenated.
* `val2`: (any) the second string/value to be concatenated.
* `val3`: (OPTIONAL) (list) the third/value string to be concatenated. There may be more rands.

---
## str-contains?
`str-contains?` (boolean) checks if the second rand is contained in the first rand.

Parameters:
* `pattern`: (string) the pattern to be found.
* `str`: (string) the string to be searched.

---
## str-empty?
`str-empty?` (boolean) checks if the given string is the empty string (`""`).

Parameters:
* `str`: (string) the string to be checked.

