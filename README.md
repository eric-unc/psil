# Psil
**Psil** is a Lisp-like programming language implemented in Rust. Currently a work-in-progress.

[Limp](https://github.com/eric-unc/limp) was the prototype for Psil and uses code from it.

## Installation
Psil can be installed by doing the following:
1. Cloning this repo (`git clone https://github.com/eric-unc/psil.git`).
2. Entering the directory (`cd psil`) and installing through Cargo (`cargo install --path .`).

## Running
Psil's command line interface has two modes: REPL and file loading:
* REPL mode: With no argument, Psil will take continually read input from the user and evaluate it. Note that the REPL will (as of yet) not automatically print what is evaluated, so it is not really a true REPL.
* Load mode: With a single argument, it will load the file and run it. Conventionally, Psil files can end in `.psil` or `.lisp`. This repository uses `.lisp` so that GitHub recognizes it as code.

## Language
Psil is a Lisp-like programming languages, with some twists.

Psil has two major constructions. The first is the "atom", which is a singular unit of value (such as an integer, or a name). The second is the "invocation," which is the activation of a procedure (must be a name type) and a series of values. The procedure being activated is called the "rator" (operator), and the values being passed are called the "rands" (operands).

### Example
```lisp
(print (+ 10 5) 1)
```

#### Example output
```
15
1
```

### Built-in procedures
| Name | Description
| :------ | :------
| `+` | Adds all rands given. Requires at least two rands (number).
| `-` | Subtracts the first rand from the remaining rands. Requires at least two rands (number).
| `print` | Prints (on new lines) each rand. Requires at least one rand (number/boolean).
| `exit` | Exits the program with a 0 status. With an optional rand, exits with that status (number).

### Special forms
Special forms are something like procedures.
| Name | Description
| :------ | :------
| `if` | Returns one expression if the given condition is true, the other if false. The other expression within will not be evaluated. Requires three rands (one boolean, two of any type).
| `define` | TODO. Requires at two rands (number).
| `do` | TODO (int/float/boolean).

## Technologies used
* [Rust](https://github.com/rust-lang/rust)
    * [Cargo](https://github.com/rust-lang/cargo)
    * [pest](https://github.com/pest-parser/pest)

## Author
1. Eric Schneider (main author)
2. Chongyi Zheng (contributor to Limp)
