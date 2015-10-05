use std::fs::File;
use std::io::*;

pub fn log_data(filename:&'static str,string:&'static [u8]) {
	let new_line = &b"\n"[..];
	let mut f = File::create(filename).unwrap();
	f.write_all(string);
	f.write_all(new_line);
}

/*fn main() {
	log_data("logs.roger",b"Server Status goes here: ");
}*/