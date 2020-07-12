mod decode;

use std::fmt;

#[derive(Debug)]
enum Mnemonic {
	ACI, ADC, ADD, ADI, ANA, ANI, CALL, CC, CM, CMA,
	CMC, CMP, CNC, CNZ, CP, CPE, CPI, CPO, CZ, DAA,
	DAD, DCR, DCX, DI, EI, HLT, IN, INR, INX, JC, JM,
	JMP, JNC, JNZ, JP, JPE, JPO, JZ, LDA, LDAX, LHLD,
	LXI, MOV, MVI, NOP, ORA, ORI, OUT, PCHL, POP,
	PUSH, RAL, RAR, RC, RET, RLC, RM, RNC, RNZ, RP,
	RPE, RPO, RRC, RST, RZ, SBB, SBI, SHLD, SPHL, STA,
	STAX, STC, SUB, SUI, XCHG, XRA, XRI, XTHL,
}

#[derive(Debug)]
enum Register {
	A, B, C, D, E, H, L,
	M, PSW, SP, // Psuedo-Registers
}

#[derive(Debug)]
enum Operand {
	Reg(Register),
	D8(u8),
	D16(u16),
	A16(u16),
}

impl fmt::Display for Operand {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Operand::Reg(val) => write!(f, "{:?}", val),
			Operand::D8(val) => write!(f, "{:#x}", val),
			Operand::D16(val) => write!(f, "{:#x}", val),
			Operand::A16(val) => write!(f, "{:#x}", val),
		}
	}
}

#[derive(Debug)]
pub struct RawInstruction {
	opcode: u8,
	data: [u8; 2],
}

#[derive(Debug)]
pub struct Instruction {
	pub length: usize,
	raw: RawInstruction,
	mnemonic: Mnemonic,
	operands: Vec<Operand>,
}

impl Instruction {
	pub fn decode(bytes: &[u8]) -> Instruction {
		decode::decode(bytes)
	}

	pub fn bytes(&self) -> Vec<u8> {
		let mut bytes = vec![self.raw.opcode];
		bytes.extend_from_slice(&self.raw.data[..self.length-1]);
		
		bytes
	}

}

impl fmt::Display for Instruction {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.operands.len() {
			0 => write!(f, "{:?}", self.mnemonic),
			1 => write!(f, "{:?} {}", self.mnemonic, self.operands[0]),
			2 => write!(f, "{:?} {}, {}", self.mnemonic, self.operands[0], self.operands[1]),
			_ => panic!(format!("Could not decode '{:?}'", self.mnemonic)),
		}
	}

}
