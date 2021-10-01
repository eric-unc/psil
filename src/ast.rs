use std::boxed::Box;

// Main
#[derive(Clone, Debug)]
pub struct ProgramAst {
	pub expr_list: ExprListAst,
}

pub type ExprListAst = Vec<ExprAst>;

#[derive(Clone, Debug)]
pub enum ExprAst {
	Atom(Box<AtomAst>),
	SpecialForm(Box<SpecialFormAst>),
	Invocation(InvocationAst),
}

#[derive(Clone, Debug)]
pub enum AtomAst {
	Number(f64),
	Boolean(bool),
	String(String),
	Symbol(String),
	Lambda(LambdaAst),
	Name(String),
}

#[derive(Clone, Debug)]
pub struct LambdaAst {
	pub params: ParamsAst,
	pub expr: ExprAst,
}

#[derive(Clone, Debug)]
pub enum SpecialFormAst {
	If(IfAst),
	Cond(CondAst),
	Define(DefineAst),
	Do(DoAst),
	And(AndAst),
	Or(OrAst)
}

#[derive(Clone, Debug)]
pub struct InvocationAst {
	pub proc: NameAst,
	pub expr_list: ExprListAst,
}

// Special forms
#[derive(Clone, Debug)]
pub struct IfAst {
	pub cond: ExprAst,
	pub if_true: ExprAst,
	pub if_false: ExprAst,
}

#[derive(Clone, Debug)]
pub struct CondAst {
	pub conds: ExprListAst,
	pub expr_list: ExprListAst,
}

#[derive(Clone, Debug)]
pub struct DefineAst {
	pub name: NameAst,
	pub value: ExprAst,
}

#[derive(Clone, Debug)]
pub struct DoAst {
	pub expr_list: ExprListAst,
}

#[derive(Clone, Debug)]
pub struct AndAst {
	pub expr_list: ExprListAst,
}

#[derive(Clone, Debug)]
pub struct OrAst {
	pub expr_list: ExprListAst,
}

// Support
#[derive(Clone, Debug)]
pub struct ParamsAst {
	pub names: Vec<NameAst>,
}

pub type NameAst = String;
