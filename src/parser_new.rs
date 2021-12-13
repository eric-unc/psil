use std::iter::Peekable;
use std::str::Chars;

use crate::ast::*;
use crate::scanner::*;

/*pub fn parse(str: String) -> ProgramAst {
	let mut scanner = str.chars().peekable();
	parse_program(&mut scanner)
}

// program ::= expr_list?
pub fn parse_program(scanner: &mut Peekable<Chars>) -> ProgramAst {
	let mut list = vec![];

	if scanner.peek().is_some() {
		list = parse_expr_list(scanner);
	}

	ProgramAst { expr_list: list }
}

// expr_list ::= expr+
fn parse_expr_list(scanner: &mut Peekable<Chars>) -> ExprListAst {
	let mut list = vec![];

	list
}

// expr ::= atom | special_form | invocation
pub fn parse_expr(scanner: &mut Peekable<Chars>) -> ExprAst {

}*/
