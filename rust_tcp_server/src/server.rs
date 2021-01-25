use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub struct Server {
	socket: TcpListener,
	clients: HashMap<i32, TcpStream>,
	running: bool,
	last_id: i32,
}

impl Server {
	fn send_to_all(&mut self, msg: &str) {
		self.clients.retain(|id, x| {
			let mut to_send = String::new();
			to_send.insert_str(0, format!("{}|", id).as_str());
			to_send.push_str(msg);

			let ok = x.write_all(to_send.as_bytes()).is_ok();
			if ok {
				println!("Sent msg to {}|{:?}", id, x.peer_addr());
			}
			ok
		});
	}

	fn receive_from_clients(&mut self) {
		let mut messages = Vec::<String>::new();
		self.clients.retain(|_id, client| {
			let mut received = Vec::<u8>::new();
			let mut buffer = [0; 1024];
			while let Ok(read) = client.read(&mut buffer) {
				if read == 0 {
					return false;
				}
				received.extend_from_slice(&buffer[..read]);
			}
			if received.is_empty() {
				return true;
			}

			let received_string = String::from_utf8(received[..received.len()].to_vec())
				.expect("Could not convert received bytes to string.");
			println!("Received message: {}", &received_string);
			messages.push(received_string);

			true
		});
		messages.iter().for_each(|x| self.send_to_all(x));
	}

	fn assign_id(&mut self, client: TcpStream) {
		let id = self.last_id;
		self.last_id += 1;
		self.clients.insert(id, client);
		let mut client = self
			.clients
			.get(&id)
			.expect("Could not get client from map.");

		client
			.write_all(&id.to_le_bytes())
			.expect("Could not write ID to client.");
		println!("Sent id of {} to {}", id, client.peer_addr().unwrap());
	}

	pub fn listen(&mut self) {
		self.socket
			.set_nonblocking(true)
			.expect("Could not set non-blocking.");

		while self.running {
			if let Ok((sock, addr)) = self.socket.accept() {
				println!("Incoming connection from: {}", addr);
				self.assign_id(sock);
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
			clients: HashMap::new(),
			running: true,
			last_id: 0,
		})
	}
}
