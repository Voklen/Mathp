use mathp::*;

#[test]
fn normal() {
	let parsed = parser::parse("(+ 1 2)".to_string());
	let eval = evaluator::evaluate(parsed);
	assert_eq!(eval, 3)
}
#[test]
fn file() {
	let parsed = parser::parse_file("test_data/add.mathp");
	let eval = evaluator::evaluate(parsed);
	assert_eq!(eval, 17)
}
