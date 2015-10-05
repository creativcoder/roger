
use nickel::Nickel;

use file_op::log_and_write;

pub fn serve_localhost() {
	let socket = "127.0.0.1:6767";
    let mut server = Nickel::new();
    server.utilize(router!{
    	get "**" => |_req,_res| {
    		log_and_write("logs.roger",b"Request to Server received");
    		"Server Logging Status : OK"
    	}
    });
    server.listen(socket);
}
