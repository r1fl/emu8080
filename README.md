```
src
	disassembler.rs			# disassembler binary crate
	emulate.rs				# emulator binary crate
	lib.rs					# taito8080 library

	taito8080				# taito8080 emulation
		cpu					# 8080 emulation
			mod.rs			
			decode.rs		# instruction decoding
			state.rs		# processor state
			execute.rs		# instruction emulation
			instruction.rs	# instruction definition

		mem.rs				# represent machine memory
		rom.rs				# load roms
		graphics.rs			# graphical abstraction

tests
	test_decode.rs 			# test instruction decoder
	...
```

```rust
pub mod taito8080 {
	pub mod cpu {
		mod instruction {
			pub enum Mnemonic;
			pub struct Instruction;
		}

		use self::instruction::*;

		mod state {
			struct ConditionFlags;
			struct State;

			pub fn execute(&mut self, instruction: Instruction);
		}
		pub use self::state::State;

		mod decode {
			use super::Mnemonic::*;
		}

		pub fn init(mem: Box<[u8]>) -> State;

		mod execute {
			use super::Instruction;
			fn mov(&mut state: State, src: Operand, dst: Operand);
		}

		use self::decode::*;
		use self::execute::*;
		use self::state::*;
	}

	pub mod rom {
		pub struct Rom;
		pub fn load(contents: Vec<u8>) -> Rom;
	}
}
```

