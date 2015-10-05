use std::fs::File;
use std::io::*;
use chrono::*;

pub fn log_and_write(filename:&'static str,string:&'static [u8]) {
	let new_line = &b"\n"[..];
	let mut f = File::create(filename).unwrap();
	f.write_all(string);
	f.write_all(new_line);
	let local:DateTime<Local> = Local::now();
	println!("[GET] Time: {}",local);
}

/*fn main() {
	log_data("logs.roger",b"Server Status goes here: ");
}*/