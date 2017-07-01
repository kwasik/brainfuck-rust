use std::env;
use std::io::prelude::*;
use std::fs::File;

mod brainfuck;

fn main() {
	let args: Vec<_> = env::args().collect();
	
	// Display usage information if there's either too many or too few args
	if args.len() < 2 || args.len() > 3 {
		println!("Usage: {} code_file input_file", &args[0]);
		println!("\tcode_file: file containing Brainfuck code");
		println!("\tinput_file: file containing input for the Brainfuck program");
		return;
	}

	// Open file containing Brainfuck code
	let mut code: Vec<u8> = Vec::new();
	File::open(&args[1]).unwrap().read_to_end(&mut code).unwrap();

	// Open file containing input for Brainfuck program if provided
	let mut input: Vec<u8> = Vec::new();
	if args.len() == 3 {
		File::open(&args[2]).unwrap().read_to_end(&mut input).unwrap();
	}

	// Create VM and execute
	let mut vm = brainfuck::Vm::new(code, input);
	let output = vm.execute().unwrap();

	// Print output
	let output = std::str::from_utf8(&output).unwrap();

	print!("{}", output);
}
