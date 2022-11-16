use mathp::{types::*, *};

#[test]
fn add() {
	let parsed = parser::parse("(+ 3 21)".to_string());
	let eval = evaluator::evaluate(parsed);
	assert_eq!(eval, Expression::Num(24))
}

#[test]
fn recursive() {
	let parsed = parser::parse("(- (* 9 (- 1 1)) (* (+ 7 6) (/ 6 1)) )".to_string());
	let eval = evaluator::evaluate(parsed);
	assert_eq!(eval, Expression::Num(-78))
}

#[test]
fn negatives() {
	let parsed = parser::parse("(+ (+ -12 -5) 6)".to_string());
	let eval = evaluator::evaluate(parsed);
	assert_eq!(eval, Expression::Num(-11))
}

#[test]
fn file() {
	let parsed = parser::parse_file("test_data/numeric.mathp");
	let eval = evaluator::evaluate(parsed);
	assert_eq!(eval, Expression::Num(17))
}
