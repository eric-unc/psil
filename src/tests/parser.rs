use crate::ast::*;
use crate::parser_new::*;
use crate::scanner::Scanner;

#[test]
fn number() {
	let mut res = parse_expr_entry("5".to_string());
	assert!(res.is_ok());
	assert_eq!(res.unwrap(), ExprAst::Atom(Box::from(AtomAst::Number(5.0))));
}

#[test]
fn symbol() {
	let mut res = parse_expr_entry("#void".to_string());
	assert!(res.is_ok());
	assert_eq!(res.unwrap(), ExprAst::Atom(Box::from(AtomAst::Symbol("void".to_string()))));
}

#[test]
fn simple_program() {
	let mut res = parse("(+ 5 6)".to_string());
	assert!(res.is_ok());
	assert_eq!(res.unwrap(),
			   ProgramAst { expr_list: vec![
				   ExprAst::NewInvocation(NewInvocationAst {
					   proc: PossibleProc::Name("+".to_string()),
					   expr_list: vec![
						   ExprAst::Atom(Box::from(AtomAst::Number(5.0))),
						   ExprAst::Atom(Box::from(AtomAst::Number(6.0)))
					   ] })
			   ] }
	);
}
