// Main
pub struct Program {
	expr_list: ExpressionList
}

pub type ExpressionList = Vec<Expression>;

pub enum Expression {
	Atom(Atom),
	SpecialForm(SpecialForm),
	Invocation(Invocation)
}

pub enum Atom {
	Number(f64),
	Boolean(bool),
	String(String),
	Void,
	Lambda {
		params: Params,
		expr: Expression
	},
	Name(String)
}

pub enum SpecialForm {
	If(If),
	Define(Define),
	Do(Do)
}

pub struct Invocation {
	proc: Name,
	expr_list: ExpressionList
}

// Special forms

pub struct If {
	cond: Expression,
	if_true: Expression,
	if_false: Expression
}

pub struct Define {
	name: Name,
	value: Expression
}

pub struct Do {
	expr_list: ExpressionList
}

// Support
pub struct Params {
	names: Vec<Name>
}

pub type Name = String;


