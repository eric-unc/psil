# Psil
**Psil** is a Lisp-like programming language implemented in Rust. It's currently a work-in-progress, although basic functionality works.

[Limp](https://github.com/eric-unc/limp) was the prototype for Psil and uses code from it.

## Installation
Psil can be installed by doing the following:
1. Cloning this repo (`git clone https://github.com/eric-unc/psil.git`).
2. Entering the directory (`cd psil`) and installing through Cargo (`cargo install --path .`).

## Running
Psil's command line interface has two modes: REPL and file loading:
* REPL mode: With no argument, Psil will take continually read input from the user and evaluate it as an expression, and then print the resulting value.
* Load mode: With a single argument, it will load the file and run it. Conventionally, Psil files can end in `.psil` or `.lisp`. This repository uses `.lisp` so that GitHub recognizes it as code.

## Language
Psil is a Lisp-like programming languages, with some twists.

Psil has two major constructions. The first is the "atom", which is a singular unit of value (such as an integer, or an identifier). The second is the "invocation," which is the activation of a procedure (must be a name type), and a series of expressions. The activated procedure is the "rator" (operator), and the passed expressions are the "rands" (operands). The rands are always resolved before the effect of the rator applies, except in the case of "special forms." Invocations will always return a value, even if that value is the `#void` symbol.

### Example
Other examples may be found in the `samples` directory.
```lisp
(put "Hello, World!")

(put) ; extra new line

; approximation for x near 0
(define sin {|x| x})

(put "What's the sin of 0?")

(define num (sin 0))

(if (== num 0)
	(put "Yay, sin(0) is 0!")
	(fail "How can sin(0) not be 0???"))
```

#### Example output
```
Hello, World!

What's the sin of 0?
Yay, sin(0) is 0!
```

### Types
| Type | Description |
| :------ | :------ |
| number | Numbers are float-point values, such as `3`, `-55`, `0.55`. |
| boolean | Booleans are truth values, which can either be `true` or `false`. |
| string | Strings are a series of characters, like `"Ahhh!"` or `"545"`. |
| symbol | Symbols are similar to strings, but more for internal use as human-readable identifiers. They can be used like enums, like `#north`, `#south`, `#east`, `#west`. One important symbol is `#void`, a singleton returned from functions that don't return anything interesting. |
| procedure | A procedure (or "proc") is a block that returns an atom with optional arguments. See `procs.lisp` in the `samples` directory for examples. Procedures can be invoked easily if defined. |
| list | A list is a composite value of values. It can be created through the `list` procedure. |

### Built-in procedures
This lists all procedures built into the standard library. Special forms are marked with a `*` next to their names. Special forms are procedures too, but with special evaluation rules that don't allow them to be implemented as regular procedures.

#### Control
| Name | Rands | Returns | Description |
| :------ | :------ | :------ | :------ |
| `if`* | boolean, any, any | any | Returns one expression if the given condition is true, and the other if false. The other expression within will not be evaluated. Requires three rands (one boolean, two of any type). |
| `cond`* | (boolean, any)+ | any | Returns expression associated with the true condition, or `void` if none are true. See `cond.lisp` or `fizzbuzz.lisp` in the `samples` folder for examples. |
| `define`* | name, any | `#void` | Creates a binding with the name given in the current scope. |
| `do`* | any+ | `#void` | Executes each invocation given. |
| `fail` | string? | `#void` | Generates an error, with an optional error message. In REPL mode, Psil will just output the error message to standard error, and continue to accept input, while in load mode, Psil will terminate execution of the given script after the error message is printed. |
| `exit` | number? | N/A | Exits the program with an status code (without a rand, the status code is `0`). |
| `type` | any | symbol | Returns the type of the given rand. |

#### Input/output
| Name | Rands | Returns | Description |
| :------ | :------ | :------ | :------ |
| `put` | any* | `#void` | Prints each rand concatenated, followed by a new line. |
| `put-each` | any+ | `#void` | Puts each rand, each on a new line. |
| `print` | any+ | `#void` | Prints each rand without any new lines. |
| `input` | _none_ | string | Takes input from console. |

#### String
| Name | Rands | Returns | Description |
| :------ | :------ | :------ | :------ |
| `2str` | any | string | Converts any value to a string. |
| `is-str?` | any | boolean | Checks if the rand is a string. |
| `str-cat` | any{2,} | string | Concatenates each rand. If a rand is not a string, it will be converted into one. |
| `str-contains?` | string{2} | boolean | Checks if the second rand is contained in the first rand. |
| `str-empty?` | string | boolean | Checks if the string is the empty string (`""`). |
| `str-insert` | string, number, string | string | Inserts the second rand into the first at the given index. |
| `str-len` | string | number | Returns the length of the string. |
| `str-low` | string | string | Returns the lowercase version of the given string. |
| `str-repeat` | string, number | string | Returns the string repeated as many times the second rand is. |
| `str-replace` | string{3} | string | Returns the first string with all instances of the second rand (the "old text") replaced by the third rand (the "new text"). |
| `str-starts-with?` | string{2} | boolean | Checks if the first rand starts with the second rand. |
| `str-strip` | string | string | Removes leading/trailing whitespace from the given string. |
| `str-trunc` | string, number | string | Shortens the given string to the length of the second rand. |
| `str-up` | string | string | Returns the uppercase version of the given string. |

#### Math
| Name | Rands | Returns | Description |
| :------ | :------ | :------ | :------ |
| `+` | number{2,} | number | Adds all rands given. |
| `-` | number{2,} | number | Subtracts the first rand from the remaining rands. |
| `*` | number{2,} | number | Multiplies all rands. |
| `/` | number{2,} | number | Divides the first by each remaining rands. |
| `%` | number{2,} | number | Gives the remainder of the first by each remaining rands. `(% 5 5 1)` is equivalent to `(% (% 5 5) 1)`. |
| `is-num?` | any | boolean | Checks if the rand is a number. |

#### Boolean
| Name | Rands | Returns | Description |
| :------ | :------ | :------ | :------ |
| `and`* | boolean{2,} | boolean | ANDs each rand together. Always short-circuited. |
| `or`* | boolean{2,} | boolean | ORs each rand together. Always short-circuited. |
| `not` | boolean | boolean | Gives the negation of the given rand. |
| `xor` | boolean{2,} | boolean | XORs all rands. |
| `==` | any{2,} | boolean | Checks for equality between all rands. |
| `!=` | any{2,} | boolean | Checks for inequality between all rands. `(!= 5 6 6)` would return false, since the two `6` are both equal, even if neither are equal to the first rand. |
| `>` | number{2,} | boolean | Checks that first rand is greater than all other rands. |
| `>=` | number{2,} | boolean | Checks that first rand is greater than or equal to all other rands. |
| `<` | number{2,} | boolean | Checks that first rand is less than all other rands. |
| `<=` | number{2,} | boolean | Checks that first rand is less than or equal to all other rands. |
| `is-bool?` | any | boolean | Checks if the rand is a boolean. |

#### Symbol
| Name | Rands | Returns | Description |
| :------ | :------ | :------ | :------ |
| `str2symb` | string | symbol | Converts given string to a symbol. |
| `symb2str` | symbol | string | Converts given symbol to a string. |
| `is-symb?` | any | boolean | Checks if the rand is a symbol. |
| `is-void?` | any | boolean | Checks if the rand is `#void`. |

#### Procedure
| Name | Rands | Returns | Description |
| :------ | :------ | :------ | :------ |
| `is-proc?` | any | boolean | Checks if the rand is a procedure. |

#### List
| Name | Rands | Returns | Description |
| :------ | :------ | :------ | :------ |
| `list` | any* | list | Creates a list from the given rands. |
| `list-len` | list | number | Returns the length of the list. | 
| `list-range` | number{2,3} | list | Creates a list of numbers from a given range, with an optional step-size. |

## Technologies used
* [Rust](https://github.com/rust-lang/rust)
    * [Cargo](https://github.com/rust-lang/cargo)

Early versions of Psil used [pest](https://github.com/pest-parser/pest).

## Authors
1. Eric Schneider (main author)
2. Chongyi Zheng (contributor to Limp)
