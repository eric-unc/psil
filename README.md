# Psil
**Psil** is a Lisp-like programming language implemented in Rust. It's currently a work-in-progress, although basic functionality works.

[Limp](https://github.com/eric-unc/limp) was the prototype for Psil and uses code from it.

***NOTE: Psil is currently in a state of rapid transformation, so some information presented is not up-to-date!***

## Features
* Variety of types: numbers, booleans, strings, symbols, procedures.
* Composite types: lists, tables (WIP).
* Bindings, control logic, mix of functional and procedural programming.
* Native documentation generator (official documentation [here](https://eric-unc.tech/psil/)) (WIP).
* Large standard library (always WIP...).

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
| number | Numbers are float-point values, such as `3`, `-55`, `0.55`. Numbers can be "integer" or "natural number" subtypes. |
| boolean | Booleans are truth values, which can either be `true` or `false`. |
| string | Strings are a series of characters, like `"Ahhh!"` or `"545"`. |
| symbol | Symbols are similar to strings, but more for internal use as human-readable identifiers. They can be used like enums, like `#north`, `#south`, `#east`, `#west`. One important symbol is `#void`, a singleton returned from functions that don't return anything interesting. |
| procedure | A procedure (or "proc") is a block that returns an atom with optional arguments. See `procs.lisp` in the `samples` directory for examples. Procedures can be invoked easily if defined. |
| list | A list is a composite value of values. It can be created through the `list` procedure. |

### Built-in procedures
The standard library is now documented on the [new fancy documentation page](https://eric-unc.tech/psil/)! It's a bit messy right now, but mostly up to date.

## Technologies used
* [Rust](https://github.com/rust-lang/rust)
    * [Cargo](https://github.com/rust-lang/cargo)

Early versions of Psil used [pest](https://github.com/pest-parser/pest).

## Authors
1. Eric Schneider (main author)
2. Chongyi Zheng (contributor to Limp)
