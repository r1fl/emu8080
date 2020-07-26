use super::{State, ConditionFlags};

pub fn execute(state: &mut State) {
	let pc = usize::from(state.registers.pc);

	let instruction = &state.memory[pc..pc+3];
	let instruction = instruction.to_vec();

	let opcode: u8 = instruction[0];

	let length = match opcode {
		/*
		 * Data Transfer Group
		 */

		0x06 => mvi(&mut state.registers.b, instruction[0]),		// MVI B,d8
		0x0e => mvi(&mut state.registers.c, instruction[0]),		// MVI C,d8
		0x16 => mvi(&mut state.registers.d, instruction[0]),		// MVI D,d8
		0x1e => mvi(&mut state.registers.e, instruction[0]),		// MVI E,d8
		0x26 => mvi(&mut state.registers.h, instruction[0]),		// MVI H,d8
		0x2e => mvi(&mut state.registers.l, instruction[0]),		// MVI L,d8
		0x3e => mvi(&mut state.registers.a, instruction[0]),		// MVI A,d8
		0x36 => {
			let address = *state.registers.pair(&state.registers.h) as usize;
			mvi(&mut state.memory[address], instruction[0])
		},															// MVI M,d8

		0x40 => {
			let b = state.registers.b;
			mov(&mut state.registers.b, &b)
		},															// MOV B,B
		0x41 => mov(&mut state.registers.b, &state.registers.c),	// MOV B,C
		0x42 => mov(&mut state.registers.b, &state.registers.d),	// MOV B,D
		0x43 => mov(&mut state.registers.b, &state.registers.e),	// MOV B,E
		0x44 => mov(&mut state.registers.b, &state.registers.h),	// MOV B,H
		0x45 => mov(&mut state.registers.b, &state.registers.l),	// MOV B,L
		0x46 => {
			let address = *state.registers.pair(&state.registers.h) as usize;
			mov(&mut state.registers.b, &state.memory[address])
		},															// MOV B,M
		0x47 => mov(&mut state.registers.b, &state.registers.a),	// MOV B,A

		0x48 => mov(&mut state.registers.c, &state.registers.b),	// MOV C,B
		0x49 => {
			let c = state.registers.c;
			mov(&mut state.registers.c, &c)
		},															// MOV C,C
		0x4a => mov(&mut state.registers.c, &state.registers.d),	// MOV C,D
		0x4b => mov(&mut state.registers.c, &state.registers.e),	// MOV C,E
		0x4c => mov(&mut state.registers.c, &state.registers.h),	// MOV C,H
		0x4d => mov(&mut state.registers.c, &state.registers.l),	// MOV C,L
		0x4e => {
			let address = *state.registers.pair(&state.registers.h) as usize;
			mov(&mut state.registers.c, &state.memory[address])
		},															// MOV C,M
		0x4f => mov(&mut state.registers.c, &state.registers.a),	// MOV C,A

		0x50 => mov(&mut state.registers.d, &state.registers.b),	// MOV D,B
		0x51 => mov(&mut state.registers.d, &state.registers.c),	// MOV D,C
		0x52 => {
			let d = state.registers.d;
			mov(&mut state.registers.d, &d)
		},															// MOV D,D
		0x53 => mov(&mut state.registers.d, &state.registers.e),	// MOV D,E
		0x54 => mov(&mut state.registers.d, &state.registers.h),	// MOV D,H
		0x55 => mov(&mut state.registers.d, &state.registers.l),	// MOV D,L
		0x56 => {
			let address = *state.registers.pair(&state.registers.h) as usize;
			mov(&mut state.registers.d, &state.memory[address])
		},															// MOV D,M
		0x57 => mov(&mut state.registers.d, &state.registers.a),	// MOV D,A

		0x58 => mov(&mut state.registers.e, &state.registers.b),	// MOV E,B 
		0x59 => mov(&mut state.registers.e, &state.registers.c),	// MOV E,C 
		0x5a => mov(&mut state.registers.e, &state.registers.d),	// MOV E,D
		0x5b => {
			let e = state.registers.e;
			mov(&mut state.registers.e, &e)
		},															// MOV E,E
		0x5c => mov(&mut state.registers.e, &state.registers.h),	// MOV E,H
		0x5d => mov(&mut state.registers.e, &state.registers.l),	// MOV E,L
		0x5e => {
			let address = *state.registers.pair(&state.registers.h) as usize;
			mov(&mut state.registers.e, &state.memory[address])
		},															// MOV E,M
		0x5f => mov(&mut state.registers.e, &state.registers.a),	// MOV E,A

		0x60 => mov(&mut state.registers.h, &state.registers.b),	// MOV H,B
		0x61 => mov(&mut state.registers.h, &state.registers.c),	// MOV H,C
		0x62 => mov(&mut state.registers.h, &state.registers.d),	// MOV H,D
		0x63 => mov(&mut state.registers.h, &state.registers.e),	// MOV H,E
		0x64 => {
			let h = state.registers.h;
			mov(&mut state.registers.h, &h)
		},															// MOV H,H
		0x65 => mov(&mut state.registers.h, &state.registers.l),	// MOV H,L
		0x66 => {
			let address = *state.registers.pair(&state.registers.h) as usize;
			mov(&mut state.registers.h, &state.memory[address])
		},															// MOV H,M
		0x67 => mov(&mut state.registers.h, &state.registers.a),	// MOV H,A

		0x68 => mov(&mut state.registers.l, &state.registers.b),	// MOV L,B 
		0x69 => mov(&mut state.registers.l, &state.registers.c),	// MOV L,C 
		0x6a => mov(&mut state.registers.l, &state.registers.d),	// MOV L,D
		0x6b => mov(&mut state.registers.l, &state.registers.e),	// MOV L,E
		0x6c => mov(&mut state.registers.l, &state.registers.h),	// MOV L,H
		0x6d => {
			let l = state.registers.l;
			mov(&mut state.registers.l, &l)
		},															// MOV L,L
		0x6e => {
			let address = *state.registers.pair(&state.registers.h) as usize;
			mov(&mut state.registers.l, &state.memory[address])
		},															// MOV L,M
		0x6f => mov(&mut state.registers.l, &state.registers.a),	// MOV L,A

		0x70 => {
			let address = *state.registers.pair(&state.registers.h) as usize;
			mov(&mut state.memory[address], &mut state.registers.b)
		},															// MOV M,B
		0x71 => {
			let address = *state.registers.pair(&state.registers.h) as usize;
			mov(&mut state.memory[address], &mut state.registers.c)
		},															// MOV M,C
		0x72 => {
			let address = *state.registers.pair(&state.registers.h) as usize;
			mov(&mut state.memory[address], &mut state.registers.d)
		},															// MOV M,D
		0x73 => {
			let address = *state.registers.pair(&state.registers.h) as usize;
			mov(&mut state.memory[address], &mut state.registers.e)
		},															// MOV M,E
		0x74 => {
			let address = *state.registers.pair(&state.registers.h) as usize;
			mov(&mut state.memory[address], &mut state.registers.h)
		},															// MOV M,H
		0x75 => {
			let address = *state.registers.pair(&state.registers.h) as usize;
			mov(&mut state.memory[address], &mut state.registers.l)
		},															// MOV M,L
		0x77 => {
			let address = *state.registers.pair(&state.registers.h) as usize;
			mov(&mut state.memory[address], &mut state.registers.a)
		},															// MOV M,A

		0x78 => mov(&mut state.registers.a, &state.registers.b),	// MOV A,B 
		0x79 => mov(&mut state.registers.a, &state.registers.c),	// MOV A,C 
		0x7a => mov(&mut state.registers.a, &state.registers.d),	// MOV A,D
		0x7b => mov(&mut state.registers.a, &state.registers.e),	// MOV A,E
		0x7c => mov(&mut state.registers.a, &state.registers.h),	// MOV A,H
		0x7d => mov(&mut state.registers.a, &state.registers.l),	// MOV A,L
		0x7e => {
			let address = *state.registers.pair(&state.registers.h) as usize;
			mov(&mut state.registers.a, &state.memory[address])
		},															// MOV A,M
		0x7f => {
			let a = state.registers.a;
			mov(&mut state.registers.a, &a)
		},															// MOV A,A

		//0x01 =>	{
		//	let mut value = [0u8; 2];
		//	value.copy_from_slice(&instruction[1..]);

		//	lxi(state.registers.mut_pair(&mut state.registers.b), 
		//		u16::from_le_bytes(value))
		//},														// LXI B,d16
		//0x11 =>	{
		//	let mut value = [0u8; 2];
		//	value.copy_from_slice(&instruction[1..]);

		//	lxi(state.registers.mut_pair(&mut state.registers.d), 
		//		u16::from_le_bytes(value))
		//},														// LXI D,d16
		//0x21 =>	{
		//	let mut value = [0u8; 2];
		//	value.copy_from_slice(&instruction[1..]);

		//	//let hl = unsafe { mem::transmute::<&mut u8, &mut u16>(&mut state.registers.h) };

		//	lxi(state.registers.mut_pair(&mut state.registers.h), 
		//		u16::from_le_bytes(value))
		//},														// LXI H,d16
		0x31 =>	{
			let mut value = [0u8; 2];
			value.copy_from_slice(&instruction[1..]);

			lxi(&mut state.registers.sp, u16::from_le_bytes(value))
		},														// LXI SP,d16;

		0x0a => unimplemented!(), 								// LDAX B
		0x1a => unimplemented!(), 								// LDAX D

		0x3a => unimplemented!(), 								// LDA a16

		0x02 => unimplemented!(),								// STAX B
		0x12 => unimplemented!(), 								// STAX D

		0x32 => unimplemented!(), 								// STA a16

		0x22 => unimplemented!(), 								// SHLD a16

		0x2a => unimplemented!(), 								// LHLD a16

		0xeb => unimplemented!(), 								// XCHG

		/*
		 * Arthimetic Group
		 */

		//0x80 => add(&mut state.a, state.b, &mut state.flags),	// ADD B
		//0x81 => add(&mut state.a, state.c, &mut state.flags), 	// ADD C
		//0x82 => add(&mut state.a, state.d, &mut state.flags), 	// ADD D
		//0x83 => add(&mut state.a, state.e, &mut state.flags), 	// ADD E
		//0x84 => add(&mut state.a, state.h, &mut state.flags), 	// ADD H
		//0x85 => add(&mut state.a, state.l, &mut state.flags), 	// ADD L
		//0x86 => {
		//	let value = state.memory[*pair(&state.h) as usize];
		//	add(&mut state.a, value, &mut state.flags)
		//},														// ADD M
		//0x87 => { 
		//	let value = state.a; 
		//	add(&mut state.a, value, &mut state.flags) 
		//},														// ADD A

		0x88 => unimplemented!("adc"),								// ADC B
		0x89 => unimplemented!("adc"), 								// ADC C
		0x8a => unimplemented!("adc"), 								// ADC D
		0x8b => unimplemented!("adc"), 								// ADC E
		0x8c => unimplemented!("adc"), 								// ADC H
		0x8d => unimplemented!("adc"), 								// ADC L
		0x8e => unimplemented!("adc"), 								// ADC M
		0x8f => unimplemented!("adc"), 								// ADC A

		0x90 => unimplemented!("sub"), 								// SUB B
		0x91 => unimplemented!("sub"), 								// SUB C
		0x92 => unimplemented!("sub"), 								// SUB D
		0x93 => unimplemented!("sub"), 								// SUB E
		0x94 => unimplemented!("sub"), 								// SUB H
		0x95 => unimplemented!("sub"), 								// SUB L
		0x96 => unimplemented!("sub"), 								// SUB M
		0x97 => unimplemented!("sub"), 								// SUB A

		0x04 => unimplemented!("inr"), 								// INR B
		0x0c => unimplemented!("inr"), 								// INR C
		0x14 => unimplemented!("inr"), 								// INR D
		0x1c => unimplemented!("inr"), 								// INR E
		0x24 => unimplemented!("inr"), 								// INR H XXX
		0x2c => unimplemented!("inr"), 								// INR L
		0x34 => unimplemented!("inr"), 								// INR M
		0x3c => unimplemented!("inr"), 								// INR A

		0x03 => unimplemented!("inx"), 								// INX B
		0x13 => unimplemented!("inx"), 								// INX D
		0x23 => unimplemented!("inx"), 								// INX H
		0x33 => unimplemented!("inx"), 								// INX SP

		0xb0 => unimplemented!("ora"), 								// ORA B
		0xb1 => unimplemented!("ora"), 								// ORA C
		0xb2 => unimplemented!("ora"), 								// ORA D
		0xb3 => unimplemented!("ora"), 								// ORA E
		0xb4 => unimplemented!("ora"), 								// ORA H
		0xb5 => unimplemented!("ora"), 								// ORA L
		0xb6 => unimplemented!("ora"), 								// ORA M
		0xb7 => unimplemented!("ora"), 								// ORA A

		0xb8 => unimplemented!("cmp"), 								// CMP B
		0xb9 => unimplemented!("cmp"), 								// CMP C
		0xba => unimplemented!("cmp"), 								// CMP D
		0xbb => unimplemented!("cmp"), 								// CMP E
		0xbc => unimplemented!("cmp"), 								// CMP H
		0xbd => unimplemented!("cmp"), 								// CMP L
		0xbe => unimplemented!("cmp"), 								// CMP M
		0xbf => unimplemented!("cmp"), 								// CMP A


		0x98 => unimplemented!("sbb"), 								// SBB B
		0x99 => unimplemented!("sbb"), 								// SBB C
		0x9a => unimplemented!("sbb"), 								// SBB D
		0x9b => unimplemented!("sbb"), 								// SBB E
		0x9c => unimplemented!("sbb"), 								// SBB H
		0x9d => unimplemented!("sbb"), 								// SBB L
		0x9e => unimplemented!("sbb"), 								// SBB M
		0x9f => unimplemented!("sbb"), 								// SBB A

		0xa0 => unimplemented!("ana"), 								// ANA B
		0xa1 => unimplemented!("ana"), 								// ANA C
		0xa2 => unimplemented!("ana"), 								// ANA D
		0xa3 => unimplemented!("ana"), 								// ANA E
		0xa4 => unimplemented!("ana"), 								// ANA H
		0xa5 => unimplemented!("ana"), 								// ANA L
		0xa6 => unimplemented!("ana"), 								// ANA M
		0xa7 => unimplemented!("ana"), 								// ANA A

		0xa8 => unimplemented!("xra"), 								// XRA B
		0xa9 => unimplemented!("xra"), 								// XRA C
		0xaa => unimplemented!("xra"), 								// XRA D
		0xab => unimplemented!("xra"), 								// XRA E
		0xac => unimplemented!("xra"), 								// XRA H
		0xad => unimplemented!("xra"), 								// XRA L
		0xae => unimplemented!("xra"), 								// XRA M
		0xaf => unimplemented!("xra"), 								// XRA A

		/*
		 * Control Group
		 */

		0x00 => nop(),											// NOP
		0x08 => nop(),											// NOP
		0x10 => nop(),			  								// NOP
		0x18 => nop(),			  								// NOP
		0x20 => nop(),			  								// NOP
		0x28 => nop(),			  								// NOP
		0x30 => nop(),			  								// NOP
		0x38 => nop(),											// NOP

		/*
		 * Branch Group
		 */

		0xc3 => {
			let mut addr = [0u8; 2];
			addr.copy_from_slice(&instruction[1..]);

			jmp(&mut state.registers.pc, u16::from_le_bytes(addr))
		},														// JMP a16
		0xcb => {
			let mut addr = [0u8; 2];
			addr.copy_from_slice(&instruction[1..]);

			jmp(&mut state.registers.pc, u16::from_le_bytes(addr))
		},														// JMP a16
		0xca => unimplemented!(), 								// JZ a16
		0xc2 => unimplemented!(), 								// JNZ a16
		0xd2 => unimplemented!(), 								// JNC a16
		0xda => unimplemented!(), 								// JC a16
		0xe2 => unimplemented!(), 								// JPO a16
		0xea => unimplemented!(), 								// JPE a16
		0xf2 => unimplemented!(), 								// JP a16
		0xfa => unimplemented!(), 								// JM a16

		/*
		 * Unsorted / Unimplemented
		 */

		0x05 => unimplemented!(), 								// DCR B
		0x07 => unimplemented!(),								// RLC
		0x09 => unimplemented!(), 								// DAD B
		0x0b => unimplemented!(), 								// DCX B
		0x0d => unimplemented!(), 								// DCR C
		0x0f => unimplemented!(), 								// RRC
		0x15 => unimplemented!(), 								// DCR D
		0x17 => unimplemented!(), 								// RAL
		0x19 => unimplemented!(), 								// DAD D
		0x1b => unimplemented!(), 								// DCX D
		0x1d => unimplemented!(), 								// DCR E
		0x1f => unimplemented!(), 								// RAR
		0x25 => unimplemented!(), 								// DCR H
		0x27 => unimplemented!(), 								// DAA
		0x29 => unimplemented!(), 								// DAD H
		0x2b => unimplemented!(), 								// DCX H
		0x2d => unimplemented!(), 								// DCR L
		0x2f => unimplemented!(), 								// CMA
		0x35 => unimplemented!(), 								// DCR M
		0x37 => unimplemented!(), 								// STC
		0x39 => unimplemented!(),								// DAD SP
		0x3b => unimplemented!(), 								// DCX SP
		0x3d => unimplemented!(), 								// DCR A
		0x3f => unimplemented!(), 								// CMC
		0x76 => unimplemented!(), 								// HLT

		0xc0 => unimplemented!(), 								// RNZ
		0xc1 => unimplemented!(), 								// POP B
		0xc4 => unimplemented!(), 								// CNZ a16
		0xc5 => unimplemented!(), 								// PUSH B
		0xc6 => unimplemented!(), 								// ADI d8
		0xc7 => unimplemented!(), 								// RST 0
		0xc8 => unimplemented!(), 								// RZ
		0xc9 => unimplemented!(), 								// RET
		0xcc => unimplemented!(), 								// CZ a16
		0xcd => unimplemented!(), 								// CALL a16
		0xce => unimplemented!(), 								// ACI d8
		0xcf => unimplemented!(), 								// RST 1
		0xd0 => unimplemented!(), 								// RNC
		0xd1 => unimplemented!(), 								// POP D
		0xd3 => unimplemented!(), 								// OUT d8
		0xd4 => unimplemented!(), 								// CNC a16
		0xd5 => unimplemented!(), 								// PUSH D
		0xd6 => unimplemented!(), 								// SUI d8
		0xd7 => unimplemented!(), 								// RST 2
		0xd8 => unimplemented!(), 								// RC
		0xd9 => unimplemented!(), 								// RET
		0xdb => unimplemented!(), 								// IN d8
		0xdc => unimplemented!(), 								// CC a16
		0xdd => unimplemented!(), 								// CALL a16
		0xde => unimplemented!(), 								// SBI d8
		0xdf => unimplemented!(), 								// RST 3
		0xe0 => unimplemented!(), 								// RPO
		0xe1 => unimplemented!(), 								// POP H
		0xe3 => unimplemented!(), 								// XTHL
		0xe4 => unimplemented!(), 								// CPO a16
		0xe5 => unimplemented!(), 								// PUSH H
		0xe6 => unimplemented!(), 								// ANI d8
		0xe7 => unimplemented!(), 								// RST 4
		0xe8 => unimplemented!(), 								// RPE
		0xe9 => unimplemented!(), 								// PCHL
		0xec => unimplemented!(), 								// CPE a16
		0xed => unimplemented!(), 								// CALL a16
		0xee => unimplemented!(), 								// XRI d8
		0xef => unimplemented!(), 								// RST 5
		0xf0 => unimplemented!(), 								// RP
		0xf1 => unimplemented!(), 								// POP PSW
		0xf3 => unimplemented!(), 								// DI
		0xf4 => unimplemented!(), 								// CP a16
		0xf5 => unimplemented!(), 								// PUSH PSW
		0xf6 => unimplemented!(), 								// ORI d8
		0xf7 => unimplemented!(), 								// RST 6
		0xf8 => unimplemented!(), 								// RM
		0xf9 => unimplemented!(), 								// SPHL
		0xfb => unimplemented!(), 								// EI
		0xfc => unimplemented!(), 								// CM a16
		0xfd => unimplemented!(), 								// CALL a16
		0xfe => unimplemented!(), 								// CPI d8
		0xff => unimplemented!(), 								// RST 7

		_ => unimplemented!(),
	};

	state.registers.pc += length;

}

