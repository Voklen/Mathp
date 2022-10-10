pub mod evaluator;
pub mod parser;
pub mod types;

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
