use super::*;

use Mnemonic::*;
use Register::*;

macro_rules! unpack16 {
	($bytes:expr) => (
		($bytes[0] as u16) << 0 |
		($bytes[1] as u16) << 8
	);
}

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
			operands: vec![Operand::d8($raw.data[0])],
			raw: $raw,
			mnemonic: $mnemonic,
		}
	);

	($raw:ident; $mnemonic:ident $reg_dst:ident, d16) => (
		Instruction {
			length: 3,
			operands: vec![Operand::d16(unpack16!($raw.data))],
			raw: $raw,
			mnemonic: $mnemonic,
		}
	);

	($raw:ident; $mnemonic:ident $reg_dst:ident, d8) => (
		Instruction {
			length: 2,
			operands: vec![Operand::d8($raw.data[0])],
			raw: $raw,
			mnemonic: $mnemonic,
		}
	);


	($raw:ident; $mnemonic:ident a16) => (
		Instruction {
			length: 3,
			operands: vec![Operand::d16(unpack16!($raw.data))],
			raw: $raw,
			mnemonic: $mnemonic,
		}
	);

	($raw:ident; $mnemonic:ident $reg:ident) => (
		Instruction {
			length: 1,
			raw: $raw,
			mnemonic: $mnemonic,
			operands: vec![Operand::reg($reg)],
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

	($raw:ident; $mnemonic:ident $num:expr) => (
		Instruction {
			length: 1,
			raw: $raw,
			mnemonic: $mnemonic,
			operands: vec![Operand::d8($num)],
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
		0x00 => instruction!(instruction; NOP),
		0x01 => instruction!(instruction; LXI B,d16),
		0x02 => instruction!(instruction; STAX B),
		0x03 => instruction!(instruction; INX B),
		0x04 => instruction!(instruction; INR B),
		0x05 => instruction!(instruction; DCR B),
		0x06 => instruction!(instruction; MVI B,d8),
		0x07 => instruction!(instruction; RLC),
		0x08 => instruction!(instruction; NOP),
		0x09 => instruction!(instruction; DAD B),
		0x0a => instruction!(instruction; LDAX B),
		0x0b => instruction!(instruction; DCX B),
		0x0c => instruction!(instruction; INR C),
		0x0d => instruction!(instruction; DCR C),
		0x0e => instruction!(instruction; MVI C,d8),
		0x0f => instruction!(instruction; RRC),
		0x10 => instruction!(instruction; NOP),
		0x11 => instruction!(instruction; LXI D,d16),
		0x12 => instruction!(instruction; STAX D),
		0x13 => instruction!(instruction; INX D),
		0x14 => instruction!(instruction; INR D),
		0x15 => instruction!(instruction; DCR D),
		0x16 => instruction!(instruction; MVI D,d8),
		0x17 => instruction!(instruction; RAL),
		0x18 => instruction!(instruction; NOP),
		0x19 => instruction!(instruction; DAD D),
		0x1a => instruction!(instruction; LDAX D),
		0x1b => instruction!(instruction; DCX D),
		0x1c => instruction!(instruction; INR E),
		0x1d => instruction!(instruction; DCR E),
		0x1e => instruction!(instruction; MVI E,d8),
		0x1f => instruction!(instruction; RAR),
		0x20 => instruction!(instruction; NOP),
		0x21 => instruction!(instruction; LXI H,d16),
		0x22 => instruction!(instruction; SHLD a16),
		0x23 => instruction!(instruction; INX H),
		0x24 => instruction!(instruction; INR H),
		0x25 => instruction!(instruction; DCR H),
		0x26 => instruction!(instruction; MVI H,d8),
		0x27 => instruction!(instruction; DAA),
		0x28 => instruction!(instruction; NOP),
		0x29 => instruction!(instruction; DAD H),
		0x2a => instruction!(instruction; LHLD a16),
		0x2b => instruction!(instruction; DCX H),
		0x2c => instruction!(instruction; INR L),
		0x2d => instruction!(instruction; DCR L),
		0x2e => instruction!(instruction; MVI L,d8),
		0x2f => instruction!(instruction; CMA),
		0x30 => instruction!(instruction; NOP),
		0x31 => instruction!(instruction; LXI SP,d16),
		0x32 => instruction!(instruction; STA a16),
		0x33 => instruction!(instruction; INX SP),
		0x34 => instruction!(instruction; INR M),
		0x35 => instruction!(instruction; DCR M),
		0x36 => instruction!(instruction; MVI M,d8),
		0x37 => instruction!(instruction; STC),
		0x38 => instruction!(instruction; NOP),
		0x39 => instruction!(instruction; DAD SP),
		0x3a => instruction!(instruction; LDA a16),
		0x3b => instruction!(instruction; DCX SP),
		0x3c => instruction!(instruction; INR A),
		0x3d => instruction!(instruction; DCR A),
		0x3e => instruction!(instruction; MVI A,d8),
		0x3f => instruction!(instruction; CMC),
		0x40 => instruction!(instruction; MOV B,B),
		0x41 => instruction!(instruction; MOV B,C),
		0x42 => instruction!(instruction; MOV B,D),
		0x43 => instruction!(instruction; MOV B,E),
		0x44 => instruction!(instruction; MOV B,H),
		0x45 => instruction!(instruction; MOV B,L),
		0x46 => instruction!(instruction; MOV B,M),
		0x47 => instruction!(instruction; MOV B,A),
		0x48 => instruction!(instruction; MOV C,B),
		0x49 => instruction!(instruction; MOV C,C),
		0x4a => instruction!(instruction; MOV C,D),
		0x4b => instruction!(instruction; MOV C,E),
		0x4c => instruction!(instruction; MOV C,H),
		0x4d => instruction!(instruction; MOV C,L),
		0x4e => instruction!(instruction; MOV C,M),
		0x4f => instruction!(instruction; MOV C,A),
		0x50 => instruction!(instruction; MOV D,B),
		0x51 => instruction!(instruction; MOV D,C),
		0x52 => instruction!(instruction; MOV D,D),
		0x53 => instruction!(instruction; MOV D,E),
		0x54 => instruction!(instruction; MOV D,H),
		0x55 => instruction!(instruction; MOV D,L),
		0x56 => instruction!(instruction; MOV D,M),
		0x57 => instruction!(instruction; MOV D,A),
		0x58 => instruction!(instruction; MOV E,B),
		0x59 => instruction!(instruction; MOV E,C),
		0x5a => instruction!(instruction; MOV E,D),
		0x5b => instruction!(instruction; MOV E,E),
		0x5c => instruction!(instruction; MOV E,H),
		0x5d => instruction!(instruction; MOV E,L),
		0x5e => instruction!(instruction; MOV E,M),
		0x5f => instruction!(instruction; MOV E,A),
		0x60 => instruction!(instruction; MOV H,B),
		0x61 => instruction!(instruction; MOV H,C),
		0x62 => instruction!(instruction; MOV H,D),
		0x63 => instruction!(instruction; MOV H,E),
		0x64 => instruction!(instruction; MOV H,H),
		0x65 => instruction!(instruction; MOV H,L),
		0x66 => instruction!(instruction; MOV H,M),
		0x67 => instruction!(instruction; MOV H,A),
		0x68 => instruction!(instruction; MOV L,B),
		0x69 => instruction!(instruction; MOV L,C),
		0x6a => instruction!(instruction; MOV L,D),
		0x6b => instruction!(instruction; MOV L,E),
		0x6c => instruction!(instruction; MOV L,H),
		0x6d => instruction!(instruction; MOV L,L),
		0x6e => instruction!(instruction; MOV L,M),
		0x6f => instruction!(instruction; MOV L,A),
		0x70 => instruction!(instruction; MOV M,B),
		0x71 => instruction!(instruction; MOV M,C),
		0x72 => instruction!(instruction; MOV M,D),
		0x73 => instruction!(instruction; MOV M,E),
		0x74 => instruction!(instruction; MOV M,H),
		0x75 => instruction!(instruction; MOV M,L),
		0x76 => instruction!(instruction; HLT),
		0x77 => instruction!(instruction; MOV M,A),
		0x78 => instruction!(instruction; MOV A,B),
		0x79 => instruction!(instruction; MOV A,C),
		0x7a => instruction!(instruction; MOV A,D),
		0x7b => instruction!(instruction; MOV A,E),
		0x7c => instruction!(instruction; MOV A,H),
		0x7d => instruction!(instruction; MOV A,L),
		0x7e => instruction!(instruction; MOV A,M),
		0x7f => instruction!(instruction; MOV A,A),
		0x80 => instruction!(instruction; ADD B),
		0x81 => instruction!(instruction; ADD C),
		0x82 => instruction!(instruction; ADD D),
		0x83 => instruction!(instruction; ADD E),
		0x84 => instruction!(instruction; ADD H),
		0x85 => instruction!(instruction; ADD L),
		0x86 => instruction!(instruction; ADD M),
		0x87 => instruction!(instruction; ADD A),
		0x88 => instruction!(instruction; ADC B),
		0x89 => instruction!(instruction; ADC C),
		0x8a => instruction!(instruction; ADC D),
		0x8b => instruction!(instruction; ADC E),
		0x8c => instruction!(instruction; ADC H),
		0x8d => instruction!(instruction; ADC L),
		0x8e => instruction!(instruction; ADC M),
		0x8f => instruction!(instruction; ADC A),
		0x90 => instruction!(instruction; SUB B),
		0x91 => instruction!(instruction; SUB C),
		0x92 => instruction!(instruction; SUB D),
		0x93 => instruction!(instruction; SUB E),
		0x94 => instruction!(instruction; SUB H),
		0x95 => instruction!(instruction; SUB L),
		0x96 => instruction!(instruction; SUB M),
		0x97 => instruction!(instruction; SUB A),
		0x98 => instruction!(instruction; SBB B),
		0x99 => instruction!(instruction; SBB C),
		0x9a => instruction!(instruction; SBB D),
		0x9b => instruction!(instruction; SBB E),
		0x9c => instruction!(instruction; SBB H),
		0x9d => instruction!(instruction; SBB L),
		0x9e => instruction!(instruction; SBB M),
		0x9f => instruction!(instruction; SBB A),
		0xa0 => instruction!(instruction; ANA B),
		0xa1 => instruction!(instruction; ANA C),
		0xa2 => instruction!(instruction; ANA D),
		0xa3 => instruction!(instruction; ANA E),
		0xa4 => instruction!(instruction; ANA H),
		0xa5 => instruction!(instruction; ANA L),
		0xa6 => instruction!(instruction; ANA M),
		0xa7 => instruction!(instruction; ANA A),
		0xa8 => instruction!(instruction; XRA B),
		0xa9 => instruction!(instruction; XRA C),
		0xaa => instruction!(instruction; XRA D),
		0xab => instruction!(instruction; XRA E),
		0xac => instruction!(instruction; XRA H),
		0xad => instruction!(instruction; XRA L),
		0xae => instruction!(instruction; XRA M),
		0xaf => instruction!(instruction; XRA A),
		0xb0 => instruction!(instruction; ORA B),
		0xb1 => instruction!(instruction; ORA C),
		0xb2 => instruction!(instruction; ORA D),
		0xb3 => instruction!(instruction; ORA E),
		0xb4 => instruction!(instruction; ORA H),
		0xb5 => instruction!(instruction; ORA L),
		0xb6 => instruction!(instruction; ORA M),
		0xb7 => instruction!(instruction; ORA A),
		0xb8 => instruction!(instruction; CMP B),
		0xb9 => instruction!(instruction; CMP C),
		0xba => instruction!(instruction; CMP D),
		0xbb => instruction!(instruction; CMP E),
		0xbc => instruction!(instruction; CMP H),
		0xbd => instruction!(instruction; CMP L),
		0xbe => instruction!(instruction; CMP M),
		0xbf => instruction!(instruction; CMP A),
		0xc0 => instruction!(instruction; RNZ),
		0xc1 => instruction!(instruction; POP B),
		0xc2 => instruction!(instruction; JNZ a16),
		0xc3 => instruction!(instruction; JMP a16),
		0xc4 => instruction!(instruction; CNZ a16),
		0xc5 => instruction!(instruction; PUSH B),
		0xc6 => instruction!(instruction; ADI d8),
		0xc7 => instruction!(instruction; RST 0),
		0xc8 => instruction!(instruction; RZ),
		0xc9 => instruction!(instruction; RET),
		0xca => instruction!(instruction; JZ a16),
		0xcb => instruction!(instruction; JMP a16),
		0xcc => instruction!(instruction; CZ a16),
		0xcd => instruction!(instruction; CALL a16),
		0xce => instruction!(instruction; ACI d8),
		0xcf => instruction!(instruction; RST 1),
		0xd0 => instruction!(instruction; RNC),
		0xd1 => instruction!(instruction; POP D),
		0xd2 => instruction!(instruction; JNC a16),
		0xd3 => instruction!(instruction; OUT d8),
		0xd4 => instruction!(instruction; CNC a16),
		0xd5 => instruction!(instruction; PUSH D),
		0xd6 => instruction!(instruction; SUI d8),
		0xd7 => instruction!(instruction; RST 2),
		0xd8 => instruction!(instruction; RC),
		0xd9 => instruction!(instruction; RET),
		0xda => instruction!(instruction; JC a16),
		0xdb => instruction!(instruction; IN d8),
		0xdc => instruction!(instruction; CC a16),
		0xdd => instruction!(instruction; CALL a16),
		0xde => instruction!(instruction; SBI d8),
		0xdf => instruction!(instruction; RST 3),
		0xe0 => instruction!(instruction; RPO),
		0xe1 => instruction!(instruction; POP H),
		0xe2 => instruction!(instruction; JPO a16),
		0xe3 => instruction!(instruction; XTHL),
		0xe4 => instruction!(instruction; CPO a16),
		0xe5 => instruction!(instruction; PUSH H),
		0xe6 => instruction!(instruction; ANI d8),
		0xe7 => instruction!(instruction; RST 4),
		0xe8 => instruction!(instruction; RPE),
		0xe9 => instruction!(instruction; PCHL),
		0xea => instruction!(instruction; JPE a16),
		0xeb => instruction!(instruction; XCHG),
		0xec => instruction!(instruction; CPE a16),
		0xed => instruction!(instruction; CALL a16),
		0xee => instruction!(instruction; XRI d8),
		0xef => instruction!(instruction; RST 5),
		0xf0 => instruction!(instruction; RP),
		0xf1 => instruction!(instruction; POP PSW),
		0xf2 => instruction!(instruction; JP a16),
		0xf3 => instruction!(instruction; DI),
		0xf4 => instruction!(instruction; CP a16),
		0xf5 => instruction!(instruction; PUSH PSW),
		0xf6 => instruction!(instruction; ORI d8),
		0xf7 => instruction!(instruction; RST 6),
		0xf8 => instruction!(instruction; RM),
		0xf9 => instruction!(instruction; SPHL),
		0xfa => instruction!(instruction; JM a16),
		0xfb => instruction!(instruction; EI),
		0xfc => instruction!(instruction; CM a16),
		0xfd => instruction!(instruction; CALL a16),
		0xfe => instruction!(instruction; CPI d8),
		0xff => instruction!(instruction; RST 7),
	}

	/*
	0x1 => instruction!("LXI D, d16", raw_instruction), // TODO: consider quotes for readability
	*/
}
