use std::fmt::{Display, Formatter, Result as ResultFmt};

use crate::ast::LambdaAst;
use crate::val::Val::{Boolean, Number, Procedure, String as StringVal, Void};

#[derive(Clone, Debug)]
pub enum Val {
	Number(f64),
	Boolean(bool),
	String(String),
	Void,
	Procedure(ProcedureType),
}

pub type ValList = Vec<Val>;

impl Display for Val {
	fn fmt(&self, f: &mut Formatter) -> ResultFmt {
		write!(f, "{}", match self {
			Number(n) => n.to_string(),
			Boolean(b) => b.to_string(),
			StringVal(s) => s.to_string(),
			Void => "void".to_string(),
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
