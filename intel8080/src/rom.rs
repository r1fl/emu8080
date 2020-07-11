use super::instruction::Instruction;

pub struct Iter<'a> {
	rom: &'a Rom,
	offset: usize,
}

impl Iterator for Iter<'_> {
	type Item = Instruction;

	fn next(&mut self) -> Option<Instruction> {
		if self.offset >= self.rom.contents.len() {
			return None;
		}

		let instruction = Instruction::decode(&self.rom.contents[self.offset..]);
		self.offset += instruction.length;

		Some(instruction)
	}
}

pub struct Rom {
	contents: Vec<u8>,
}

impl Rom {
	pub fn load(contents: Vec<u8>) -> Rom {
		Rom {
			contents: contents
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
