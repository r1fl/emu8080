use std::env;
use std::fs;
use std::process;
use std::error;

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

fn main() -> Result<(), Box<dyn error::Error>> {
	let args = env::args().collect();
	
	// XXX why unwrap_or_else works and `!` type
	// https://stackoverflow.com/questions/36250091/how-do-i-write-the-signature-of-a-function-that-displays-an-error-and-exits-the
	// TODO: logs

	let config = Config::parse(args).unwrap_or_else(|error_msg| {
		eprintln!("{}", error_msg);
		process::exit(1);
	}); 

	let contents: Vec<u8> = fs::read(&config.filename)?;
	let rom = intel8080::rom::load(contents);

	rom.instructions().for_each(|instruction| {
		println!("{:?}", instruction);
	});

	Ok(())
}
