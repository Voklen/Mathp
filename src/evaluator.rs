use std::collections::HashMap;

use crate::{throw, types::*};

pub fn evaluate(expr: Expression) -> Expression {
	match expr {
		Expression::Func(func) => evaluate_function(func),
		Expression::Num(num) => Expression::Num(num),
		Expression::Matrix(matrix) => Expression::Matrix(matrix),
	}
}

fn evaluate_function(func: Function) -> Expression {
	fn convert_args(args: Vec<Expression>) -> Vec<Expression> {
		args.into_iter().map(convert_arg).collect()
	}
	fn convert_arg(expr: Expression) -> Expression {
		match expr {
			Expression::Func(function) => evaluate_function(function),
			Expression::Num(num) => Expression::Num(num),
			Expression::Matrix(matrix) => Expression::Matrix(matrix),
		}
	}
	let mut functions = HashMap::<&str, MathsFunction>::new();
	// functions.insert("+", add);
	// functions.insert("-", sub);
	// functions.insert("*", mul);
	// functions.insert("/", div);
	let new_func = match functions.get(&func.name as &str) {
		Some(i) => i,
		None => throw("No function {func.name}"),
	};
	let new_args = convert_args(func.arguments);
	new_func(new_args)
}

type MathsFunction = fn(Vec<Expression>) -> Expression;

// fn add(x: Vec<Expression>) -> Expression {
// 	x.into_iter()
// 		.fold(Expression::Num(0), |total, new| total + new)
// }
// fn sub(x: Vec<Expression>) -> Expression {
// 	let (first, rest) = x
// 		.split_first()
// 		.unwrap_or_else(|| throw("Not enough arguments"));
// 	rest.into_iter().fold(*first, |total, new| total - new)
// }
// fn mul(x: Vec<Expression>) -> Expression {
// 	x.into_iter()
// 		.fold(Expression::Num(1), |total, new| total * new)
// }
// fn div(x: Vec<Expression>) -> Expression {
// 	let (first, rest) = x
// 		.split_first()
// 		.unwrap_or_else(|| throw("Not enough arguments"));
// 	rest.into_iter().fold(*first, |total, new| total / new)
// }
