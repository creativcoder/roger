use std::thread;
use nickel::Nickel;

use file_op::log_and_write;
use server_events::filesys_events;

pub fn serve_localhost() {
	let socket = "127.0.0.1:6767";
    let mut server = Nickel::new();
    server.utilize(router!{
    	get "**" => |_req,_res| {
    		log_and_write("logs.roger",b"Request to Server received");
    		"Server Logging Status : OK"
    	}
    });
    thread::spawn(move || {
    	println!("Listening for file changes");
    	filesys_events();
    });
    server.listen(socket);
    
}
