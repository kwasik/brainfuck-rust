use std::collections::VecDeque;
use std::fmt;

pub struct Vm {
	memory: VecDeque<u8>,
	code: Vec<u8>,
	input: Vec<u8>,
	output: Vec<u8>,
	pc: usize,
	ptr: usize,
}

pub enum VmError {
	PCOutOfScope,
	PCOverflow,
	PCUnderflow,
	UnmatchedForwardJump,
	UnmatchedBackwardJump,
}

impl fmt::Debug for VmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
        	VmError::PCOutOfScope => write!(f, "Vm Error: PC out of scope."),
        	VmError::PCOverflow => write!(f, "Vm Error: PC overflow."),
        	VmError::PCUnderflow => write!(f, "Vm Error: PC underflow."),
        	VmError::UnmatchedForwardJump => write!(f, "Vm Error: Unmatched forward jump."),
        	VmError::UnmatchedBackwardJump => write!(f, "Vm Error: Unmatched backward jump."),
        }
    }
}

impl Vm {
	pub fn new(code: Vec<u8>, mut input: Vec<u8>) -> Self {
		input.reverse();

		let mut mem = VecDeque::new();
		mem.push_front(0);

		Vm { memory: mem, code: code, input: input, output: Vec::new(), pc: 0, ptr: 0 }
	}

	fn jump_relative(&mut self, relative_position: i32) -> Result<(), VmError> {

		/* Move program counter to new position checking for overflow/underflow */
		if relative_position >= 0 {
			match self.pc.checked_add(relative_position as usize) {
				Some(pc) => self.pc = pc,
				None => return Err(VmError::PCOverflow),
			}
		} else {
			match self.pc.checked_sub((-relative_position) as usize) {
				Some(pc) => self.pc = pc,
				None => return Err(VmError::PCUnderflow),
			}
		}

		Ok(())
	}

	fn get_instruction(&self) -> Result<u8, VmError> {

		/* Return current instruction */
		if self.pc < self.code.len() {
			Ok(self.code[self.pc])
		} else {
			Err(VmError::PCOutOfScope)
		}
	}

	fn increment_data_ptr(&mut self) {
		self.ptr += 1;

		/* Extend if we've reached the end */
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

	fn cond_jump_forward(&mut self) -> Result<(), VmError> {

		/* Jump forward only if current memory cell has value of zero */
		if self.memory[self.ptr] == 0 {

			/* Move program counter to the next instruction (the one after '[') */
			self.jump_relative(1).unwrap();

			/* Jump to the matching ']' sign */
			match self.jump_to_matching_instruction('[', ']', false) {
				Ok(_) => {
					/* We found the end of the block. Now we have to
					 * move program to the first instruction after
					 * the block's ending. */
					 self.jump_relative(1).unwrap();
				},
				Err(_) => return Err(VmError::UnmatchedForwardJump),
			}
		}

		Ok(())
	}

	fn jump_backward(&mut self) -> Result<(), VmError> {

		/* Move program counter to the previous instruction */
		self.jump_relative(-1).unwrap();

		/* Jump to the matching '[' sign */
		match self.jump_to_matching_instruction(']', '[', true) {
			Ok(_) => {
				/* Position program counter so that it points to the instruction
				 * before '[' */
				self.jump_relative(-1).unwrap();
				Ok(())
			},
			Err(_) => return Err(VmError::UnmatchedBackwardJump),
		}
	}

	/* Moves PC to the instruction ending current block (end_instr).
	 * Each start_instr is treated as a beginning of nested block and therefore
	 * next instruction ending block (end_instr) is ignored. */
	fn jump_to_matching_instruction(&mut self, start_instr: char, end_instr: char, backward: bool) -> Result<(), ()> {
		let mut depth = 0;

		let mut step = 1;
		if backward {
			step = -1;
		}

		/* Move in given direction until matching instruction ending current
		 * block is found. Skip instructions ending nested blocks. */
		loop {

			/* Get current instruction.
			 * Return error if it's not available since we still haven't
			 * found the matching instruction. */
			let instr;
			match self.get_instruction() {
				Ok(i) => instr = i as char,
				Err(_) => {
					return Err(());
				}
			}

			if instr == end_instr {
				/* Check if that's our matching instruction or just the end
				 * of a nested block. */
				if depth == 0 {
					break; /* That's it. Found it! */
				} else {
					depth -= 1;
				}
			} else if instr == start_instr {
				/* We've encountered the beginning of a nested block.
				 * Increase depth. */
				 depth += 1;
			}

			/* Jump to the next instruction */
			match self.jump_relative(step) {
				Ok(_) => {},
				Err(_) => {
					return Err(());
				}
			}
		}

		Ok(())
	}

	pub fn execute(&mut self) -> Result<&Vec<u8>, VmError> {

		loop {

			/* Try to get current instruction. */
			let instr;
			match self.get_instruction() {
				Ok(i) => instr =  i as char,
				Err(_) => {
					break /* Reached the end of the program */
				}
			}

			/* Execute current instruction */
			match instr {
				'>' => self.increment_data_ptr(),
				'<' => self.decrement_data_ptr(),
				'+' => self.memory[self.ptr] += 1, /* Increase value of the current memory cell */
				'-' => self.memory[self.ptr] -= 1, /* Decrease value of the current memory cell */
				'.' => self.output.push(self.memory[self.ptr]),
				',' => self.memory[self.ptr] = self.input.pop().unwrap_or(0),
				'[' => match self.cond_jump_forward() {
					Err(e) => return Err(e),
					Ok(_) => {},
				},
				']' => match self.jump_backward() {
					Err(e) => return Err(e),
					Ok(_) => {},
				},
				_ => { /* Ignore unknown instruction */	}
			}

			/* Move program counter to the next instruction */
			self.jump_relative(1).unwrap();
		}

		Ok(&self.output)
	}
}