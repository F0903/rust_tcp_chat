use log::*;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub struct Server {
	socket: TcpListener,
	clients: Vec<TcpStream>,
	running: bool,
}

//TODO: implement the log crate
impl Server {
	fn send_to_all(&mut self, msg: &str) {
		self.clients
			.retain(|mut x| x.write_all(msg.as_bytes()).is_err());
	}

	fn receive_from_clients(&mut self) {
		let mut messages = Vec::<String>::new();
		self.clients.retain(|mut client| {
			let mut received = Vec::<u8>::new();
			let mut buffer = [0; 1024];
			let mut total_read = 0;
			while let Ok(read) = client.read(&mut buffer) {
				if read == 0 {
					return false;
				}
				received.extend_from_slice(&buffer);
				total_read += read;
			}
			if total_read < 1 {
				return true;
			}

			let received_string =
				String::from_utf8(received).expect("Could not convert received slice to string.");
			println!("Received message: {}", &received_string);
			messages.push(received_string);

			true
		});
		for msg in &messages {
			self.send_to_all(msg);
		}
	}

	pub fn listen(&mut self) {
		self.socket
			.set_nonblocking(true)
			.expect("Could not set non-blocking.");

		while self.running {
			if let Ok((sock, _addr)) = self.socket.accept() {
				println!("Incoming connection from: {}", _addr);
				self.clients.push(sock);
			}
			self.receive_from_clients();
		}
	}

	pub fn stop(mut self) {
		self.running = false;
	}

	pub fn bind(address: &str) -> std::io::Result<Server> {
		let listener = TcpListener::bind(address)?;
		println!("Socket bound to {}", address);
		Ok(Server {
			socket: listener,
			clients: Vec::new(),
			running: true,
		})
	}
}
