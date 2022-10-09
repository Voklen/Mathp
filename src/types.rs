#[derive(Debug)]
pub enum Expression {
	Func(Function),
	Num(i64),
}

#[derive(Debug)]
pub struct Function {
	pub name: String,
	pub arguments: Vec<Expression>,
}