mod server;
use log::*;
use server::Server;

fn main() -> std::io::Result<()> {
	let mut server = Server::bind("127.0.0.1:2")?;
	server.listen();
	server.stop();
	Ok(())
}
