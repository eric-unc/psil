use std::boxed::Box;

// Main
#[derive(Clone, Debug, PartialEq)]
pub struct ProgramAst {
	pub expr_list: ExprListAst,
}

pub type ExprListAst = Vec<ExprAst>;

#[derive(Clone, Debug, PartialEq)]
pub enum ExprAst {
	Atom(Box<AtomAst>),
	Invocation(InvocationAst)
}

#[derive(Clone, Debug, PartialEq)]
pub enum AtomAst {
	Number(f64),
	Boolean(bool),
	String(String),
	Symbol(String),
	Lambda(LambdaAst),
	Name(Name)
}

#[derive(Clone, Debug, PartialEq)]
pub struct LambdaAst {
	pub params: ParamsAst,
	pub expr: ExprAst
}

#[derive(Clone, Debug, PartialEq)]
pub struct InvocationAst {
	pub proc: PossibleProc,
	pub expr_list: ExprListAst
}

#[derive(Clone, Debug, PartialEq)]
pub enum PossibleProc {
	Name(Name),
	SpecialForm(SpecialForms)
}

#[derive(Clone, Debug, PartialEq)]
pub enum SpecialForms {
	If,
	Cond,
	Define,
	Do,
	And,
	Or
}

// Support
#[derive(Clone, Debug, PartialEq)]
pub struct ParamsAst {
	pub names: Vec<Name>
}

pub type Name = String;
