<!--
NOTE: This documentation is generated automatically!
Rather than editing this file, please update the associated file in stdlib!
Thanks, and have a good day!
-->
# control
This is the documentation for the `control` module.

---
## help

`help` shares the documentation for a given procedure or constant.

Parameters:
* `binding`: (string) the binding.

---
## doc

`doc` adds a new entry to the built-in documentation.

Parameters:
* `aliases`: (list of strings) all aliases for the procedure.
* `desc`: (string) a description of the procedure.
* `params`: (list) a list of the param, with descriptions of each.
* `proc`: (string) the procedure to be documented.

---
## fail

`fail` generates an error, with an optional error message. In REPL mode, Psil will just output the error message to standard error, and continue to accept input, while in load mode, Psil will terminate execution of the given script after the error message is printed.

Parameters:
* `message`: (OPTIONAL) (string) the error message.

---
## load

`load` loads and runs a given Psil script.

Parameters:
* `path`: (string) the path of the Psil script.

---
## type

`type` give the type of the given rand as a symbol.

Parameters:
* `rand`: (any) the given rand.

---
## exit

`exit` exits the program with a status code.

Parameters:
* `code`: (OPTIONAL) (integer) the exit code. Defaults is 0.

