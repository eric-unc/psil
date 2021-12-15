use std::fmt::{Display, Formatter, Result as ResultFmt};

use crate::ast::{LambdaAst, SpecialForms};

#[derive(Clone, Debug)]
pub enum Val {
	Number(f64),
	Boolean(bool),
	StringV(String),
	Symbol(String),
	Procedure(ProcedureType),
	List(ValList)
}
use Val::*;

pub type ValList = Vec<Val>;

impl Val {
	pub fn get_type_name(&self) -> &str {
		match self {
			Number(_) => "number",
			Boolean(_) => "boolean",
			StringV(_) => "string",
			Symbol(_) => "symbol",
			Procedure(_) => "procedure",
			List(_) => "list"
		}
	}
}

impl Display for Val {
	fn fmt(&self, f: &mut Formatter) -> ResultFmt {
		write!(f, "{}", match self {
			Number(n) => n.to_string(),
			Boolean(b) => b.to_string(),
			StringV(s) => s.to_string(),
			Symbol(s) => {
				let mut ret = String::from("#");
				ret.push_str(s);
				ret
			}
			Procedure(_) => "<procedure>".to_string(), // TODO: https://github.com/eric-unc/psil/issues/3
			List(l) => {
				let mut ret = String::from("(list");

				for v in l {
					ret.push_str(" ");
					ret.push_str(v.to_string().as_str());
				}

				ret.push_str(")");
				ret
			}
		})
	}
}

impl PartialEq for Val {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Number(n), Number(o)) => *n == *o,
			(Boolean(b), Boolean(o)) => *b == *o,
			(StringV(s), StringV(o)) => s.eq(o),
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
	Pure(LambdaAst),
	SpecialForm(SpecialForms)
}

pub type NativeProcedure = fn(ValList) -> Result<Val, String>;

pub fn void() -> Val {
	Symbol("void".to_string())
}
