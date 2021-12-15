mod boolean;
mod control;
mod io;
mod list;
mod math;
mod number;
mod parser;
mod proc;
mod scanner;
mod str;
mod symb;

use crate::ast::ExprAst;
use crate::environment::Environment;
use crate::eval::{eval_expr};
use crate::parser::{parse_expr_entry, ParserError};
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

pub fn parse(src: &str) -> Result<ExprAst, ParserError> {
	parse_expr_entry(src.to_string())
}

pub fn eval(src: &str) -> Result<Val, String> {
	eval_with_env(src, &mut Environment::new())
}

pub fn eval_with_env(src: &str, env: &mut Environment) -> Result<Val, String> {
	let parse_tree = parse_expr_entry(src.to_string()).unwrap();

	eval_expr(parse_tree, env)
}
