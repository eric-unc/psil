use std::fmt::{Display, Formatter, Result as ResultFmt};

use crate::ast::LambdaAst;
use crate::val::Val::{Boolean, Number, Procedure, String as StringVal, Symbol};

#[derive(Clone, Debug)]
pub enum Val {
	Number(f64),
	Boolean(bool),
	String(String),
	Symbol(String),
	Procedure(ProcedureType),
}

pub type ValList = Vec<Val>;

impl Val {
	pub fn get_type_name(&self) -> &str {
		match self {
			Number(_) => "number",
			Boolean(_) => "boolean",
			StringVal(_) => "string",
			Symbol(_) => "symbol",
			Procedure(_) => "procedure"
		}
	}
}

impl Display for Val {
	fn fmt(&self, f: &mut Formatter) -> ResultFmt {
		write!(f, "{}", match self {
			Number(n) => n.to_string(),
			Boolean(b) => b.to_string(),
			StringVal(s) => s.to_string(),
			Symbol(s) => {
				let mut ret = String::from("#");
				ret.push_str(s);
				ret
			}
			Procedure(_) => "<procedure>".to_string() // #3: more interesting output
		})
	}
}

impl PartialEq for Val {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Number(n), Number(o)) => *n == *o,
			(Boolean(b), Boolean(o)) => *b == *o,
			(StringVal(s), StringVal(o)) => s.eq(o),
			(Symbol(s), Symbol(o)) => s.eq(o),
			_ => false
		}
	}

	fn ne(&self, other: &Self) -> bool {
		!self.eq(other)
	}
}

#[derive(Clone, Debug)]
pub enum ProcedureType {
	Native(NativeProcedure),
	Pure(LambdaAst)
}

pub type NativeProcedure = fn(ValList) -> Result<Val, String>;

pub fn void() -> Val {
	Symbol("void".to_string())
}
