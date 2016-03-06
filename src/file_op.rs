use std::fs::File;
use std::io::*;
use chrono::*;

pub fn log_and_write(filename:&'static str,log_string:&'static [u8]) {
	let timestamp:DateTime<Local> = Local::now();
	println!("[GET] Time: {}",timestamp);
	let new_line = &b"\n"[..];
	let mut f = File::create(filename).unwrap();
	f.write_all(log_string);
	f.write_all(new_line);
	f.write_all(timestamp);
	f.write_all(new_line);
}

pub fn log_and_write_filesys(filename:&'static str,log_string:&'static [u8]) {
	// Todo
}