/*
 * Control Group
 */

fn nop() -> u16 { 1 }

/*
 * Data Transfer Group
 */


fn mvi(dst: &mut u8, value: u8) -> u16 {
	*dst = value;
	2
}

fn mov(dst: &mut u8, src: &u8) -> u16 {
	*dst = *src;
	2
}

fn lxi(dst: &mut u16, value: u16) -> u16 {
	*dst = value;
	2
}

/*
 * Arthimetic Group
 */

fn sub(accumulator: &mut u8, value: u8, flags: &mut u8) {
}

/*
/// Z,S,P,CY,AC FIXME
fn add(accumulator: &mut u8, value: u8, flags: &mut ConditionFlags) {
	*accumulator = *accumulator + value;
	let result = *accumulator + value;

	flags.carry = bool::from((u16::from(*accumulator) + u16::from(value)) >> 8 != 0);

	flags.zero = bool::from(result == 0);
	flags.sign = bool::from((result as i8) < 0);
	flags.parity = bool::from((result % 2) == 0);
	//flags.carry = bool::from(*);
	//flags.aux_carry = bool::from(result == 0);
	
}
*/

/*
 * Branch Group
 */

fn jmp(pc: &mut u16, address: u16) -> u16 {
	*pc = address;
	0
}

fn call(pc: &mut u16, address: u16) -> u16 {
	jmp(pc, address);
	0
}

