mod decode;

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
	F, A, B, C, D, E, H, L,
	M, PSW, SP, // Psuedo-Registers
}

#[derive(Debug)]
enum Operand {
	reg(Register),
	d8(u8),
	d16(u16),
	a16(u16),
}

impl Operand {
	/// TODO: Format an operand to a human readable form.
	#[allow(dead_code)]
	fn format(&self) -> String { 
		String::new()
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
	operands: Vec<Operand>, // TODO: box
}


impl Instruction {
	pub fn decode(bytes: &[u8]) -> Instruction {
		decode::decode(bytes)
	}

	/// TODO: Format an instruction to a human readable form.
	#[allow(dead_code)]
	fn format(instruction: &Instruction) -> String {
		String::new()
	}

}
