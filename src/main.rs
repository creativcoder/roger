#[macro_use]
extern crate nickel;
extern crate chrono;
extern crate inotify;

mod router;
mod file_op;
mod server_events;

use router::serve_localhost;

fn main() {
	serve_localhost();
}