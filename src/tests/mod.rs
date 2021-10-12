mod number;

use pest::error::Error;
use pest::iterators::{Pair, Pairs};

use crate::PsilPestParser;
use crate::Rule;
use crate::pest::Parser;
use crate::environment::Environment;
use crate::eval::{eval as eval_psil, eval_expr};
use crate::parser::{parse as parse_psil, parse_expr};
use crate::val::Val;

pub fn parse(src: &str) -> Result<Pairs<Rule>, Error<Rule>> {
	PsilPestParser::parse(Rule::expr, src)
}

pub fn eval(src: &str)  -> Result<Val, String> {
	let mut env = Environment::new();
	let parse_tree = parse(src).unwrap();

	for pair in parse_tree {
		return eval_expr(parse_expr(pair), &mut env);
	}

	unreachable!();
}

pub fn eval_tree(expr_tree: Pair<Rule>) -> Result<Val, String> {
	let mut env = Environment::new();
	eval_expr(parse_expr(expr_tree), &mut env)
}
