# An Introduction to Psil
This brief guide is written in the style of ["Learn X in Y minutes"](https://learnxinyminutes.com/).

```lisp
;;; 0. Intro
; Comments are started with a semi-colon.
; Currently, there are no multiline comments.

; Psil is a Lisp-like programming language written in Rust.
; Nothing about it is too crazy, as long as you've programmed before.
; The command `psil` without any arguments will start a Psil REPL.
; `psil file.lisp` will load and run a particlar psil file.

;;; 1. Basic types
; We have numbers,
5 ; 5

; booleans,
true ; true
false ; false

; strings,
"Hello" ; Hello

; symbols,
#symbol ; #symbol

; and lastly...

;;; 2. Procedures
; The standard library includes many procedures, such as...
+ ; <procedure>
put ; <procedure>

; Procedures are first-class citizens, and are their own type, as you can see.
; But obviously, they are most useful in being directly invoked, as follows:
(+ 5 5) ; 10
(put "Hello World") ; Hello World

; Psil uses "S-expression" syntax for these invocations.
; An invocation has the following grammar:
; (proc arg1 arg2)
; An invocation starts and ends with matching parentheses.
; Like Polish notation, the procedure being invoked goes first, then the arguments.
; A procedure might have multiple arguments, or none at all.
; Arguments may have type restrictions as well.
; As usual, consult the documentation: https://eric-unc.tech/psil/

; Every procedure returns something.
; If there is nothing interesting to return, returning `#void` is conventional.
; You can combine invocations together to do interesting things, like this:
(put "5*5 + 8 is... " (+ (* 5 5) 8)) ; 5*5 + 8 is... 33

;;; 3. Special forms
; Special forms are special kinds of procedures, but more like "keywords" from other languages.
; They are as first-class, just like other precedures, which may be unusual from other backgrounds.
; As of the current version, Psil has the following special forms:
define ; <procedure>
cond ; <procedure>
if ; <procedure>
and ; <procedure>
or ; <procedure>

; Special forms generally have special parsing or execution.
```
