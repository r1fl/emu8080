use super::instruction::*;
use super::execute;

#[derive(Debug, Default)]
#[repr(C)]
pub struct ConditionFlags {
	pub sign: bool,
	pub zero: bool,
	_padding0: bool,		// always false
	pub aux_carry: bool,	// 4th bit
	_padding1: bool,		// always false
	pub parity: bool,		// even parity
	_padding2: bool,		// always true
	pub carry: bool,
}

impl ConditionFlags {
	pub fn new() -> Self {
		Self { 
			_padding0: false,
			_padding1: false,
			_padding2: true,
			..Default::default() 
		}
	}
}

#[derive(Default, Debug)]
pub struct State {
	pub a: u8,
	pub flags: ConditionFlags,
	//pub f: u8,

	pub b: u8,
	pub c: u8,

	pub d: u8,
	pub e: u8,

	pub h: u8,
	pub l: u8,

	pub sp: u16,
	pub pc: u16,

	pub memory: Box<[u8]>,
	pub int_enable: bool
}

impl State {
	#[allow(unused_variables)]
	pub fn init(mem: Box<[u8]>) -> Self {
		Self {
			flags: ConditionFlags::new(),
			..Default::default()
		}
	}

	pub	fn execute(&mut self, instruction: Instruction) {
		execute::execute(self, instruction)
	}
}
