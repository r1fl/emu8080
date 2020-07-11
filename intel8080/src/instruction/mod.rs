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
	reg(Register),
	d8(u8),
	d16(u16),
	a16(u16),
}

impl fmt::Display for Operand {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Operand::reg(val) => write!(f, "{:?}", val),
			Operand::d8(val) => write!(f, "{:#x}", val),
			Operand::d16(val) => write!(f, "{:#x}", val),
			Operand::a16(val) => write!(f, "{:#x}", val),
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
		if self.operands.len() == 0 { 
			return write!(f, "{:?}", self.mnemonic) 
		}

		/*
		for operand in self.operands.iter() {
			formatted_operands += format!("{}", operand);
		}
		*/ // FIXME
		write!(f, "{:?} {}", self.mnemonic, self.operands[0])
	}

}
