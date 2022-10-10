use mathp::*;

fn main() {
	let files = get_arguments();
	for file_name in files {
		let file = std::fs::read_to_string(file_name).unwrap();
		let parsed = parser::parse(file);
		let eval = evaluator::evaluate(parsed);
		println!("Result: {eval}")
	}
}

fn get_arguments() -> Vec<String> {
	// Skip the first argument because it's just the executable path
	let arguments_as_strings: Vec<String> = std::env::args().skip(1).collect();

	if arguments_as_strings.is_empty() {
		throw_void("missing operand");
	}

	for arg in &arguments_as_strings {
		process_arg(arg)
	}
	return arguments_as_strings;
}

fn process_arg(arg: &str) {
	match arg {
		"-h" => {
			let program_name = env!("CARGO_PKG_NAME");
			let program_ver = env!("CARGO_PKG_VERSION");
			println!("{program_name} {program_ver}");
			println!("Usage: {program_name} files ...");
			std::process::exit(0)
		}
		"--version" => {
			println!(
				"{program_name} {program_ver}",
				program_name = env!("CARGO_PKG_NAME"),
				program_ver = env!("CARGO_PKG_VERSION")
			);
			println!("Copyright (C) 2022 Alexander Gorichev\nLicense GPL-3.0-only: GNU GPL version 3.0 only <https://gnu.org/licenses/gpl-3.0.html>.\nThis is free software: you are free to change and redistribute it.\nThere is NO WARRANTY, to the extent permitted by law.\n\nWritten by Alexander Gorichev.");
			std::process::exit(0)
		}
		_ => {}
	}
}
