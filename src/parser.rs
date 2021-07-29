use crate::Rule;
use pest::iterators::{Pair, Pairs};

use crate::ast::*;

pub fn parse(tree: Pairs<Rule>) -> ProgramAst {
	parse_program(tree)
}

pub fn parse_program(tree: Pairs<Rule>) -> ProgramAst {
	let mut list = vec![];

	for pair in tree {
		for inner_pair in pair.into_inner() {
			match inner_pair.as_rule() {
				Rule::expr_list => {
					list = parse_expr_list(inner_pair);
				},
				Rule::EOI => {},
				_ => {
					unreachable!();
				}
			}
		}
	}

	ProgramAst { expr_list: list }
}

pub fn parse_expr_list(tree: Pair<Rule>) -> ExprListAst {
	println!("{}", tree);

	vec![]
}
