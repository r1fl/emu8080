use super::instruction::{self, Instruction, Register};
use super::state::{self, State, ConditionFlags};

pub fn execute(state: &mut State, instruction: Instruction) {
	use instruction::Operand::*;
	use instruction::Operands::*;
	use instruction::Mnemonic::*;

	match instruction.raw.opcode {
		0x1 => add(&mut state.c, &mut state.l, &mut state.flags),
		0x00 => nop(), // NOP
		//0x01 =>	lxi(&mut state.bc, instruction.raw.data[..])	// LXI B,d16
		0x01 =>	unimplemented!(),
		0x02 => unimplemented!(), // STAX B
		0x03 => unimplemented!(), // INX B
		0x04 => unimplemented!(), // INR B
		0x05 => unimplemented!(), // DCR B
		0x06 => unimplemented!(), // MVI B,d8
		0x07 => unimplemented!(), // RLC
		0x08 => nop(), // NOP
		0x09 => unimplemented!(), // DAD B
		0x0a => unimplemented!(), // LDAX B
		0x0b => unimplemented!(), // DCX B
		0x0c => unimplemented!(), // INR C
		0x0d => unimplemented!(), // DCR C
		0x0e => unimplemented!(), // MVI C,d8
		0x0f => unimplemented!(), // RRC
		0x10 => nop(), // NOP
		0x11 => unimplemented!(), // LXI D,d16
		0x12 => unimplemented!(), // STAX D
		0x13 => unimplemented!(), // INX D
		0x14 => unimplemented!(), // INR D
		0x15 => unimplemented!(), // DCR D
		0x16 => unimplemented!(), // MVI D,d8
		0x17 => unimplemented!(), // RAL
		0x18 => nop(), // NOP
		0x19 => unimplemented!(), // DAD D
		0x1a => unimplemented!(), // LDAX D
		0x1b => unimplemented!(), // DCX D
		0x1c => unimplemented!(), // INR E
		0x1d => unimplemented!(), // DCR E
		0x1e => unimplemented!(), // MVI E,d8
		0x1f => unimplemented!(), // RAR
		0x20 => nop(), // NOP
		0x21 => unimplemented!(), // LXI H,d16
		0x22 => unimplemented!(), // SHLD a16
		0x23 => unimplemented!(), // INX H
		0x24 => unimplemented!(), // INR H
		0x25 => unimplemented!(), // DCR H
		0x26 => unimplemented!(), // MVI H,d8
		0x27 => unimplemented!(), // DAA
		0x28 => nop(), // NOP
		0x29 => unimplemented!(), // DAD H
		0x2a => unimplemented!(), // LHLD a16
		0x2b => unimplemented!(), // DCX H
		0x2c => unimplemented!(), // INR L
		0x2d => unimplemented!(), // DCR L
		0x2e => unimplemented!(), // MVI L,d8
		0x2f => unimplemented!(), // CMA
		0x30 => nop(), // NOP
		0x31 => unimplemented!(), // LXI SP,d16
		0x32 => unimplemented!(), // STA a16
		0x33 => unimplemented!(), // INX SP
		0x34 => unimplemented!(), // INR M
		0x35 => unimplemented!(), // DCR M
		0x36 => unimplemented!(), // MVI M,d8
		0x37 => unimplemented!(), // STC
		0x38 => nop(), // NOP
		0x39 => unimplemented!(), // DAD SP
		0x3a => unimplemented!(), // LDA a16
		0x3b => unimplemented!(), // DCX SP
		0x3c => unimplemented!(), // INR A
		0x3d => unimplemented!(), // DCR A
		0x3e => unimplemented!(), // MVI A,d8
		0x3f => unimplemented!(), // CMC
		0x40 => unimplemented!(), // MOV B,B
		0x41 => unimplemented!(), // MOV B,C
		0x42 => unimplemented!(), // MOV B,D
		0x43 => unimplemented!(), // MOV B,E
		0x44 => unimplemented!(), // MOV B,H
		0x45 => unimplemented!(), // MOV B,L
		0x46 => unimplemented!(), // MOV B,M
		0x47 => unimplemented!(), // MOV B,A
		0x48 => unimplemented!(), // MOV C,B
		0x49 => unimplemented!(), // MOV C,C
		0x4a => unimplemented!(), // MOV C,D
		0x4b => unimplemented!(), // MOV C,E
		0x4c => unimplemented!(), // MOV C,H
		0x4d => unimplemented!(), // MOV C,L
		0x4e => unimplemented!(), // MOV C,M
		0x4f => unimplemented!(), // MOV C,A
		0x50 => unimplemented!(), // MOV D,B
		0x51 => unimplemented!(), // MOV D,C
		0x52 => unimplemented!(), // MOV D,D
		0x53 => unimplemented!(), // MOV D,E
		0x54 => unimplemented!(), // MOV D,H
		0x55 => unimplemented!(), // MOV D,L
		0x56 => unimplemented!(), // MOV D,M
		0x57 => unimplemented!(), // MOV D,A
		0x58 => unimplemented!(), // MOV E,B
		0x59 => unimplemented!(), // MOV E,C
		0x5a => unimplemented!(), // MOV E,D
		0x5b => unimplemented!(), // MOV E,E
		0x5c => unimplemented!(), // MOV E,H
		0x5d => unimplemented!(), // MOV E,L
		0x5e => unimplemented!(), // MOV E,M
		0x5f => unimplemented!(), // MOV E,A
		0x60 => unimplemented!(), // MOV H,B
		0x61 => unimplemented!(), // MOV H,C
		0x62 => unimplemented!(), // MOV H,D
		0x63 => unimplemented!(), // MOV H,E
		0x64 => unimplemented!(), // MOV H,H
		0x65 => unimplemented!(), // MOV H,L
		0x66 => unimplemented!(), // MOV H,M
		0x67 => unimplemented!(), // MOV H,A
		0x68 => unimplemented!(), // MOV L,B
		0x69 => unimplemented!(), // MOV L,C
		0x6a => unimplemented!(), // MOV L,D
		0x6b => unimplemented!(), // MOV L,E
		0x6c => unimplemented!(), // MOV L,H
		0x6d => unimplemented!(), // MOV L,L
		0x6e => unimplemented!(), // MOV L,M
		0x6f => unimplemented!(), // MOV L,A
		0x70 => unimplemented!(), // MOV M,B
		0x71 => unimplemented!(), // MOV M,C
		0x72 => unimplemented!(), // MOV M,D
		0x73 => unimplemented!(), // MOV M,E
		0x74 => unimplemented!(), // MOV M,H
		0x75 => unimplemented!(), // MOV M,L
		0x76 => unimplemented!(), // HLT
		0x77 => unimplemented!(), // MOV M,A
		0x78 => unimplemented!(), // MOV A,B
		0x79 => unimplemented!(), // MOV A,C
		0x7a => unimplemented!(), // MOV A,D
		0x7b => unimplemented!(), // MOV A,E
		0x7c => unimplemented!(), // MOV A,H
		0x7d => unimplemented!(), // MOV A,L
		0x7e => unimplemented!(), // MOV A,M
		0x7f => unimplemented!(), // MOV A,A
		0x80 => unimplemented!(), // ADD B
		0x81 => unimplemented!(), // ADD C
		0x82 => unimplemented!(), // ADD D
		0x83 => unimplemented!(), // ADD E
		0x84 => unimplemented!(), // ADD H
		0x85 => unimplemented!(), // ADD L
		0x86 => unimplemented!(), // ADD M
		0x87 => unimplemented!(), // ADD A
		0x88 => unimplemented!(), // ADC B
		0x89 => unimplemented!(), // ADC C
		0x8a => unimplemented!(), // ADC D
		0x8b => unimplemented!(), // ADC E
		0x8c => unimplemented!(), // ADC H
		0x8d => unimplemented!(), // ADC L
		0x8e => unimplemented!(), // ADC M
		0x8f => unimplemented!(), // ADC A
		0x90 => unimplemented!(), // SUB B
		0x91 => unimplemented!(), // SUB C
		0x92 => unimplemented!(), // SUB D
		0x93 => unimplemented!(), // SUB E
		0x94 => unimplemented!(), // SUB H
		0x95 => unimplemented!(), // SUB L
		0x96 => unimplemented!(), // SUB M
		0x97 => unimplemented!(), // SUB A
		0x98 => unimplemented!(), // SBB B
		0x99 => unimplemented!(), // SBB C
		0x9a => unimplemented!(), // SBB D
		0x9b => unimplemented!(), // SBB E
		0x9c => unimplemented!(), // SBB H
		0x9d => unimplemented!(), // SBB L
		0x9e => unimplemented!(), // SBB M
		0x9f => unimplemented!(), // SBB A
		0xa0 => unimplemented!(), // ANA B
		0xa1 => unimplemented!(), // ANA C
		0xa2 => unimplemented!(), // ANA D
		0xa3 => unimplemented!(), // ANA E
		0xa4 => unimplemented!(), // ANA H
		0xa5 => unimplemented!(), // ANA L
		0xa6 => unimplemented!(), // ANA M
		0xa7 => unimplemented!(), // ANA A
		0xa8 => unimplemented!(), // XRA B
		0xa9 => unimplemented!(), // XRA C
		0xaa => unimplemented!(), // XRA D
		0xab => unimplemented!(), // XRA E
		0xac => unimplemented!(), // XRA H
		0xad => unimplemented!(), // XRA L
		0xae => unimplemented!(), // XRA M
		0xaf => unimplemented!(), // XRA A
		0xb0 => unimplemented!(), // ORA B
		0xb1 => unimplemented!(), // ORA C
		0xb2 => unimplemented!(), // ORA D
		0xb3 => unimplemented!(), // ORA E
		0xb4 => unimplemented!(), // ORA H
		0xb5 => unimplemented!(), // ORA L
		0xb6 => unimplemented!(), // ORA M
		0xb7 => unimplemented!(), // ORA A
		0xb8 => unimplemented!(), // CMP B
		0xb9 => unimplemented!(), // CMP C
		0xba => unimplemented!(), // CMP D
		0xbb => unimplemented!(), // CMP E
		0xbc => unimplemented!(), // CMP H
		0xbd => unimplemented!(), // CMP L
		0xbe => unimplemented!(), // CMP M
		0xbf => unimplemented!(), // CMP A
		0xc0 => unimplemented!(), // RNZ
		0xc1 => unimplemented!(), // POP B
		0xc2 => unimplemented!(), // JNZ a16
		0xc3 => unimplemented!(), // JMP a16
		0xc4 => unimplemented!(), // CNZ a16
		0xc5 => unimplemented!(), // PUSH B
		0xc6 => unimplemented!(), // ADI d8
		0xc7 => unimplemented!(), // RST 0
		0xc8 => unimplemented!(), // RZ
		0xc9 => unimplemented!(), // RET
		0xca => unimplemented!(), // JZ a16
		0xcb => unimplemented!(), // JMP a16
		0xcc => unimplemented!(), // CZ a16
		0xcd => unimplemented!(), // CALL a16
		0xce => unimplemented!(), // ACI d8
		0xcf => unimplemented!(), // RST 1
		0xd0 => unimplemented!(), // RNC
		0xd1 => unimplemented!(), // POP D
		0xd2 => unimplemented!(), // JNC a16
		0xd3 => unimplemented!(), // OUT d8
		0xd4 => unimplemented!(), // CNC a16
		0xd5 => unimplemented!(), // PUSH D
		0xd6 => unimplemented!(), // SUI d8
		0xd7 => unimplemented!(), // RST 2
		0xd8 => unimplemented!(), // RC
		0xd9 => unimplemented!(), // RET
		0xda => unimplemented!(), // JC a16
		0xdb => unimplemented!(), // IN d8
		0xdc => unimplemented!(), // CC a16
		0xdd => unimplemented!(), // CALL a16
		0xde => unimplemented!(), // SBI d8
		0xdf => unimplemented!(), // RST 3
		0xe0 => unimplemented!(), // RPO
		0xe1 => unimplemented!(), // POP H
		0xe2 => unimplemented!(), // JPO a16
		0xe3 => unimplemented!(), // XTHL
		0xe4 => unimplemented!(), // CPO a16
		0xe5 => unimplemented!(), // PUSH H
		0xe6 => unimplemented!(), // ANI d8
		0xe7 => unimplemented!(), // RST 4
		0xe8 => unimplemented!(), // RPE
		0xe9 => unimplemented!(), // PCHL
		0xea => unimplemented!(), // JPE a16
		0xeb => unimplemented!(), // XCHG
		0xec => unimplemented!(), // CPE a16
		0xed => unimplemented!(), // CALL a16
		0xee => unimplemented!(), // XRI d8
		0xef => unimplemented!(), // RST 5
		0xf0 => unimplemented!(), // RP
		0xf1 => unimplemented!(), // POP PSW
		0xf2 => unimplemented!(), // JP a16
		0xf3 => unimplemented!(), // DI
		0xf4 => unimplemented!(), // CP a16
		0xf5 => unimplemented!(), // PUSH PSW
		0xf6 => unimplemented!(), // ORI d8
		0xf7 => unimplemented!(), // RST 6
		0xf8 => unimplemented!(), // RM
		0xf9 => unimplemented!(), // SPHL
		0xfa => unimplemented!(), // JM a16
		0xfb => unimplemented!(), // EI
		0xfc => unimplemented!(), // CM a16
		0xfd => unimplemented!(), // CALL a16
		0xfe => unimplemented!(), // CPI d8
		0xff => unimplemented!(), // RST 7
		
		//Instruction { mnemonic: MOV, operands: Two(Reg(dst), Reg(src)), .. } => mov(state, dst, )
		//0x40 => instruction!(instruction; MOV B,B),
		//match_operands!(LXI B,d16 => (src, dst))

		//Instruction { mnemonic: LXI, operands: Two(Reg(dst), D16(src)), .. } => lxi(state, dst, src), 
		//instruction!(_; LXI B,d16; Two(Reg(dst), D16(src))) => execute::unimplemented(self)
		//instruction!(_; LXI B,d16; lxi) => execute::unimplemented(self)
	};
}

