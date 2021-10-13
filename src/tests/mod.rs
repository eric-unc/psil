mod boolean;
mod control;
mod io;
mod math;
mod number;
mod proc;
mod str;
mod symb;

use pest::error::Error;
use pest::iterators::{Pair, Pairs};

use crate::PsilPestParser;
use crate::Rule;
use crate::pest::Parser;
use crate::environment::Environment;
use crate::eval::{eval as eval_psil, eval_expr};
use crate::parser::{parse as parse_psil, parse_expr};
use crate::val::Val;

#[macro_export]
macro_rules! evals_and_eq {
	( $testing:literal, $expected:expr ) => {
		let testing = $testing;
		assert!(parse(testing).is_ok(), "{} does not parse!", testing);
		let eval_result = eval(testing);
		assert!(eval_result.is_ok(), "{} does not eval!", testing);
		assert_eq!(eval_result.unwrap(), $expected, "{} doesn't equal {}!", testing, $expected);
	}
}

#[macro_export]
macro_rules! evals_and_eq_with_env {
	( $testing:literal, $expected:expr, $env:expr ) => {
		let testing = $testing;
		assert!(parse(testing).is_ok(), "{} does not parse!", testing);
		let eval_result = eval_with_env(testing, &mut $env);
		assert!(eval_result.is_ok(), "{} does not eval!", testing);
		assert_eq!(eval_result.unwrap(), $expected, "{} doesn't equal {}!", testing, $expected);
	}
}

#[macro_export]
macro_rules! fails_eval {
	( $testing:literal ) => {
		let testing = $testing;
		assert!(parse(testing).is_ok(), "{} does not parse!", testing);
		assert!(eval(testing).is_err(), "{} evals when it should not!", testing);
	}
}

pub fn parse(src: &str) -> Result<Pairs<Rule>, Error<Rule>> {
	PsilPestParser::parse(Rule::expr, src)
}

pub fn eval(src: &str)  -> Result<Val, String> {
	eval_with_env(src, &mut Environment::new())
}

pub fn eval_with_env(src: &str, env: &mut Environment) -> Result<Val, String> {
	let parse_tree = parse(src).unwrap();

	for pair in parse_tree {
		return eval_expr(parse_expr(pair), env);
	}

	unreachable!();
}

pub fn eval_tree(expr_tree: Pair<Rule>) -> Result<Val, String> {
	eval_tree_with_env(expr_tree, &mut Environment::new())
}

pub fn eval_tree_with_env(expr_tree: Pair<Rule>, env: &mut Environment) -> Result<Val, String> {
	eval_expr(parse_expr(expr_tree), env)
}
