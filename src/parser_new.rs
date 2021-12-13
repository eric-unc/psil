use std::iter::Peekable;
use std::str::Chars;

use crate::ast::*;
use crate::scanner::*;
use crate::scanner::Token::*;

pub enum ParserError {
	ScannerError(ScannerError),
	UnexpectedToken(Token)
}
use ParserError::*;

pub fn parse(str: String) -> Result<ProgramAst, ParserError> {
	let mut scanner = Scanner::new(str.as_str());
	parse_program(&mut scanner)
}

// program ::= expr_list?
pub fn parse_program(scanner: &mut Scanner) -> Result<ProgramAst, ParserError> {
	let mut list = vec![];

	if scanner.peek()? != Token::End {
		list = parse_expr_list(scanner)?;
	}

	Ok(ProgramAst { expr_list: list })
}

// expr_list ::= expr+
fn parse_expr_list(scanner: &mut Scanner) -> Result<ExprListAst, ParserError> {
	let mut list = vec![];

	list.push(parse_expr(scanner)?);

	while is_expr_start(scanner.peek()?) {
		list.push(parse_expr(scanner)?);
	}

	Ok(list)
}

fn is_expr_start(token: Token) -> bool {
	token == Identifier || token == LeftParen || token == LeftBracket || token == Number || token == Boolean || token == StringT || token == Symbol || token == If || token == Cond || token == Define || token == Do || token == And || token == Or
}

// expr ::= atom | invocation
pub fn parse_expr(scanner: &mut Scanner) -> Result<ExprAst, ParserError> {
	if scanner.peek()? != LeftParen {
		Ok(ExprAst::Atom(Box::from(parse_atom(scanner)?)))
	} else {
		Ok(ExprAst::Invocation(parse_invocation(scanner)?))
	}
}

// atom ::= number | boolean | string | lambda | name
fn parse_atom(scanner: &mut Scanner) -> Result<AtomAst, ParserError> {
	match scanner.peek()? {
		Number(n) => Ok(AtomAst::Number(n)),
		Boolean(b) => Ok(AtomAst::Boolean(b)),
		StringT(s) => Ok(AtomAst::String(s)),
		LeftBracket => Ok(AtomAst::Lambda(parse_lambda(scanner)?)),
		o => Err(UnexpectedToken(o))
	}
}

// invocation ::= ( identifier expr_list? )
fn parse_invocation(scanner: &mut Scanner) -> Result<InvocationAst, ParserError> {
	expect(scanner, LeftParen)?;

	let ident = expect_identifier(scanner)?;

	if scanner.peek()? == RightParen {
		scanner.scan()?;
		Ok(InvocationAst { proc: ident, expr_list: vec![] })
	} else {
		let expr_list = parse_expr_list(scanner)?;
		expect(scanner, RightParen)?;
		Ok(InvocationAst { proc: ident, expr_list })
	}
}

// lambda ::= { params? expr }
fn parse_lambda(scanner: &mut Scanner) -> Result<LambdaAst, ParserError> {

}

fn expect(scanner: &mut Scanner, token: Token) -> Result<Token, ParserError> {
	match scanner.scan() {
		Ok(t) =>
			if t == token {
				Ok(t)
			} else {
				UnexpectedToken(t)
			}
		Err(e) => ScannerError(e)
	}
}

fn expect_identifier(scanner: &mut Scanner) -> Result<String, ParserError> {
	match scanner.scan() {
		Ok(Identifier(i)) => Ok(i),
		Ok(t) => UnexpectedToken(t),
		Err(e) => ScannerError(e)
	}
}
