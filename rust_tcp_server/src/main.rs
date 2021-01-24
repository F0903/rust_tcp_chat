mod server;
use server::Server;

const SERVER_ADDR: &str = "192.168.1.2:2";

fn main() -> std::io::Result<()> {
	let mut server = Server::bind(SERVER_ADDR)?;
	server.listen();
	server.stop();
	Ok(())
}
