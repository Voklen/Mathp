use std::{iter::Peekable, str::Chars};

use crate::throw;
use crate::types::*;

pub fn parse_file(filename: &str) -> Expression {
	let file = std::fs::read_to_string(filename).unwrap();
	parse(file)
}
pub fn parse(string: String) -> Expression {
	let mut chars = string.chars().peekable();
	let expression = evaluate_expression(&mut chars);
	check_for_trailing_characters(chars);
	expression
}

fn check_for_trailing_characters(mut chars: Peekable<Chars>) {
	while chars.peek() == Some(&' ') {
		chars.next();
	}
	if chars.peek().is_some() {
		throw("Characters after end of expression")
	}
}

fn evaluate_expression(iter: &mut Peekable<Chars>) -> Expression {
	let character = iter.peek().unwrap_or_else(throw_eof);
	match character {
		' ' => {
			iter.next();
			evaluate_expression(iter)
		}
		'(' => {
			iter.next();
			let name = get_next_word(iter);
			let arguments = parse_function(iter, Vec::<Expression>::new());
			let func = Function { name, arguments };
			Expression::Func(func)
		}
		_ => get_num(iter),
	}
}

fn parse_function(iter: &mut Peekable<Chars>, mut arguments: Vec<Expression>) -> Vec<Expression> {
	let character = iter.peek().unwrap_or_else(throw_eof);
	if character == &')' || character.is_whitespace() {
		iter.next();
		return arguments;
	}
	let expr = evaluate_expression(iter);
	arguments.push(expr);
	match iter.peek() {
		Some(_) => parse_function(iter, arguments),
		None => arguments,
	}
}

fn get_num(iter: &mut Peekable<Chars>) -> Expression {
	let num_string = get_next_word(iter);
	Expression::Num(num_string.parse().unwrap())
}

fn get_next_word(iter: &mut Peekable<Chars>) -> String {
	let mut character = iter.next().unwrap_or_else(throw_eof);
	let mut string = "".to_string();
	while !(character.is_whitespace() || character == ')') {
		string.push(character);
		character = iter.next().unwrap_or_else(throw_eof)
	}
	string
}

pub fn throw_eof<T>() -> T {
	throw("Unexpected end of file: Missing closing bracket")
}
