use std::str::Chars;

pub fn parse(string: String) {
	evaluate_expression(string.chars());
}

fn evaluate_expression(mut iter: Chars) -> Option<Chars> {
	let character = iter
		.next()
		.unwrap_or_else(|| throw("Missing closing bracket"));
	match character {
		' ' => evaluate_expression(iter),
		'(' => todo!(),
		')' => todo!(),
		_ => todo!(),
	}
}

pub fn throw(error: &str) -> char {
	let program_name = env!("CARGO_PKG_NAME");
	println!("{program_name}: {error}");
	#[cfg(not(debug_assertions))]
	std::process::exit(1);
	#[cfg(debug_assertions)]
	panic!();
}
