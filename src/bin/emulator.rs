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
	emulate(rom);
}

fn emulate(rom: rom::Rom) -> Result<(), ()> {
	let mut cpu = cpu::state::State::init(Box::new([1,2]));

	for instruction in rom.instructions() {
		cpu.execute(instruction);
	}

	Ok(())

}
