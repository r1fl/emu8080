use std::process;
use std::fs;

use clap::clap_app;

use emu8080::taito8080::rom;
use emu8080::taito8080::cpu;

pub fn main() {

	let args = clap_app!(emu8080 =>
		(version: "1.0")
		(@arg PATH: +required "ROM path.")
	).get_matches();

	let path = args.value_of("PATH").unwrap();
	println!("emulating {:?}", path);

	let rom: Vec<u8> = fs::read(path).unwrap_or_else(|error_msg| {
		eprintln!("Reading ROM: {}.", error_msg);
		process::exit(1);
	});

	let rom = rom::load(rom);

	/*
	 * Test
	 */

	//let mem: Vec<[u8]> = Vec::with_capacity(100);
	let mem: Box<[u8]> = Box::new([1; 100]);
	let mut cpu = cpu::state::State::init(mem);

	let bytes = [0x11u8, 0x13, 0x37];
	let instruction = cpu::instruction::Instruction::decode(&bytes);

	println!("{}", instruction);
	cpu.execute(instruction);

	println!("{:#x?}", cpu);
	//println!("{:?}", cpu.memory[0..10]);

	//emulate(rom);
}

fn emulate(rom: rom::Rom) -> Result<(), ()> {
	let mut cpu = cpu::state::State::init(Box::new([1,2]));
	println!("{:#?}", cpu);

	for instruction in rom.instructions() {
		cpu.execute(instruction);
	}

	Ok(())

}
