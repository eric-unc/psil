use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter, Result as ResultFmt};

use crate::ast::{LambdaAst, SpecialForms};
use crate::Environment;

#[derive(Clone, Debug)]
pub enum Val {
	Number(f64),
	Boolean(bool),
	StringV(String),
	Symbol(String),
	ProcedureV(Procedure),
	List(ValList),
	Table(HashMap<Val, Val>)
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
			ProcedureV(_) => "procedure",
			List(_) => "list",
			Table(_) => "table"
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
			ProcedureV(_) => "<procedure>".to_string(), // TODO: https://github.com/eric-unc/psil/issues/3
			List(l) => {
				let mut ret = String::from("(list");

				for v in l {
					ret.push(' ');
					ret.push_str(v.to_string().as_str());
				}

				ret.push(')');
				ret
			},
			Table(t) => {
				let mut ret = String::from("(table");

				for (k, v) in t {
					ret.push(' ');
					ret.push_str(k.to_string().as_str());
					ret.push(' ');
					ret.push_str(v.to_string().as_str());
				}

				ret.push(')');
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
			(List(l), List(o)) => l.eq(o),
			//(Table(t), Table(o)) => t.eq(&o), // TODO: table equality
			_ => false
		}
	}
}

#[derive(Clone)]
pub enum Procedure {
	Native(NativeProcedure),
	Pure(LambdaAst),
	SpecialForm(SpecialForms)
}

pub type NativeProcedure = fn(ValList, &mut Environment) -> Result<Val, String>;

impl Debug for Procedure {
	fn fmt(&self, f: &mut Formatter<'_>) -> ResultFmt {
		write!(f, "{}", match self {
			Procedure::Native(_) => "<native procedure>",
			Procedure::Pure(_) => "<pure procedure>",
			Procedure::SpecialForm(_) => "<special form>"
		})
	}
}

pub fn void() -> Val {
	Symbol("void".to_string())
}
