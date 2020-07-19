use super::instruction;
use super::state;

pub fn unimplemented(state: &mut state::State) {
	panic!("unimplmented instruction");
}

pub fn lxi(state: &mut state::State, dst: instruction::Register, src: u16) {
	panic!("unimplmented instruction");
}

