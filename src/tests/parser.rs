use crate::ast::*;
use crate::parser::*;

#[test]
fn number() {
	let res = parse_expr_entry("5".to_string());
	assert!(res.is_ok());
	assert_eq!(res.unwrap(), ExprAst::Atom(Box::from(AtomAst::Number(5.0))));
}

#[test]
fn symbol() {
	let res = parse_expr_entry("#void".to_string());
	assert!(res.is_ok());
	assert_eq!(res.unwrap(), ExprAst::Atom(Box::from(AtomAst::Symbol("void".to_string()))));
}

#[test]
fn simple_program() {
	let res = parse("(+ 5 6)".to_string());
	assert!(res.is_ok());
	assert_eq!(res.unwrap(),
			   ProgramAst { expr_list: vec![
				   ExprAst::Invocation(InvocationAst {
					   proc: PossibleProc::Name("+".to_string()),
					   expr_list: vec![
						   ExprAst::Atom(Box::from(AtomAst::Number(5.0))),
						   ExprAst::Atom(Box::from(AtomAst::Number(6.0)))
					   ] })
			   ] }
	);
}
