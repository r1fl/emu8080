pub mod rom;
pub use rom::load;

/*
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

/*
instruction!(0x80, im1, im2, "int {}, {}")
*/

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
enum Operations {
	NOP = 0x0,
	LXI = 0x1,
	STAX = 0x2,
	INX = 0x3,
}

use Operations::*;
*/

#[derive(Debug)]
pub struct Instruction {
	size: usize,
	//opcode: Operations,
	opcode: String,
	operand: u16
}


impl Instruction {
	fn decode(bytes: &[u8]) -> Instruction {
		let opcode = bytes[0];

		match opcode {
			0x00 => Instruction {
				size: 1,
				opcode: String::from("NOP"),
				operand: 0
			},
			0x01 => Instruction {
				size: 3,
				opcode: String::from("LXI B,d16"),
				operand: 0
			},
			0x02 => Instruction {
				size: 1,
				opcode: String::from("STAX B"),
				operand: 0
			},
			0x03 => Instruction {
				size: 1,
				opcode: String::from("INX B"),
				operand: 0
			},
			0x04 => Instruction {
				size: 1,
				opcode: String::from("INR B"),
				operand: 0
			},
			0x05 => Instruction {
				size: 1,
				opcode: String::from("DCR B"),
				operand: 0
			},
			0x06 => Instruction {
				size: 2,
				opcode: String::from("MVI B,d8"),
				operand: 0
			},
			0x07 => Instruction {
				size: 1,
				opcode: String::from("RLC"),
				operand: 0
			},
			0x08 => Instruction {
				size: 1,
				opcode: String::from("*NOP"),
				operand: 0
			},
			0x09 => Instruction {
				size: 1,
				opcode: String::from("DAD B"),
				operand: 0
			},
			0x0a => Instruction {
				size: 1,
				opcode: String::from("LDAX B"),
				operand: 0
			},
			0x0b => Instruction {
				size: 1,
				opcode: String::from("DCX B"),
				operand: 0
			},
			0x0c => Instruction {
				size: 1,
				opcode: String::from("INR C"),
				operand: 0
			},
			0x0d => Instruction {
				size: 1,
				opcode: String::from("DCR C"),
				operand: 0
			},
			0x0e => Instruction {
				size: 2,
				opcode: String::from("MVI C,d8"),
				operand: 0
			},
			0x0f => Instruction {
				size: 1,
				opcode: String::from("RRC"),
				operand: 0
			},
			0x10 => Instruction {
				size: 1,
				opcode: String::from("*NOP"),
				operand: 0
			},
			0x11 => Instruction {
				size: 3,
				opcode: String::from("LXI D,d16"),
				operand: 0
			},
			0x12 => Instruction {
				size: 1,
				opcode: String::from("STAX D"),
				operand: 0
			},
			0x13 => Instruction {
				size: 1,
				opcode: String::from("INX D"),
				operand: 0
			},
			0x14 => Instruction {
				size: 1,
				opcode: String::from("INR D"),
				operand: 0
			},
			0x15 => Instruction {
				size: 1,
				opcode: String::from("DCR D"),
				operand: 0
			},
			0x16 => Instruction {
				size: 2,
				opcode: String::from("MVI D,d8"),
				operand: 0
			},
			0x17 => Instruction {
				size: 1,
				opcode: String::from("RAL"),
				operand: 0
			},
			0x18 => Instruction {
				size: 1,
				opcode: String::from("*NOP"),
				operand: 0
			},
			0x19 => Instruction {
				size: 1,
				opcode: String::from("DAD D"),
				operand: 0
			},
			0x1a => Instruction {
				size: 1,
				opcode: String::from("LDAX D"),
				operand: 0
			},
			0x1b => Instruction {
				size: 1,
				opcode: String::from("DCX D"),
				operand: 0
			},
			0x1c => Instruction {
				size: 1,
				opcode: String::from("INR E"),
				operand: 0
			},
			0x1d => Instruction {
				size: 1,
				opcode: String::from("DCR E"),
				operand: 0
			},
			0x1e => Instruction {
				size: 2,
				opcode: String::from("MVI E,d8"),
				operand: 0
			},
			0x1f => Instruction {
				size: 1,
				opcode: String::from("RAR"),
				operand: 0
			},
			0x20 => Instruction {
				size: 1,
				opcode: String::from("*NOP"),
				operand: 0
			},
			0x21 => Instruction {
				size: 3,
				opcode: String::from("LXI H,d16"),
				operand: 0
			},
			0x22 => Instruction {
				size: 1,
				opcode: String::from("SHLD a16"),
				operand: 0
			},
			0x23 => Instruction {
				size: 1,
				opcode: String::from("INX H"),
				operand: 0
			},
			0x24 => Instruction {
				size: 1,
				opcode: String::from("INR H"),
				operand: 0
			},
			0x25 => Instruction {
				size: 1,
				opcode: String::from("DCR H"),
				operand: 0
			},
			0x26 => Instruction {
				size: 2,
				opcode: String::from("MVI H,d8"),
				operand: 0
			},
			0x27 => Instruction {
				size: 1,
				opcode: String::from("DAA"),
				operand: 0
			},
			0x28 => Instruction {
				size: 1,
				opcode: String::from("*NOP"),
				operand: 0
			},
			0x29 => Instruction {
				size: 1,
				opcode: String::from("DAD H"),
				operand: 0
			},
			0x2a => Instruction {
				size: 1,
				opcode: String::from("LHLD a16"),
				operand: 0
			},
			0x2b => Instruction {
				size: 1,
				opcode: String::from("DCX H"),
				operand: 0
			},
			0x2c => Instruction {
				size: 1,
				opcode: String::from("INR L"),
				operand: 0
			},
			0x2d => Instruction {
				size: 1,
				opcode: String::from("DCR L"),
				operand: 0
			},
			0x2e => Instruction {
				size: 2,
				opcode: String::from("MVI L,d8"),
				operand: 0
			},
			0x2f => Instruction {
				size: 1,
				opcode: String::from("CMA"),
				operand: 0
			},
			0x30 => Instruction {
				size: 1,
				opcode: String::from("*NOP"),
				operand: 0
			},
			0x31 => Instruction {
				size: 3,
				opcode: String::from("LXI SP,d16"),
				operand: 0
			},
			0x32 => Instruction {
				size: 1,
				opcode: String::from("STA a16"),
				operand: 0
			},
			0x33 => Instruction {
				size: 1,
				opcode: String::from("INX SP"),
				operand: 0
			},
			0x34 => Instruction {
				size: 1,
				opcode: String::from("INR M"),
				operand: 0
			},
			0x35 => Instruction {
				size: 1,
				opcode: String::from("DCR M"),
				operand: 0
			},
			0x36 => Instruction {
				size: 2,
				opcode: String::from("MVI M,d8"),
				operand: 0
			},
			0x37 => Instruction {
				size: 1,
				opcode: String::from("STC"),
				operand: 0
			},
			0x38 => Instruction {
				size: 1,
				opcode: String::from("*NOP"),
				operand: 0
			},
			0x39 => Instruction {
				size: 1,
				opcode: String::from("DAD SP"),
				operand: 0
			},
			0x3a => Instruction {
				size: 1,
				opcode: String::from("LDA a16"),
				operand: 0
			},
			0x3b => Instruction {
				size: 1,
				opcode: String::from("DCX SP"),
				operand: 0
			},
			0x3c => Instruction {
				size: 1,
				opcode: String::from("INR A"),
				operand: 0
			},
			0x3d => Instruction {
				size: 1,
				opcode: String::from("DCR A"),
				operand: 0
			},
			0x3e => Instruction {
				size: 2,
				opcode: String::from("MVI A,d8"),
				operand: 0
			},
			0x3f => Instruction {
				size: 1,
				opcode: String::from("CMC"),
				operand: 0
			},
			0x40 => Instruction {
				size: 1,
				opcode: String::from("MOV B,B"),
				operand: 0
			},
			0x41 => Instruction {
				size: 1,
				opcode: String::from("MOV B,C"),
				operand: 0
			},
			0x42 => Instruction {
				size: 1,
				opcode: String::from("MOV B,D"),
				operand: 0
			},
			0x43 => Instruction {
				size: 1,
				opcode: String::from("MOV B,E"),
				operand: 0
			},
			0x44 => Instruction {
				size: 1,
				opcode: String::from("MOV B,H"),
				operand: 0
			},
			0x45 => Instruction {
				size: 1,
				opcode: String::from("MOV B,L"),
				operand: 0
			},
			0x46 => Instruction {
				size: 1,
				opcode: String::from("MOV B,M"),
				operand: 0
			},
			0x47 => Instruction {
				size: 1,
				opcode: String::from("MOV B,A"),
				operand: 0
			},
			0x48 => Instruction {
				size: 1,
				opcode: String::from("MOV C,B"),
				operand: 0
			},
			0x49 => Instruction {
				size: 1,
				opcode: String::from("MOV C,C"),
				operand: 0
			},
			0x4a => Instruction {
				size: 1,
				opcode: String::from("MOV C,D"),
				operand: 0
			},
			0x4b => Instruction {
				size: 1,
				opcode: String::from("MOV C,E"),
				operand: 0
			},
			0x4c => Instruction {
				size: 1,
				opcode: String::from("MOV C,H"),
				operand: 0
			},
			0x4d => Instruction {
				size: 1,
				opcode: String::from("MOV C,L"),
				operand: 0
			},
			0x4e => Instruction {
				size: 1,
				opcode: String::from("MOV C,M"),
				operand: 0
			},
			0x4f => Instruction {
				size: 1,
				opcode: String::from("MOV C,A"),
				operand: 0
			},
			0x50 => Instruction {
				size: 1,
				opcode: String::from("MOV D,B"),
				operand: 0
			},
			0x51 => Instruction {
				size: 1,
				opcode: String::from("MOV D,C"),
				operand: 0
			},
			0x52 => Instruction {
				size: 1,
				opcode: String::from("MOV D,D"),
				operand: 0
			},
			0x53 => Instruction {
				size: 1,
				opcode: String::from("MOV D,E"),
				operand: 0
			},
			0x54 => Instruction {
				size: 1,
				opcode: String::from("MOV D,H"),
				operand: 0
			},
			0x55 => Instruction {
				size: 1,
				opcode: String::from("MOV D,L"),
				operand: 0
			},
			0x56 => Instruction {
				size: 1,
				opcode: String::from("MOV D,M"),
				operand: 0
			},
			0x57 => Instruction {
				size: 1,
				opcode: String::from("MOV D,A"),
				operand: 0
			},
			0x58 => Instruction {
				size: 1,
				opcode: String::from("MOV E,B"),
				operand: 0
			},
			0x59 => Instruction {
				size: 1,
				opcode: String::from("MOV E,C"),
				operand: 0
			},
			0x5a => Instruction {
				size: 1,
				opcode: String::from("MOV E,D"),
				operand: 0
			},
			0x5b => Instruction {
				size: 1,
				opcode: String::from("MOV E,E"),
				operand: 0
			},
			0x5c => Instruction {
				size: 1,
				opcode: String::from("MOV E,H"),
				operand: 0
			},
			0x5d => Instruction {
				size: 1,
				opcode: String::from("MOV E,L"),
				operand: 0
			},
			0x5e => Instruction {
				size: 1,
				opcode: String::from("MOV E,M"),
				operand: 0
			},
			0x5f => Instruction {
				size: 1,
				opcode: String::from("MOV E,A"),
				operand: 0
			},
			0x60 => Instruction {
				size: 1,
				opcode: String::from("MOV H,B"),
				operand: 0
			},
			0x61 => Instruction {
				size: 1,
				opcode: String::from("MOV H,C"),
				operand: 0
			},
			0x62 => Instruction {
				size: 1,
				opcode: String::from("MOV H,D"),
				operand: 0
			},
			0x63 => Instruction {
				size: 1,
				opcode: String::from("MOV H,E"),
				operand: 0
			},
			0x64 => Instruction {
				size: 1,
				opcode: String::from("MOV H,H"),
				operand: 0
			},
			0x65 => Instruction {
				size: 1,
				opcode: String::from("MOV H,L"),
				operand: 0
			},
			0x66 => Instruction {
				size: 1,
				opcode: String::from("MOV H,M"),
				operand: 0
			},
			0x67 => Instruction {
				size: 1,
				opcode: String::from("MOV H,A"),
				operand: 0
			},
			0x68 => Instruction {
				size: 1,
				opcode: String::from("MOV L,B"),
				operand: 0
			},
			0x69 => Instruction {
				size: 1,
				opcode: String::from("MOV L,C"),
				operand: 0
			},
			0x6a => Instruction {
				size: 1,
				opcode: String::from("MOV L,D"),
				operand: 0
			},
			0x6b => Instruction {
				size: 1,
				opcode: String::from("MOV L,E"),
				operand: 0
			},
			0x6c => Instruction {
				size: 1,
				opcode: String::from("MOV L,H"),
				operand: 0
			},
			0x6d => Instruction {
				size: 1,
				opcode: String::from("MOV L,L"),
				operand: 0
			},
			0x6e => Instruction {
				size: 1,
				opcode: String::from("MOV L,M"),
				operand: 0
			},
			0x6f => Instruction {
				size: 1,
				opcode: String::from("MOV L,A"),
				operand: 0
			},
			0x70 => Instruction {
				size: 1,
				opcode: String::from("MOV M,B"),
				operand: 0
			},
			0x71 => Instruction {
				size: 1,
				opcode: String::from("MOV M,C"),
				operand: 0
			},
			0x72 => Instruction {
				size: 1,
				opcode: String::from("MOV M,D"),
				operand: 0
			},
			0x73 => Instruction {
				size: 1,
				opcode: String::from("MOV M,E"),
				operand: 0
			},
			0x74 => Instruction {
				size: 1,
				opcode: String::from("MOV M,H"),
				operand: 0
			},
			0x75 => Instruction {
				size: 1,
				opcode: String::from("MOV M,L"),
				operand: 0
			},
			0x76 => Instruction {
				size: 1,
				opcode: String::from("HLT"),
				operand: 0
			},
			0x77 => Instruction {
				size: 1,
				opcode: String::from("MOV M,A"),
				operand: 0
			},
			0x78 => Instruction {
				size: 1,
				opcode: String::from("MOV A,B"),
				operand: 0
			},
			0x79 => Instruction {
				size: 1,
				opcode: String::from("MOV A,C"),
				operand: 0
			},
			0x7a => Instruction {
				size: 1,
				opcode: String::from("MOV A,D"),
				operand: 0
			},
			0x7b => Instruction {
				size: 1,
				opcode: String::from("MOV A,E"),
				operand: 0
			},
			0x7c => Instruction {
				size: 1,
				opcode: String::from("MOV A,H"),
				operand: 0
			},
			0x7d => Instruction {
				size: 1,
				opcode: String::from("MOV A,L"),
				operand: 0
			},
			0x7e => Instruction {
				size: 1,
				opcode: String::from("MOV A,M"),
				operand: 0
			},
			0x7f => Instruction {
				size: 1,
				opcode: String::from("MOV A,A"),
				operand: 0
			},
			0x80 => Instruction {
				size: 1,
				opcode: String::from("ADD B"),
				operand: 0
			},
			0x81 => Instruction {
				size: 1,
				opcode: String::from("ADD C"),
				operand: 0
			},
			0x82 => Instruction {
				size: 1,
				opcode: String::from("ADD D"),
				operand: 0
			},
			0x83 => Instruction {
				size: 1,
				opcode: String::from("ADD E"),
				operand: 0
			},
			0x84 => Instruction {
				size: 1,
				opcode: String::from("ADD H"),
				operand: 0
			},
			0x85 => Instruction {
				size: 1,
				opcode: String::from("ADD L"),
				operand: 0
			},
			0x86 => Instruction {
				size: 1,
				opcode: String::from("ADD M"),
				operand: 0
			},
			0x87 => Instruction {
				size: 1,
				opcode: String::from("ADD A"),
				operand: 0
			},
			0x88 => Instruction {
				size: 1,
				opcode: String::from("ADC B"),
				operand: 0
			},
			0x89 => Instruction {
				size: 1,
				opcode: String::from("ADC C"),
				operand: 0
			},
			0x8a => Instruction {
				size: 1,
				opcode: String::from("ADC D"),
				operand: 0
			},
			0x8b => Instruction {
				size: 1,
				opcode: String::from("ADC E"),
				operand: 0
			},
			0x8c => Instruction {
				size: 1,
				opcode: String::from("ADC H"),
				operand: 0
			},
			0x8d => Instruction {
				size: 1,
				opcode: String::from("ADC L"),
				operand: 0
			},
			0x8e => Instruction {
				size: 1,
				opcode: String::from("ADC M"),
				operand: 0
			},
			0x8f => Instruction {
				size: 1,
				opcode: String::from("ADC A"),
				operand: 0
			},
			0x90 => Instruction {
				size: 1,
				opcode: String::from("SUB B"),
				operand: 0
			},
			0x91 => Instruction {
				size: 1,
				opcode: String::from("SUB C"),
				operand: 0
			},
			0x92 => Instruction {
				size: 1,
				opcode: String::from("SUB D"),
				operand: 0
			},
			0x93 => Instruction {
				size: 1,
				opcode: String::from("SUB E"),
				operand: 0
			},
			0x94 => Instruction {
				size: 1,
				opcode: String::from("SUB H"),
				operand: 0
			},
			0x95 => Instruction {
				size: 1,
				opcode: String::from("SUB L"),
				operand: 0
			},
			0x96 => Instruction {
				size: 1,
				opcode: String::from("SUB M"),
				operand: 0
			},
			0x97 => Instruction {
				size: 1,
				opcode: String::from("SUB A"),
				operand: 0
			},
			0x98 => Instruction {
				size: 1,
				opcode: String::from("SBB B"),
				operand: 0
			},
			0x99 => Instruction {
				size: 1,
				opcode: String::from("SBB C"),
				operand: 0
			},
			0x9a => Instruction {
				size: 1,
				opcode: String::from("SBB D"),
				operand: 0
			},
			0x9b => Instruction {
				size: 1,
				opcode: String::from("SBB E"),
				operand: 0
			},
			0x9c => Instruction {
				size: 1,
				opcode: String::from("SBB H"),
				operand: 0
			},
			0x9d => Instruction {
				size: 1,
				opcode: String::from("SBB L"),
				operand: 0
			},
			0x9e => Instruction {
				size: 1,
				opcode: String::from("SBB M"),
				operand: 0
			},
			0x9f => Instruction {
				size: 1,
				opcode: String::from("SBB A"),
				operand: 0
			},
			0xa0 => Instruction {
				size: 1,
				opcode: String::from("ANA B"),
				operand: 0
			},
			0xa1 => Instruction {
				size: 1,
				opcode: String::from("ANA C"),
				operand: 0
			},
			0xa2 => Instruction {
				size: 1,
				opcode: String::from("ANA D"),
				operand: 0
			},
			0xa3 => Instruction {
				size: 1,
				opcode: String::from("ANA E"),
				operand: 0
			},
			0xa4 => Instruction {
				size: 1,
				opcode: String::from("ANA H"),
				operand: 0
			},
			0xa5 => Instruction {
				size: 1,
				opcode: String::from("ANA L"),
				operand: 0
			},
			0xa6 => Instruction {
				size: 1,
				opcode: String::from("ANA M"),
				operand: 0
			},
			0xa7 => Instruction {
				size: 1,
				opcode: String::from("ANA A"),
				operand: 0
			},
			0xa8 => Instruction {
				size: 1,
				opcode: String::from("XRA B"),
				operand: 0
			},
			0xa9 => Instruction {
				size: 1,
				opcode: String::from("XRA C"),
				operand: 0
			},
			0xaa => Instruction {
				size: 1,
				opcode: String::from("XRA D"),
				operand: 0
			},
			0xab => Instruction {
				size: 1,
				opcode: String::from("XRA E"),
				operand: 0
			},
			0xac => Instruction {
				size: 1,
				opcode: String::from("XRA H"),
				operand: 0
			},
			0xad => Instruction {
				size: 1,
				opcode: String::from("XRA L"),
				operand: 0
			},
			0xae => Instruction {
				size: 1,
				opcode: String::from("XRA M"),
				operand: 0
			},
			0xaf => Instruction {
				size: 1,
				opcode: String::from("XRA A"),
				operand: 0
			},
			0xb0 => Instruction {
				size: 1,
				opcode: String::from("ORA B"),
				operand: 0
			},
			0xb1 => Instruction {
				size: 1,
				opcode: String::from("ORA C"),
				operand: 0
			},
			0xb2 => Instruction {
				size: 1,
				opcode: String::from("ORA D"),
				operand: 0
			},
			0xb3 => Instruction {
				size: 1,
				opcode: String::from("ORA E"),
				operand: 0
			},
			0xb4 => Instruction {
				size: 1,
				opcode: String::from("ORA H"),
				operand: 0
			},
			0xb5 => Instruction {
				size: 1,
				opcode: String::from("ORA L"),
				operand: 0
			},
			0xb6 => Instruction {
				size: 1,
				opcode: String::from("ORA M"),
				operand: 0
			},
			0xb7 => Instruction {
				size: 1,
				opcode: String::from("ORA A"),
				operand: 0
			},
			0xb8 => Instruction {
				size: 1,
				opcode: String::from("CMP B"),
				operand: 0
			},
			0xb9 => Instruction {
				size: 1,
				opcode: String::from("CMP C"),
				operand: 0
			},
			0xba => Instruction {
				size: 1,
				opcode: String::from("CMP D"),
				operand: 0
			},
			0xbb => Instruction {
				size: 1,
				opcode: String::from("CMP E"),
				operand: 0
			},
			0xbc => Instruction {
				size: 1,
				opcode: String::from("CMP H"),
				operand: 0
			},
			0xbd => Instruction {
				size: 1,
				opcode: String::from("CMP L"),
				operand: 0
			},
			0xbe => Instruction {
				size: 1,
				opcode: String::from("CMP M"),
				operand: 0
			},
			0xbf => Instruction {
				size: 1,
				opcode: String::from("CMP A"),
				operand: 0
			},
			0xc0 => Instruction {
				size: 1,
				opcode: String::from("RNZ"),
				operand: 0
			},
			0xc1 => Instruction {
				size: 1,
				opcode: String::from("POP B"),
				operand: 0
			},
			0xc2 => Instruction {
				size: 1,
				opcode: String::from("JNZ a16"),
				operand: 0
			},
			0xc3 => Instruction {
				size: 1,
				opcode: String::from("JMP a16"),
				operand: 0
			},
			0xc4 => Instruction {
				size: 1,
				opcode: String::from("CNZ a16"),
				operand: 0
			},
			0xc5 => Instruction {
				size: 1,
				opcode: String::from("PUSH B"),
				operand: 0
			},
			0xc6 => Instruction {
				size: 2,
				opcode: String::from("ADI d8"),
				operand: 0
			},
			0xc7 => Instruction {
				size: 1,
				opcode: String::from("RST 0"),
				operand: 0
			},
			0xc8 => Instruction {
				size: 1,
				opcode: String::from("RZ"),
				operand: 0
			},
			0xc9 => Instruction {
				size: 1,
				opcode: String::from("RET"),
				operand: 0
			},
			0xca => Instruction {
				size: 1,
				opcode: String::from("JZ a16"),
				operand: 0
			},
			0xcb => Instruction {
				size: 1,
				opcode: String::from("*JMP a16"),
				operand: 0
			},
			0xcc => Instruction {
				size: 1,
				opcode: String::from("CZ a16"),
				operand: 0
			},
			0xcd => Instruction {
				size: 1,
				opcode: String::from("CALL a16"),
				operand: 0
			},
			0xce => Instruction {
				size: 2,
				opcode: String::from("ACI d8"),
				operand: 0
			},
			0xcf => Instruction {
				size: 1,
				opcode: String::from("RST 1"),
				operand: 0
			},
			0xd0 => Instruction {
				size: 1,
				opcode: String::from("RNC"),
				operand: 0
			},
			0xd1 => Instruction {
				size: 1,
				opcode: String::from("POP D"),
				operand: 0
			},
			0xd2 => Instruction {
				size: 1,
				opcode: String::from("JNC a16"),
				operand: 0
			},
			0xd3 => Instruction {
				size: 2,
				opcode: String::from("OUT d8"),
				operand: 0
			},
			0xd4 => Instruction {
				size: 1,
				opcode: String::from("CNC a16"),
				operand: 0
			},
			0xd5 => Instruction {
				size: 1,
				opcode: String::from("PUSH D"),
				operand: 0
			},
			0xd6 => Instruction {
				size: 2,
				opcode: String::from("SUI d8"),
				operand: 0
			},
			0xd7 => Instruction {
				size: 1,
				opcode: String::from("RST 2"),
				operand: 0
			},
			0xd8 => Instruction {
				size: 1,
				opcode: String::from("RC"),
				operand: 0
			},
			0xd9 => Instruction {
				size: 1,
				opcode: String::from("*RET"),
				operand: 0
			},
			0xda => Instruction {
				size: 1,
				opcode: String::from("JC a16"),
				operand: 0
			},
			0xdb => Instruction {
				size: 2,
				opcode: String::from("IN d8"),
				operand: 0
			},
			0xdc => Instruction {
				size: 1,
				opcode: String::from("CC a16"),
				operand: 0
			},
			0xdd => Instruction {
				size: 1,
				opcode: String::from("*CALL a16"),
				operand: 0
			},
			0xde => Instruction {
				size: 2,
				opcode: String::from("SBI d8"),
				operand: 0
			},
			0xdf => Instruction {
				size: 1,
				opcode: String::from("RST 3"),
				operand: 0
			},
			0xe0 => Instruction {
				size: 1,
				opcode: String::from("RPO"),
				operand: 0
			},
			0xe1 => Instruction {
				size: 1,
				opcode: String::from("POP H"),
				operand: 0
			},
			0xe2 => Instruction {
				size: 1,
				opcode: String::from("JPO a16"),
				operand: 0
			},
			0xe3 => Instruction {
				size: 1,
				opcode: String::from("XTHL"),
				operand: 0
			},
			0xe4 => Instruction {
				size: 1,
				opcode: String::from("CPO a16"),
				operand: 0
			},
			0xe5 => Instruction {
				size: 1,
				opcode: String::from("PUSH H"),
				operand: 0
			},
			0xe6 => Instruction {
				size: 2,
				opcode: String::from("ANI d8"),
				operand: 0
			},
			0xe7 => Instruction {
				size: 1,
				opcode: String::from("RST 4"),
				operand: 0
			},
			0xe8 => Instruction {
				size: 1,
				opcode: String::from("RPE"),
				operand: 0
			},
			0xe9 => Instruction {
				size: 1,
				opcode: String::from("PCHL"),
				operand: 0
			},
			0xea => Instruction {
				size: 1,
				opcode: String::from("JPE a16"),
				operand: 0
			},
			0xeb => Instruction {
				size: 1,
				opcode: String::from("XCHG"),
				operand: 0
			},
			0xec => Instruction {
				size: 1,
				opcode: String::from("CPE a16"),
				operand: 0
			},
			0xed => Instruction {
				size: 1,
				opcode: String::from("*CALL a16"),
				operand: 0
			},
			0xee => Instruction {
				size: 2,
				opcode: String::from("XRI d8"),
				operand: 0
			},
			0xef => Instruction {
				size: 1,
				opcode: String::from("RST 5"),
				operand: 0
			},
			0xf0 => Instruction {
				size: 1,
				opcode: String::from("RP"),
				operand: 0
			},
			0xf1 => Instruction {
				size: 1,
				opcode: String::from("POP PSW"),
				operand: 0
			},
			0xf2 => Instruction {
				size: 1,
				opcode: String::from("JP a16"),
				operand: 0
			},
			0xf3 => Instruction {
				size: 1,
				opcode: String::from("DI"),
				operand: 0
			},
			0xf4 => Instruction {
				size: 1,
				opcode: String::from("CP a16"),
				operand: 0
			},
			0xf5 => Instruction {
				size: 1,
				opcode: String::from("PUSH PSW"),
				operand: 0
			},
			0xf6 => Instruction {
				size: 2,
				opcode: String::from("ORI d8"),
				operand: 0
			},
			0xf7 => Instruction {
				size: 1,
				opcode: String::from("RST 6"),
				operand: 0
			},
			0xf8 => Instruction {
				size: 1,
				opcode: String::from("RM"),
				operand: 0
			},
			0xf9 => Instruction {
				size: 1,
				opcode: String::from("SPHL"),
				operand: 0
			},
			0xfa => Instruction {
				size: 1,
				opcode: String::from("JM a16"),
				operand: 0
			},
			0xfb => Instruction {
				size: 1,
				opcode: String::from("EI"),
				operand: 0
			},
			0xfc => Instruction {
				size: 1,
				opcode: String::from("CM a16"),
				operand: 0
			},
			0xfd => Instruction {
				size: 1,
				opcode: String::from("*CALL a16"),
				operand: 0
			},
			0xfe => Instruction {
				size: 2,
				opcode: String::from("CPI d8"),
				operand: 0
			},
			0xff => Instruction {
				size: 1,
				opcode: String::from("RST 7"),
				operand: 0
			
			},
		}
		
	}
}


#[test]
fn decode_test() {
	let mut rom: Vec<u8> = vec![0; 3];

	for op in 0..255 {
		rom[0] = op;
		println!("{:?}", Instruction::decode(&rom));
	}
}

