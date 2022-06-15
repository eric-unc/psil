<!--
NOTE: This documentation is generated automatically!
Rather than editing this file, please update the associated file in stdlib!
Thanks, and have a good day!
-->
# control
This is the documentation for the `control` module.

---
## cond
`cond` (any) returns the evaluated expression associated with the true condition, or `#void` if none are true. See `cond.lisp` or `fizzbuzz.lisp` in the `samples` folder for examples.

Note `cond` is a special form.

Parameters:
* `cond1`: (boolean) the first conditional to be considered.
* `cond2`: (OPTIONAL) (boolean) the second conditional to be considered.
* `t1`: (any) the expression to return/evaluate if the first conditional is true.
* `t2`: (OPTIONAL) (any) the expression to return/evaluate if the second conditional is true. There may be more pairs of conditionals and associated expressions.

---
## define
`define` (`#void`) creates a binding with the name given in the current scope.

Note `define` is a special form.

Parameters:
* `name`: (name) the name of the binding.
* `value`: (any) the value to bind to that name.

---
## do
`do` (`#void`) executes each invocation/expression given.

Note `do` is a special form.

Parameters:
* `expr1`: (any) the first expression to be evaluated
* `expr2`: (OPTIONAL) (any) the second expression to be evaluated. There may be more rands.

---
## doc
`doc` (`#void`) adds a new entry to the built-in documentation.

Parameters:
* `aliases`: (list of strings) all aliases for the procedure.
* `desc`: (string) a description of the procedure.
* `params`: (list) a list of the param, with descriptions of each.
* `proc`: (string) the procedure to be documented.

---
## exit
`exit` (N/A) exits the program with a status code.

Parameters:
* `code`: (OPTIONAL) (integer) the exit code. Defaults is 0.

---
## fail
`fail` (`#void`) generates an error, with an optional error message. In REPL mode, Psil will just output the error message to standard error, and continue to accept input, while in load mode, Psil will terminate execution of the given script after the error message is printed.

Parameters:
* `message`: (OPTIONAL) (string) the error message.

---
## help
`help` (`#void`) shares the documentation for a given procedure or constant.

Parameters:
* `binding`: (string) the binding.

---
## if
`if` (any) returns and evaluates one expression if the given condition is true, and the other if false. The other expression within will not be evaluated.

Note `if` is a special form.

Parameters:
* `cond`: (boolean) the conditional.
* `f`: (any) the expression to return/evaluate if false.
* `t`: (any) the expression to return/evaluate if true.

---
## load
`load` (`#void`) loads and runs a given Psil script.

Parameters:
* `path`: (string) the path of the Psil script.

---
## type
`type` (symbol) give the type of the given rand as a symbol.

Parameters:
* `rand`: (any) the given rand.

