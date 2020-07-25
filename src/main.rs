use clap::clap_app;

use std::fs;
use std::process;

use emu8080::cpu;

pub fn main() {
	let args = clap_app!(emu8080 =>
			(version: "1.0")
			(@arg PATH: +required "ROM path")
		).get_matches();

	let path = args.value_of("PATH").unwrap();

	let rom = fs::read(path).unwrap_or_else(|error| {
		eprintln!("{}", error);
		process::exit(-1);
	});

	//let mut memory = Vec::with_capacity(4096);
	#[allow(unused_mut)]
	let mut memory = rom;
	let mut cpu = cpu::State::init(memory);

	loop {
		//println!("{:?}", cpu);
		cpu.step();
	}
}
