use super::instruction::*;
use super::execute;

use Operand::*;
use Operands::*;

#[derive(Default)]
struct ConditionFlags {
	Zero: bool,
	Sign: bool,
	Parity: bool,
	Carry: bool,
	AuxCarry: bool,
}

// TODO: define register pairs as a union 
#[derive(Default)]
pub struct State {
	condition: ConditionFlags,
	a: u16,
	f: u8,
	b: u16,
	c: u8,
	d: u16,
	e: u8,
	h: u16,
	l: u8,
	sp: u16,
	pc: u16,
	memory: Box<[u8]>,
	int_enable: bool
}

impl State {
	pub fn init(mem: Box<[u8]>) -> Self {
		Default::default()
	}

	pub	fn execute(&mut self, instruction: Instruction) {
		use Mnemonic::*;

		match instruction {
			/*
			Instruction { 
				mnemonic: LXI, 
				operands: Two(Reg(dst), D16(src)), .. 
			} => execute::lxi(self, dst, src), 
			*/

			//instruction!(_; LXI B,d16; Two(Reg(dst), D16(src))) => execute::unimplemented(self)
			//instruction!(_; LXI B,d16; lxi) => execute::unimplemented(self)

			_ => execute::unimplemented(self),
		};
	}
}
