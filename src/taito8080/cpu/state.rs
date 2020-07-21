use super::instruction::*;
use super::execute;

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct ConditionFlags {
	zero: bool,
	sign: bool,
	parity: bool,
	carry: bool,
	aux_carry: bool,
}

#[derive(Default, Debug)]
pub struct State {
	pub a: u8,
	pub f: u8,

	pub b: u8,
	pub c: u8,

	pub d: u8,
	pub e: u8,

	pub h: u8,
	pub l: u8,

	pub sp: u16,
	pub pc: u16,

	pub flags: ConditionFlags,
	pub memory: Box<[u8]>,
	pub int_enable: bool
}

impl State {
	#[allow(unused_variables)]
	pub fn init(mem: Box<[u8]>) -> Self {
		Default::default()
	}

	pub	fn execute(&mut self, instruction: Instruction) {
		execute::execute(self, instruction)
	}
}
