use crate::Rule;
use pest::iterators::{Pair, Pairs};

use crate::ast::*;

pub fn parse(tree: Pairs<Rule>) -> ProgramAst {
	parse_program(tree)
}

// program ::= expr_list?
pub fn parse_program(tree: Pairs<Rule>) -> ProgramAst {
	let mut list = vec![];

	for pair in tree {
		for inner_pair in pair.into_inner() {
			match inner_pair.as_rule() {
				Rule::expr_list => list = parse_expr_list(inner_pair),
				Rule::EOI => {}
				_ => unreachable!()
			}
		}
	}

	ProgramAst { expr_list: list }
}

// expr_list ::= expr+
fn parse_expr_list(exprs_tree: Pair<Rule>) -> ExprListAst {
	exprs_tree.into_inner()
		.into_iter()
		.map(|expr| parse_expr(expr))
		.collect()
}

// expr ::= atom | special_form | invocation
pub fn parse_expr(expr_tree: Pair<Rule>) -> ExprAst {
	for inner_pair in expr_tree.into_inner() {
		return match inner_pair.as_rule() {
			Rule::atom => ExprAst::Atom(Box::from(parse_atom(inner_pair))),
			Rule::special_form => ExprAst::SpecialForm(Box::from(parse_special_form(inner_pair))),
			Rule::invocation => ExprAst::Invocation(parse_invocation(inner_pair)),
			_ => unreachable!()
		}
	}

	unreachable!()
}

// atom ::= number | boolean | string | void | lambda | name
fn parse_atom(atom_tree: Pair<Rule>) -> AtomAst {
	for inner_pair in atom_tree.into_inner() {
		return match inner_pair.as_rule() {
			Rule::number => AtomAst::Number(parse_number(inner_pair)),
			Rule::boolean => AtomAst::Boolean(parse_boolean(inner_pair)),
			Rule::string => AtomAst::String(parse_string(inner_pair)),
			Rule::symbol => AtomAst::String(parse_symbol(inner_pair)),
			Rule::void => AtomAst::Void,
			Rule::lambda => AtomAst::Lambda(parse_lambda(inner_pair)),
			Rule::name => AtomAst::Name(inner_pair.as_str().to_string()),
			_ => unreachable!()
		}
	}

	unreachable!()
}

// special_form ::= if | cond | define | do | and | or
fn parse_special_form(special_form_tree: Pair<Rule>) -> SpecialFormAst {
	for inner_pair in special_form_tree.into_inner() {
		return match inner_pair.as_rule() {
			Rule::if_form => SpecialFormAst::If(parse_if(inner_pair)),
			Rule::cond => SpecialFormAst::Cond(parse_cond(inner_pair)),
			Rule::define => SpecialFormAst::Define(parse_define(inner_pair)),
			Rule::do_form => SpecialFormAst::Do(parse_do(inner_pair)),
			Rule::and => SpecialFormAst::And(parse_and(inner_pair)),
			Rule::or => SpecialFormAst::Or(parse_or(inner_pair)),
			_ => unreachable!()
		}
	}

	unreachable!()
}

// invocation ::= ( name expr_list? )
fn parse_invocation(invocation_tree: Pair<Rule>) -> InvocationAst {
	let mut iter = invocation_tree.into_inner();

	let proc = iter.next().unwrap().as_str().to_string();

	let expr_list = match iter.next() {
		Some(s) => parse_expr_list(s),
		None => ExprListAst::new()
	};

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

// cond ::= ( cond (expr expr)+ )
fn parse_cond(cond_tree: Pair<Rule>) -> CondAst {
	let mut is_on_cond = true;

	let mut conds = ExprListAst::new();
	let mut expr_list = ExprListAst::new();

	for inner_pair in cond_tree.into_inner() {
		let expr = parse_expr(inner_pair);

		if is_on_cond {
			conds.push(expr);
			is_on_cond = false;
		} else {
			expr_list.push(expr);
			is_on_cond = true;
		}
	}

	if conds.len() != expr_list.len() { // This should be resolved by pest, but just in case.
		panic!("Conditions and expressions are unmatching!")
	}

	CondAst { conds, expr_list }
}

// define ::= ( define name expr )
fn parse_define(define_tree: Pair<Rule>) -> DefineAst {
	let mut iter = define_tree.into_inner();

	let name = iter.next().unwrap().as_str().to_string();
	let value = parse_expr(iter.next().unwrap());

	DefineAst { name, value }
}

// do ::= ( do expr_list? )
fn parse_do(do_tree: Pair<Rule>) -> DoAst {
	let expr_list = parse_expr_list(do_tree.into_inner().next().unwrap());

	DoAst { expr_list }
}

// and ::= ( and expr_list )
fn parse_and(and_tree: Pair<Rule>) -> AndAst {
	let expr_list = parse_expr_list(and_tree.into_inner().next().unwrap());

	AndAst { expr_list }
}

// or ::= ( or expr_list )
fn parse_or(or_tree: Pair<Rule>) -> OrAst {
	let expr_list = parse_expr_list(or_tree.into_inner().next().unwrap());

	OrAst { expr_list }
}

fn parse_number(float_tree: Pair<Rule>) -> f64 {
	float_tree.as_span().as_str().parse::<f64>().unwrap()
}

fn parse_boolean(boolean_tree: Pair<Rule>) -> bool {
	match boolean_tree.as_span().as_str() {
		"true" => true,
		"false" => false,
		_ => unreachable!()
	}
}

fn parse_string(string_tree: Pair<Rule>) -> String {
	for inner in string_tree.into_inner() {
		return match inner.as_rule() {
			Rule::string_inner => inner.as_str().to_string(),
			_ => unreachable!()
		}
	}

	unreachable!()
}

fn parse_symbol(symbol_tree: Pair<Rule>) -> String {
	for inner in symbol_tree.into_inner() {
		return match inner.as_rule() {
			Rule::symbol_inner => inner.as_str().to_string(),
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
			LambdaAst { params, expr }
		}
		Rule::expr => {
			let params = ParamsAst { names: vec![] };
			let expr = parse_expr(first);
			LambdaAst { params, expr }
		}
		_ => unreachable!()
	}
}

fn parse_params(params_tree: Pair<Rule>) -> ParamsAst {
	let names = params_tree
		.into_inner()
		.map(|pair| pair.as_str().to_string())
		.collect();

	ParamsAst { names }
}
