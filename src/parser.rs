use crate::Rule;
use pest::iterators::{Pair, Pairs};

use crate::ast::*;
use pest::Parser;

pub fn parse(tree: Pairs<Rule>) -> ProgramAst {
	parse_program(tree)
}

// program ::= expr_list?
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

// expr_list ::= expr+
fn parse_expr_list(exprs_tree: Pair<Rule>) -> ExprListAst {
	let mut ret = Vec::new();

	for expr in exprs_tree.into_inner() {
		ret.push(parse_expr(expr));
	}

	ret
}

// expr ::= atom | special_form | invocation
fn parse_expr(expr_tree: Pair<Rule>) -> ExprAst {
	for inner_pair in expr_tree.into_inner() {
		match inner_pair.as_rule() {
			Rule::atom => return ExprAst::Atom(Box::from(parse_atom(inner_pair))),
			Rule::special_form => return ExprAst::SpecialForm(Box::from(parse_special_form(inner_pair))),
			Rule::invocation => return ExprAst::Invocation(parse_invocation(inner_pair)),
			_ => unreachable!()
		}
	}

	unreachable!()
}

// atom ::= number | boolean | string | void | lambda | name
fn parse_atom(atom_tree: Pair<Rule>) -> AtomAst {
	for inner_pair in atom_tree.into_inner() {
		match inner_pair.as_rule() {
			Rule::number => return AtomAst::Number(parse_number(inner_pair)),
			Rule::boolean => return AtomAst::Boolean(parse_boolean(inner_pair)),
			Rule::string => return AtomAst::String(parse_string(inner_pair)),
			Rule::void => return AtomAst::Void,
			Rule::lambda => return AtomAst::Lambda(parse_lambda(inner_pair)),
			Rule::name => return AtomAst::Name(inner_pair.to_string()),
			_ => unreachable!()
		}
	}

	unreachable!();
}

// special_form ::= if | define | do
fn parse_special_form(special_form_tree: Pair<Rule>) -> SpecialFormAst {
	for inner_pair in special_form_tree.into_inner() {
		match inner_pair.as_rule() {
			Rule::if_form => return SpecialFormAst::If(parse_if(inner_pair)),
			Rule::define => return SpecialFormAst::Define(parse_define(inner_pair)),
			Rule::do_form => return SpecialFormAst::Do(parse_do(inner_pair)),
			_ => unreachable!()
		}
	}

	unreachable!();
}

// invocation ::= ( name expr_list? )
fn parse_invocation(invocation_tree: Pair<Rule>) -> InvocationAst {
	let mut iter = invocation_tree.into_inner();

	let proc = iter.next().unwrap().to_string();
	let expr_list = parse_expr_list(iter.next().unwrap());

	InvocationAst { proc, expr_list }
}

// if ::= ( if expr expr expr )
fn parse_if(if_tree: Pair<Rule>) -> IfAst {
	let mut iter = if_tree.into_inner();

	let cond = parse_expr(iter.next().unwrap());
	let if_true = parse_expr(iter.next().unwrap());
	let if_false = parse_expr(iter.next().unwrap());

	IfAst { cond, if_true, if_false }
}

// define ::= ( define name expr )
fn parse_define(define_tree: Pair<Rule>) -> DefineAst {
	let mut iter = define_tree.into_inner();

	let name = iter.next().unwrap().to_string();
	let value = parse_expr(iter.next().unwrap());

	DefineAst { name, value }
}

// do ::= ( do expr_list? )
fn parse_do(do_tree: Pair<Rule>) -> DoAst {
	let expr_list = parse_expr_list(do_tree.into_inner().next().unwrap());

	DoAst { expr_list }
}

fn parse_number(float_tree: Pair<Rule>) -> f64 {
	float_tree.as_span().as_str().parse::<f64>().unwrap()
}

fn parse_boolean(boolean_tree: Pair<Rule>) -> bool {
	match boolean_tree.as_span().as_str() {
		"true" => true,
		"false" => false,
		_ => panic!()
	}
}

fn parse_string(string_tree: Pair<Rule>) -> String {
	for inner in string_tree.into_inner() {
		match inner.as_rule() {
			Rule::string_inner => return inner.to_string(),
			_ => unreachable!()
		}
	}

	unreachable!()
}

fn parse_lambda(lambda_tree: Pair<Rule>) -> LambdaAst {
	let mut iter = lambda_tree.into_inner();

	let first = iter.next().unwrap();

	match first.as_rule() {
		Rule::params => {
			let params = parse_params(first);
			let expr = parse_expr(iter.next().unwrap());
			return LambdaAst { params, expr }
		},
		Rule::expr => {
			let params = ParamsAst { names: vec![] };
			let expr = parse_expr(first);
			return LambdaAst { params, expr }
		},
		_ => unreachable!()
	};
}

fn parse_params(params_tree: Pair<Rule>) -> ParamsAst {
	let mut names = Vec::new();

	for inner_pair in params_tree.into_inner() {
		match inner_pair.as_rule() {
			Rule::name => names.push(inner_pair.to_string()),
			_ => unreachable!()
		}
	}

	ParamsAst { names }
}
