use std::collections::HashMap;

use crate::{throw, types::*};

pub fn evaluate(expr: Expression) -> i64 {
	match expr {
		Expression::Func(func) => evaluate_function(func),
		Expression::Num(num) => num,
	}
}

fn evaluate_function(func: Function) -> i64 {
	fn convert_args(args: Vec<Expression>) -> Vec<i64> {
		args.into_iter().map(convert_arg).collect()
	}
	fn convert_arg(expr: Expression) -> i64 {
		match expr {
			Expression::Func(function) => evaluate_function(function),
			Expression::Num(num) => num,
		}
	}
	let mut functions = HashMap::<&str, &dyn Fn(Vec<i64>) -> i64>::new();
	functions.insert("+", &add);
	functions.insert("-", &sub);
	let new_func = match functions.get(&func.name as &str) {
		Some(i) => i,
		None => throw("No function {func.name}"),
	};
	let new_args = convert_args(func.arguments);
	new_func(new_args)
}

fn add(x: Vec<i64>) -> i64 {
	x.iter().fold(0, |total, new| total + new)
}
fn sub(x: Vec<i64>) -> i64 {
	x.iter().fold(0, |total, new| total - new)
}
