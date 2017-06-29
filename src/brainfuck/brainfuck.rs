use std::collections::VecDeque;

pub struct Vm {
	memory: VecDeque<u8>,
	code: Vec<u8>,
	input: Vec<u8>,
	output: Vec<u8>,
	pc: usize,
	ptr: usize,
}

impl Vm {
	pub fn new(code: Vec<u8>, mut input: Vec<u8>) -> Self {
		input.reverse();
		Vm { memory: VecDeque::new(), code: code, input: input, output: Vec::new(), pc: 0, ptr: 0 }
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