use std::str::Chars;

#[derive(Debug)]
enum Expression {
	Func(Function),
	Num(i64),
}

#[derive(Debug)]
struct Function {
	name: String,
	arguments: Vec<Expression>,
}

pub fn parse(string: String) {
	let expr = evaluate_expression(&mut string.chars(), None);
	println!("{:?}", expr)
}

fn evaluate_expression(iter: &mut Chars, next: Option<char>) -> Expression {
	let character = match next {
		Some(i) => i,
		None => iter.next().unwrap_or_else(throw_eof),
	};
	match character {
		' ' => evaluate_expression(iter, None),
		'(' => {
			let name = get_next_word(iter, None);
			let arguments = parse_function(iter, None, Vec::<Expression>::new());
			let func = Function { name, arguments };
			Expression::Func(func)
		}
		i => get_num(iter, Some(i)),
	}
}

fn parse_function(
	iter: &mut Chars,
	next: Option<char>,
	mut arguments: Vec<Expression>,
) -> Vec<Expression> {
	let character = match next {
		Some(i) => i,
		None => iter.next().unwrap_or_else(throw_eof),
	};
	if character == ')' {
		return arguments;
	}
	let expr = evaluate_expression(iter, Some(character));
	arguments.push(expr);
	match iter.next() {
		Some(next) => parse_function(iter, Some(next), arguments),
		None => arguments,
	}
}

fn get_num(iter: &mut Chars, next: Option<char>) -> Expression {
	let num_string = get_next_word(iter, next);
	Expression::Num(num_string.parse().unwrap())
}

fn get_next_word(iter: &mut Chars, next: Option<char>) -> String {
	let mut character = match next {
		Some(i) => i,
		None => iter.next().unwrap_or_else(throw_eof),
	};
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

pub fn throw_void(error: &str) {
	throw(error)
}

pub fn throw<T>(error: &str) -> T {
	let program_name = env!("CARGO_PKG_NAME");
	println!("{program_name}: {error}");
	#[cfg(not(debug_assertions))]
	std::process::exit(1);
	#[cfg(debug_assertions)]
	panic!();
}
