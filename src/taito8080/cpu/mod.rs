mod execute;

use std::mem;

bitflags! {
	struct ConditionFlags: u8 {
		const SIGN		= 0b00000001;
		const ZERO		= 0b00000010;
		const _PADDING0 = 0b00000100; // always false
		const AUX_CARRY = 0b00001000; // nibble carry
		const _PADDING1 = 0b00010000; // always false
		const PARITY	= 0b00100000; // even parity
		const _PADDING2 = 0b01000000; // always true
		const CARRY		= 0b10000000;
	}
}

impl Default for ConditionFlags {
	fn default() -> Self {
		Self::_PADDING2
	}
}

/*
TODO: Format ConditionFlags.
impl Display for ConditionFlags {
``[-Z-P-]``
}
*/

#[derive(Default, Debug)]
pub struct Registers {
	a: u8,
	f: u8,

	b: u8,
	c: u8,

	d: u8,
	e: u8,

	h: u8,
	l: u8,

	sp: u16,
	pc: u16
}

// FIXME: The compiler is not informed the second variable is referenced.
// TODO: Internal doc tests.
// TODO: Perhaps reimplement with macro_rules.
impl Registers {
	/// Casts a register ref to it's corresponding register pair ref.
	fn pair(&self, register: &u8) -> &u16 {
		let register_ptr = register as *const u8;

		// The given reference has to be a field.
		assert!(register_ptr >= (&self.a as *const u8) &&
				register_ptr <= (&self.l as *const u8));

		// We shouldn't cast two different from different pairs.
		assert!((register_ptr as usize) % 2 == 0,
				"invalid register pair"); 

		unsafe { mem::transmute::<&u8, &u16>(register) }
	}

	/// [#method.pair] for mutable references.
	fn mut_pair(&self, register: &mut u8) -> &mut u16 {
		let register_ptr = register as *const u8;

		// The given reference has to be a field.
		assert!(register_ptr >= (&self.a as *const u8) &&
				register_ptr <= (&self.l as *const u8));

		// We shouldn't cast two different from different pairs.
		assert!((register_ptr as usize) % 2 == 0,
				"invalid register pair"); 

		unsafe { mem::transmute::<&mut u8, &mut u16>(register) }
	}
}

#[derive(Default, Derivative)]
#[derivative(Debug)]
pub struct State {
	registers: Registers,
	int_enable: bool,
	#[derivative(Debug="ignore")]
	memory: Vec<u8>,
}

impl State {
	#[warn(missing_docs)]
	pub fn init(memory: Vec<u8>) -> Self {
		Self { 
			memory, 
			..Default::default() 
		}
	}

	/// Step the emulation of one cpu instruction.
	/// All instructions increases ``state.pc`` by at-least 1 byte.
	///
	/// ```
	/// use taito8080::cpu;
	///
	/// let memory = Vec::with_capacity(4096);
	/// let mut state = cpu::init(memory);
	///
	/// state.step([0xa3, 0xd8]);
	/// ```
	pub fn step(&mut self) {
		execute::execute(self);
	}
}

/*
impl Display for State {
	/* 
	[I]
	PSW = 00fe [-Z-P-] BC = babe DE = dead hl = beef
	SP	= 72de PC = 1337
	 */
	TODO: Format State.
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.)
	}
}
*/

