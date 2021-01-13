mod logging;
mod server;
use server::Server;

fn main() -> std::io::Result<()> {
	let mut server = Server::bind("127.0.0.1:2", logging::standard_logger::StandardLogger {})?;
	server.listen();
	server.stop();
	Ok(())
}
