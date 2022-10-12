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
	let mut functions = HashMap::<&str, MathsFunction>::new();
	functions.insert("+", add);
	functions.insert("-", sub);
	functions.insert("*", mul);
	functions.insert("/", div);
	let new_func = match functions.get(&func.name as &str) {
		Some(i) => i,
		None => throw("No function {func.name}"),
	};
	let new_args = convert_args(func.arguments);
	new_func(new_args)
}

type MathsFunction = fn(Vec<i64>) -> i64;

fn add(x: Vec<i64>) -> i64 {
	x.into_iter().fold(0, |total, new| total + new)
}
fn sub(x: Vec<i64>) -> i64 {
	let (first, rest) = x
		.split_first()
		.unwrap_or_else(|| throw("Not enough arguments"));
	rest.into_iter().fold(*first, |total, new| total - new)
}
fn mul(x: Vec<i64>) -> i64 {
	x.into_iter().fold(1, |total, new| total * new)
}
fn div(x: Vec<i64>) -> i64 {
	let (first, rest) = x
		.split_first()
		.unwrap_or_else(|| throw("Not enough arguments"));
	rest.into_iter().fold(*first, |total, new| total / new)
}
