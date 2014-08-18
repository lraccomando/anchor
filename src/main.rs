extern crate http;
extern crate iron;
extern crate anchor;

use std::io::net::ip::Ipv4Addr;
use iron::{Iron, Chain, Server};

use anchor::{Anchor, App};

fn main () {
	let mut server: Server = Iron::new();
	let mut app: App = Anchor::new();
	server.chain.link(app);
	server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}
