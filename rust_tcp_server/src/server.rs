use crate::logging::logger::Logger;
use std::io::{ErrorKind, Read};
use std::net::{TcpListener, TcpStream};

pub struct Server<L: Logger + Send + Sync> {
	_logger: L,
	socket: TcpListener,
	clients: Vec<TcpStream>,
	running: bool,
}

impl<L: Logger + Send + Sync> Server<L> {
	fn receive_from_clients(&self) {
		for mut client in &self.clients {
			let mut received = Vec::<u8>::new();
			let count = match client.read_to_end(&mut received) {
				Ok(val) => val,
				Err(e) if e.kind() == ErrorKind::WouldBlock => received.len(), // Dunno if this is proper, but the read_to_end keeps returning WOULD_BLOCK, even on success
				Err(e) => panic!("Encountered unknown IO error: {}", e),
			};
			if count < 1 {
				continue;
			}

			L::log(format!(
				"Received message: {}",
				std::str::from_utf8(received.as_slice())
					.expect("Could not convert received slice to str")
			));
		}
	}

	pub fn listen(&mut self) {
		self.socket
			.set_nonblocking(true)
			.expect("Could not set non-blocking.");

		while self.running {
			if let Ok((sock, _addr)) = self.socket.accept() {
				L::log(format!("Incoming connection from: {}", _addr));
				self.clients.push(sock);
			}
			self.receive_from_clients();
		}
	}

	pub fn stop(mut self) {
		self.running = false;
	}

	pub fn bind(address: &str, logger: L) -> std::io::Result<Server<L>> {
		let listener = TcpListener::bind(address)?;
		L::log(format!("Socket bound to {}", address));
		Ok(Server {
			socket: listener,
			_logger: logger,
			clients: Vec::new(),
			running: true,
		})
	}
}
