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

Psil has two major constructions. The first is the "atom", which is a singular unit of value (such as an integer, or a name). The second is the "invocation," which is the activation of a procedure (must be a name type), and a series of expressions. The activated procedure is the "rator" (operator), and the passed expressions are the "rands" (operands). The rands are always resolved before the effect of the rator applies, except in the case of "special forms." Invocations will always return a value, even if that value is the void value.

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
| Type | Description
| :------ | :------
| number | Numbers are float-point values, such as `3`, `-55`, `0.55`.
| boolean | Booleans are truth values, which can either be `true` or `false`.
| string | Strings are a series of characters, like `"Ahhh!"` or `"545"`.
| procedure | A procedure (or "proc") is a block that returns an atom with optional arguments. See `procs.lisp` in the `samples` directory for examples. Procedures can be invoked easily if defined.
| void | Void is a singleton, usually returned from functions that don't return anything interesting.

### Built-in procedures
Special forms are marked with a `*` next to their names. They are similar to procedures, but with special evaluation rules that don't allow them to be implemented as regular procedures.

#### Control
| Name | Rands | Returns | Description
| :------ | :------ | :------ | :------
| `if`* | boolean, any, any | any | Returns one expression if the given condition is true, and the other if false. The other expression within will not be evaluated. Requires three rands (one boolean, two of any type).
| `cond`* | (boolean, any)+ | any | Returns expression associated with the true condition, or `void` if none are true. See `cond.lisp` or `fizzbuzz.lisp` in the `samples` folder for examples.
| `define`* | name, any | void | Creates a binding with the name given in the current scope.
| `do`* | any+ | void | Executes each invocation given.
| `fail` | string? | void | Generates an error, with an optional error message. In REPL mode, Psil will just output the error message to standard error, and continue to accept input, while in load mode, Psil will terminate execution of the given script after the error message is printed.
| `exit` | number? | void | Exits the program with an status code (without a rand, the status code is `0`).

#### String, input/output
| Name | Rands | Returns | Description
| :------ | :------ | :------ | :------
| `put` | any* | void | Prints (each on a new line) each rand.
| `print` | any+ | void | Prints each rand without any new lines.
| `input` | _none_ | string | Takes input from console.

#### Math
| Name | Rands | Returns | Description
| :------ | :------ | :------ | :------
| `+` | number{2,} | number | Adds all rands given.
| `-` | number{2,} | number | Subtracts the first rand from the remaining rands.
| `*` | number{2,} | number | Multiplies all rands.
| `/` | number{2,} | number | Divides the first by each remaining rands.
| `%` | number{2,} | number | Gives the remainder of the first by each remaining rands. `(% 5 5 1)` is equivalent to `(% (% 5 5) 1)`.

#### Boolean
| Name | Rands | Returns | Description
| :------ | :------ | :------ | :------
| `and`* | boolean{2,} | boolean | ANDs each rand together. Always short-circuited.
| `or`* | boolean{2,} | boolean | ORs each rand together. Always short-circuited.
| `not` | boolean | boolean | Gives the negation of the given rand.
| `xor` | boolean{2,} | boolean | XORs all rands.
| `==` | any{2,} | boolean | Checks for equality between all rands.
| `!=` | any{2,} | boolean | Checks for inequality between all rands. `(!= 5 6 6)` would return false, since the two `6` are both equal, even if neither are equal to the first rand.
| `>` | number{2,} | boolean | Checks that first rand is greater than all other rands.
| `>=` | number{2,} | boolean | Checks that first rand is greater than or equal to all other rands.
| `<` | number{2,} | boolean | Checks that first rand is less than all other rands.
| `<=` | number{2,} | boolean | Checks that first rand is less than or equal to all other rands.

## Technologies used
* [Rust](https://github.com/rust-lang/rust)
    * [Cargo](https://github.com/rust-lang/cargo)
    * [pest](https://github.com/pest-parser/pest)

## Authors
1. Eric Schneider (main author)
2. Chongyi Zheng (contributor to Limp)
