use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::collections::VecDeque;

struct BrainfuckVM {
	memory: VecDeque<u8>,
	code: Vec<u8>,
	input: Vec<u8>,
	output: Vec<u8>,
	pc: usize,
	ptr: usize,
}

impl BrainfuckVM {
	pub fn new(code: Vec<u8>, mut input: Vec<u8>) -> Self {
		input.reverse();
		BrainfuckVM { memory: VecDeque::new(), code: code, input: input, output: Vec::new(), pc: 0, ptr: 0 }
	}

	fn get_next_instruction(&self) -> Option<u8> {
		if self.pc < self.code.len() {
			Some(self.code[self.pc])
		} else {
			None
		}
	}

	fn increment_data_ptr(&mut self) {
		self.ptr += 1;

		// Extend if we've reached the end
		if self.ptr == self.memory.len() {
			self.memory.push_back(0);
		}
	}

	fn decrement_data_ptr(&mut self) {
		if self.ptr > 0 {
			self.ptr -= 1;
		} else {
			/* We're already at the first cell, so we have to push new cell
			 * to the front */
			self.memory.push_front(0);
		}
	}

	fn conditional_jump_forward(&mut self) {
		
	}

	fn conditional_jump_backward(&mut self) {
		
	}

	pub fn execute(&mut self) -> &Vec<u8> {
		loop {

			/* Try to get the next instruction.
			 * Exit if next instruction is not available. */
			let instr;
			match self.get_next_instruction() {
				Some(i) => instr = i,
				None => break,
			}

			// Execute next instruction
			match instr as char {
				'>' => self.increment_data_ptr(),
				'<' => self.decrement_data_ptr(),
				'+' => self.memory[self.ptr] += 1,
				'-' => self.memory[self.ptr] -= 1,
				'.' => self.output.push(self.memory[self.ptr]),
				',' => self.memory[self.ptr] = self.input.pop().unwrap_or(0),
				'[' => self.conditional_jump_forward(),
				']' => self.conditional_jump_backward(),
				_ => continue, // Ignore unknown instruction
			}
		}

		&self.output
	}
}

fn main() {
	let args: Vec<_> = env::args().collect();
	
	// Make sure there's exactly one argument
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

	// Execute
	let mut vm = BrainfuckVM::new(code, input);
	let output = vm.execute();
}
