use clap::clap_app;

use std::fs;
use std::process;

use emu8080::cpu;

const EXTENSIONS: [&str; 4] = ["h", "g", "f", "e"];

pub fn main() {
	let args = clap_app!(emu8080 =>
			(version: "1.0")
			(@arg PATH: +required "ROM path")
		).get_matches();

	let path = args.value_of("PATH").unwrap();
	let mut memory = Vec::with_capacity(0x4000);

	EXTENSIONS.iter().for_each(|extension| {
		let path = format!("{}.{}", path, extension);

		let mut file = fs::read(&path).unwrap_or_else(|error| {
			eprintln!("'{}': {}", path, error);
			process::exit(-1);
		});

		memory.append(&mut file);
		println!("'{}' loaded at {:#x}", path, memory.len());
	});

	let mut cpu = cpu::State::init(memory);

	loop {
		println!("{:x?}", cpu);
		cpu.step();
	}
}
