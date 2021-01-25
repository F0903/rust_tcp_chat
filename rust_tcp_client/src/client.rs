use std::io::{Read, Write};
use std::net::TcpStream;

pub struct Client {
	id: i32,
	stream: TcpStream,
}

impl Client {
	pub fn id(&self) -> i32 {
		self.id
	}

	pub fn get_msg(&mut self) -> Option<(i32, String)> {
		let bytes = self.read();
		if bytes.is_empty() {
			return None;
		}
		let full = std::str::from_utf8(&bytes).expect("Could not convert to str.");
		let mut split = full.split('|');

		let client_id = split.next().expect("Could not get next item in split.");
		let msg = split.next().expect("Could not get next item in split.");
		Some((
			client_id
				.parse::<i32>()
				.expect("Could not parse split result to i32."),
			String::from(msg),
		))
	}

	pub fn read(&mut self) -> Vec<u8> {
		let mut received = Vec::<u8>::new();
		let mut buffer = [0; 1024];
		while let Ok(read) = self.stream.read(&mut buffer) {
			if read < 1 {
				panic!("Read was less than one.");
			}
			received.extend_from_slice(&buffer[..read]);
		}
		received
	}

	pub fn write(&mut self, msg: &mut str) -> std::io::Result<()> {
		unsafe {
			let bytes = msg.as_bytes_mut();
			let mut stream = &self.stream;
			stream.write_all(bytes)
		}
	}

	pub fn start(server_addr: &str) -> std::io::Result<Client> {
		let mut stream = TcpStream::connect(server_addr)?;
		println!("Connected to {}", server_addr);

		let mut buf = [0; 4];
		stream
			.read_exact(&mut buf)
			.expect("Could not read from stream.");
		let id = i32::from_le_bytes(buf);
		println!("Received ID {}", id);

		stream
			.set_nonblocking(true)
			.expect("Could not set non_blocking.");
		Ok(Client { id, stream })
	}
}
