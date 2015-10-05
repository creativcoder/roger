
use nickel::Nickel;

use file_op::log_data;

pub fn serve_localhost() {
	let socket = "127.0.0.1:6767";
    let mut server = Nickel::new();
    server.utilize(router!{
    	get "**" => |_req,_res| {
    		log_data("logs.roger",b"Server Status goes here: ");
    		"Server Status : OK"
    	}
    });
    server.listen(socket);
}
