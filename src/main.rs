#[macro_use]
extern crate nickel;

mod router;
mod file_op;

use router::serve_localhost;

fn main() {
	serve_localhost();
}