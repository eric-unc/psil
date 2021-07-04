pub struct Program {
	expr_list: ExpressionList
}

type ExpressionList = Vec<Expression>;

// TODO: this would be better as like an abstract class or something.
union Expression {
	atom: Atom,
	special_form: SpecialForm,
	invocation: Invocation
}

enum Atom {
	Number(f64),
	Boolean(bool),
	String(String),
	Void,
	Name(String)
}

union SpecialForm {
	if_form: If,
	define: Define
}

pub struct If {
	cond: Expression,
	if_true: Expression,
	if_false: Expression
}

pub struct Define {
	name: Atom,
	value: Expression
}

pub struct Invocation {
	expr_list: ExpressionList
}

impl Invocation {
	pub fn get_proc(&self) -> Expression {
		self.exprs[0]
	}

	// the 1st arg has arg_num `1`
	pub fn get_arg(&self, arg_num: i32) -> Expression {
		self.exprs[arg_num]
	}
}
