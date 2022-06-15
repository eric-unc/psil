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

---
## str-insert
`str-insert` (string) inserts the second string into the first at the given index.

Parameters:
* `index`: (natural number) the index of the first string where the second is inserted.
* `str1`: (string) the string to be insert into.
* `str2`: (string) the string that is inserted.

---
## str-len
`str-len` (natural number) returns the length of a given string.

Parameters:
* `str`: (string) the string.

---
## str-low
`str-low` (string) returns the lowercase version of the given string.

Parameters:
* `str`: (string) the string.

---
## str-repeat
`str-repeat` (string) returns the string repeated a certain number of times.

Parameters:
* `count`: (natural number) the number of times to repeat the string.
* `str`: (string) the string to be repeated.

---
## str-replace
`str-replace` (string) returns the first string with all instances of the second (the "old text") replaced by the third (the "new text").

Parameters:
* `new-text`: (string) the new text that the old text is replaced with
* `old-text`: (string) the old text to be replaced.
* `str`: (string) the string to be modified.

---
## str-starts-with?
`str-starts-with?` (boolean) checks if the first rand starts with the second rand.

Parameters:
* `str1`: (string) the string.
* `str2`: (string) the testing string.

---
## str-strip
`str-strip` (string) removes leading/trailing whitespace from the given string.

Parameters:
* `str`: (string) the string.

---
## str-trunc
`str-trunc` (string) shortens the given string to a given length.

Parameters:
* `new-length`: (natural number) the new length. Cannot be larger than the current length.
* `str`: (string) the string.

---
## str-up
`str-up` (string) returns the uppercase version of the given string.

Parameters:
* `str`: (string) the string.

