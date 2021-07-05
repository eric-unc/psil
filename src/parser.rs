use pest::iterators::{Pairs, Pair};
use crate::Rule;

use crate::ast::*;

pub fn parse(tree: Pairs<Rule>) -> Program {
	parse_program(tree)
}

pub fn parse_program(tree: Pairs<Rule>) -> Program {
	// TODO
}