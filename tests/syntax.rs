use mathp::*;

#[test]
#[should_panic]
fn unclosed_bracket() {
	parser::parse("(+ (+ 3 21) 4".to_string());
}

/// There shouldn't really be any reason to allow characters after an expression
/// has finished, and will only lead to confusion such as the example below
///
/// At first glance it seems like it should evaluate to 28 (+ (+ 3 21) 4)
/// But in reality is 24 (+ (+ 3 21)) with a trailing ' 4)'
#[test]
#[should_panic]
fn unopened_bracket() {
	parser::parse("(+ (+ 3 21)) 4)".to_string());
}

#[test]
#[should_panic]
fn thats_not_an_s_expression() {
	parser::parse("(1 + 1)".to_string());
}
