use std::fmt;
use super::decode;

#[derive(Debug)]
pub enum Mnemonic {
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
pub enum Register {
	A, B, C, D, E, H, L,
	M, PSW, SP, // Psuedo-Registers
}

#[derive(Debug)]
pub enum Operand {
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
	pub opcode: u8,
	pub data: [u8; 2],
}

#[derive(Debug)]
pub enum Operands {
	Nothing,
	One(Operand),
	Two(Operand, Operand),
}

#[derive(Debug)]
pub struct Instruction {
	pub length: usize,
	pub raw: RawInstruction,
	pub mnemonic: Mnemonic,
	pub operands: Operands,
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
		use Operands::*;

		match &self.operands {
			Nothing				=> write!(f, "{:?}", self.mnemonic),
			One(operand)		=> write!(f, "{:?} {}", self.mnemonic, operand),
			Two(src, dst)		=> write!(f, "{:?} {}, {}", self.mnemonic, dst, src),
			//Two(operands @ ..)	=> write!(f, "{:?} {}, {}", self.mnemonic, operands.0, operands.1),
			_					=> panic!(format!("Could not decode '{:?}'", self.mnemonic)),
		}
	}

}
