use std::env;
use std::fs;
use std::process;
use std::error;
use std::cmp;

use ansi_term::Color;
use clap::clap_app;

mod rom;

const MIN_ADDR_WIDTH: usize = 9;
const COLUMN_GAP_WIDTH: usize = 4;

fn main() {
	let args = clap_app!(("HIDA Pro") =>
		(version: "1.0")
		(@arg PATH: +required "ROM path.")
		//(@arg verbose: -v --verbose "Be verbose.")
		//(@arg color: --color "Colored output.")
	).get_matches();

	let path = args.value_of("PATH").unwrap();

	let rom: Vec<u8> = fs::read(path).unwrap_or_else(|error_msg| {
		eprintln!("Reading ROM: {}.", error_msg);
		process::exit(1);
	});

	let rom = rom::load(rom);
	disasm(rom);
}

fn disasm(rom: rom::Rom) {
	let mut counter = 0;
	let offset_width = cmp::max(MIN_ADDR_WIDTH, hex_digit_count(rom.contents.len()) + 2);

	rom.instructions().for_each(|instruction| {
		let bytes = stringify_bytes(instruction.bytes());

		println!("{}{GAP}{}{GAP}{}", 
				Color::Green.paint(format!("{:<#01$x}", counter, offset_width)),
				Color::Yellow.paint(format!("{:<8}", bytes)),
				instruction,
				GAP=" ".repeat(COLUMN_GAP_WIDTH)); 

		counter += instruction.length;
	});
}

/// Stringify a byte vector with items seperated by a whitespace.
///
/// ```
/// assert_eq!(strigify_bytes(vec![0xbe, 0xef], String::from("be ef")))
/// ```
fn stringify_bytes(bytes: Vec<u8>) -> String {
	bytes.iter().fold(String::new(), |string, byte| { 
		format!("{} {:02x}", string, byte)
	}).trim().to_string()
}

/// Count base 16 digits of a given number.
///
/// ```
///	assert_eq!(hex_digit_count(0x1337), 4);
/// ```
fn hex_digit_count(mut number: usize) -> usize {
	let mut digits = 0;

	loop {
		number /= 0x10;
		digits += 1;

		if number == 0 { break digits; }
	}
}

