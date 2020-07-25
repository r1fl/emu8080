use super::{State, ConditionFlags};

pub fn execute(state: &mut State) {
	let instruction = &state.memory[usize::from(state.registers.pc)..];
	let opcode: u8 = instruction[0];

	match opcode {
		// ...
		_	=> unimplemented!(),
	};
}