/*
 * Control Group
 */

pub fn nop() {
	println!("executing nop");
}

/*
 * Data Transfer Group
 */


pub fn mvi() {

}

// Move from register to register
//#[inline(always)]
//pub fn mov<T>(dst: &mut T, src: &mut T) {
//	*dst = *src;
//}

pub fn lxi() {

}

pub fn stax() {

}

/*
 * Arthimetic Group
 */

pub fn add(accumulator: &mut u8, value: &u8, flags: &mut ConditionFlags) {
	*accumulator += *value;
}

/*
 * Branch Group
 */

#[inline(always)]
pub fn jmp(pc: &mut u16, address: u16) {
	*pc = address;
}

#[inline(always)]
pub fn call(pc: &mut u16, address: u16) {
	jmp(pc, address);
}


/*
macro_rules! unpack16 {
	($bytes:expr) => (
		($bytes[0] as u16) << 0 |
		($bytes[1] as u16) << 8
	);
}


//	mov => |instruction, state| mov(state, state.b, state.a)
//	jmp => |instruction, state| jmp(state, instruction.operands[0])

fn get_register(state: &mut State, register: Register) {

}

pub fn mov(state: &mut State, dst: Register, src: Register) { 
	match (dst, src) {
		(_ @ reg, M) => do_mov(get_register(state, reg), M!(state)),
		(M, _ @ reg) => do_mov(M!(state), get_register(state, reg)),
		_			 => do_mov(get_register(state, dst), get_register(state, src)),
	}
}

/// Move from memory to register
pub fn mov_mr(state: &mut State, register: &mut u8) {
	let mem_addr = unpack16!([state.h, state.l]);
	*register = state.mem[mem_addr];
}

/// Move from register to memory
pub fn mov_rm(state: &mut State, register: &mut u8) {
	let mem_addr = unpack16!([state.h, state.l]);
	*state.mem[mem_addr] = *register;
}
*/
