use mathp::{types::*, *};

#[test]
#[should_panic]
fn unclosed_bracket() {
	parser::parse("(+ (+ 3 21) 4".to_string());
}

// #[test]
// #[should_panic]
// fn unopened_bracket() {
// 	parser::parse("(+ (+ 3 21)) 4)".to_string());
// }

#[test]
#[should_panic]
fn thats_not_an_s_expression() {
	parser::parse("(1 + 1)".to_string());
}
