use std::env;
use std::fs;
use std::process;
use std::error;
use std::cmp;

use ansi_term::Color;

#[derive(Debug)]
struct Config {
	filename: String,
}

impl Config {
	fn parse(args: Vec<String>) -> Result<Config, String> {
		if args.len() < 2 {
			return Err(format!("usage: {} FILENAME", args[0]));
		}
		
		Ok(Config {
			filename: args[1].clone(),
		})
	}
}

const MIN_ADDR_WIDTH: usize = 9;

fn main() -> Result<(), Box<dyn error::Error>> {
	let args = env::args().collect();

	/*
	 * Read file
	 */

	let config = Config::parse(args).unwrap_or_else(|error_msg| {
		eprintln!("{}", error_msg);
		process::exit(1);
	}); 

	let contents: Vec<u8> = fs::read(&config.filename).unwrap_or_else(|error_msg| {
		eprintln!("{}", error_msg);
		process::exit(1);
	});

	let rom = intel8080::rom::load(contents);

	/*
	 * Print disassembly
	 */

	let mut counter = 0;
	let offset_width = cmp::max(MIN_ADDR_WIDTH, hex_digit_count(rom.contents.len()) + 2);

	rom.instructions().for_each(|instruction| {
		let bytes: String = instruction.bytes().iter()
										.fold(String::new(), |string, byte| { 
											format!("{} {:02x}", string, byte)
										}).trim().to_string();

		println!("{}  {}  {}", 
				Color::Green.paint(format!("{:<#01$x}", counter, offset_width)),
				Color::Yellow.paint(format!("{:<8}", bytes)),
				instruction); 

		counter += instruction.length;
	});

	Ok(())
}

fn hex_digit_count(mut number: usize) -> usize {
	let mut digits = 0;

	loop {
		number /= 0x10;
		digits += 1;

		if number == 0 { break digits; }
	}
}

