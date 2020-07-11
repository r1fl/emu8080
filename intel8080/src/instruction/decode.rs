use super::*;

use Mnemonic::*;
use Register::*;

macro_rules! instruction {
	($raw:ident; $mnemonic:ident) => (
		Instruction {
			length: 1,
			raw: $raw,
			mnemonic: $mnemonic,
			operands: vec![],
		}
	);

	($raw:ident; $mnemonic:ident d8) => (
		Instruction {
			length: 2,
			raw: $raw,
			mnemonic: $mnemonic,
			operands: vec![],
		}
	);

	($raw:ident; $mnemonic:ident $reg_dst:ident, d16) => (
		Instruction {
			length: 3,
			raw: $raw,
			mnemonic: $mnemonic,
			operands: vec![],
		}
	);


	($raw:ident; $mnemonic:ident a16) => (
		Instruction {
			length: 3,
			raw: $raw,
			mnemonic: $mnemonic,
			operands: vec![],
		}
	);

	($raw:ident; $mnemonic:ident $reg_dst:ident, $reg_src:ident) => (
		Instruction {
			length: 1,
			raw: $raw,
			mnemonic: $mnemonic,
			operands: vec![Operand::reg($reg_dst), Operand::reg($reg_src)],
		}
	);
}


#[inline(always)]
pub fn decode(bytes: &[u8]) -> Instruction {
	//! Construct a new instruction from raw bytes.
	
	let mut data = [0; 2];
	data.clone_from_slice(&bytes[1..3]);

	let instruction = RawInstruction {
		opcode: bytes[0],
		data: data,
	};

	match instruction.opcode {
		0x1 => instruction!(instruction; LXI B, d16),
		_ => instruction!(instruction; LXI B, d16), // XXX
	}

	/*
	0x1 => instruction!("LXI D, d16", raw_instruction), // TODO: create instruction macro
	0x2 => instruction!(LXI, Operand::Register(A), Operand::Address(raw_instruction.data))
	0x3 => Self::new(LXI, Operand::Register(A), Operand::Address(raw_instruction.data))
	*/
}
