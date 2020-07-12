use super::instruction::Instruction;

pub struct Iter<'a> {
	rom: &'a Rom,
	offset: usize,
}

impl Iterator for Iter<'_> {
	type Item = Instruction;

	fn next(&mut self) -> Option<Instruction> {
		// XXX
		if self.offset >= self.rom.contents.len()-3 {
			return None;
		}

		let instruction = Instruction::decode(&self.rom.contents[self.offset..]);
		self.offset += instruction.length;

		Some(instruction)
	}
}

pub struct Rom {
	pub contents: Vec<u8>,
}

impl Rom {
	pub fn load(contents: Vec<u8>) -> Rom {
		Rom {
			contents
		}
	}

	pub fn instructions(&self) -> Iter {
		Iter {
			rom: &self,
			offset: 0,
		}
	}
}

pub fn load(contents: Vec<u8>) -> Rom {
	Rom::load(contents)
}
