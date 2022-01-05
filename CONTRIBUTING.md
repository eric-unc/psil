# Contributing to Psil
Thanks for your interest in contributing to Psil!

## Liability
By contributing to Psil, you agree that all contributions made are your own and to be licensed under [the given license](LICENSE).

## Rust conventions
This project currently does not use [rustfmt](https://github.com/rust-lang/rustfmt) because it doesn't fully support my coding style. Still, Psil somewhat loosely follows [the project's standard style guide](https://github.com/rust-dev-tools/fmt-rfcs/blob/master/guide/guide.md), with some notable diversions:
* Tabs should be used for identation for all source files.
* "Trailing commas" should not be included, as such:
```rust
// Do this:
enum Direction {
	Up,
	Down
}
// Not this:
enum Direction {
	Up,
	Down,
}
```
* There is no maximum line length/width. I recommend using line wrapping (which should be standard with any text editor). I prefer long lines over artificially split ones, although if a line seems too unwieldly, it might be a good idea to split it using variables or shorter names when appropriate.

## Other notes
Psil is currently something of a pet project by yours truly (@eric-unc). I have something of a loose vision for the project, and I do want to be familiar with every part of the codebase. Regardless, I welcome any and all contributions, but anything big should probably be run by me first.
