# Grammar
This document describes Psil's grammar in a somewhat formal way, which roughly follows [EBNF](https://en.wikipedia.org/wiki/Extended_Backus-Naur_form).

The actual parser may do something slightly different (but equivalent) than this, based on convenience.

## The Grammar
* program ::= expr_list? _end_
* expr_list ::= expr+
* expr ::= atom | invocation
* atom ::= _number_ | _boolean_ | _string_ | _symbol_ | lambda | _identifier_
* invocation ::= **(** (_identifier_ | special_form) expr_list? **)**
* lambda ::= **{** params? expr **}**
* params ::= **|** identifier+ **|**
* special_form ::= **if** | **cond** | **define** | **do** | **and** | **or**

### Tokens
Above, all words/characters **in bold** are token literals, while all words _in italics_ are "specially defined tokens":
* _end_ is the end of the input stream.
* _number_ is any "word" that can be parsed as a Rust `f64`.
* _boolean_ ::= **true** | **false**
* _string_ ::= **"** _c_* **"**, where _c_ is any character except `"`. There are also the escape sequences `\\`, `\n`, `\r`, `\t`, `\"`. Strings such as `"\"` and "\z" are considered invalid tokens.
* _symbol_ ::= **#**_c_+, where _c_ is any character excluding whitespace.
* _identifier_ is pretty much any word that doesn't conflict with another token.
