use std::{fmt::Display, ops::Add};

use crate::throw;

// Core
#[derive(Debug, Eq, PartialEq)]
pub enum Expression {
	Func(Function),
	Num(i64),
	Matrix(Matrix),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Function {
	pub name: String,
	pub arguments: Vec<Expression>,
}

impl Display for Expression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		//TODO Actually write a function for this
		write!(f, "{:?}", self)
	}
}

impl std::ops::Add for Expression {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		match self {
			Self::Num(num) => add_num(num, rhs),
			Self::Func(_) => todo!(),
			Self::Matrix(matrix) => add_matrix(matrix, rhs),
		}
	}
}

fn add_num(num: i64, expr: Expression) -> Expression {
	match expr {
		Expression::Num(other_num) => Expression::Num(num + other_num),
		Expression::Func(_) => todo!(),
		Expression::Matrix(_) => throw("Cannot add constant to matrix"),
	}
}
fn add_matrix(matrix: Matrix, expr: Expression) -> Expression {
	match expr {
		Expression::Num(_) => throw("Connot add constant to matrix"),
		Expression::Func(_) => todo!(),
		Expression::Matrix(other_matrix) => Expression::Matrix(matrix + other_matrix),
	}
}

impl std::ops::Sub for Expression {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		match self {
			Self::Num(num) => sub_num(num, rhs),
			Self::Func(_) => todo!(),
			Self::Matrix(matrix) => todo!(),
		}
	}
}

fn sub_num(num: i64, expr: Expression) -> Expression {
	match expr {
		Expression::Num(other_num) => Expression::Num(num - other_num),
		Expression::Func(_) => todo!(),
		Expression::Matrix(_) => throw("Cannot subtract a matrix from a constant"),
	}
}

impl std::ops::Mul for Expression {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		match self {
			Self::Num(num) => mul_num(num, rhs),
			Self::Func(_) => todo!(),
			Self::Matrix(matrix) => todo!(),
		}
	}
}

fn mul_num(num: i64, expr: Expression) -> Expression {
	match expr {
		Expression::Num(other_num) => Expression::Num(num * other_num),
		Expression::Func(_) => todo!(),
		Expression::Matrix(_) => throw("Cannot subtract a matrix from a constant"),
	}
}

impl std::ops::Div for Expression {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		match self {
			Self::Num(num) => div_num(num, rhs),
			Self::Func(_) => todo!(),
			Self::Matrix(matrix) => todo!(),
		}
	}
}

fn div_num(num: i64, expr: Expression) -> Expression {
	match expr {
		Expression::Num(other_num) => Expression::Num(num / other_num),
		Expression::Func(_) => todo!(),
		Expression::Matrix(_) => throw("Cannot subtract a matrix from a constant"),
	}
}

// Number types

#[derive(Debug, Eq, PartialEq)]
//TODO? Potentially make it so it's only possible to store valid matrices
// i.e. all the columns are the same length
pub struct Matrix {
	data: Vec<Vec<i64>>,
}

impl Add for Matrix {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		let matrix0 = self.data.into_iter();
		let matrix1 = rhs.data.into_iter();
		let zipped = matrix0.zip(matrix1);
		let data = zipped.map(add_vectors).collect();
		Self { data }
	}
}

fn add_vectors((vec0, vec1): (Vec<i64>, Vec<i64>)) -> Vec<i64> {
	let zipped = vec0.into_iter().zip(vec1.into_iter());
	zipped.map(|(x, y)| x + y).collect()
}
