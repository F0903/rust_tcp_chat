use log::info;
use std::io::{Read, Write};
use std::net::TcpStream;

pub struct Client {
	stream: TcpStream,
}

//TODO: implement the log crate
impl Client {
	pub fn read<F: Fn(&str)>(&mut self, on_read: F) {
		let mut received = Vec::<u8>::new();
		let mut buffer = [0; 1024];
		while let Ok(read) = self.stream.read(&mut buffer) {
			if read < 1 {
				panic!("Read was less than one.");
			}
			received.extend_from_slice(&buffer);
		}
		if received.is_empty() {
			return;
		}
		on_read(std::str::from_utf8(&received).expect("Could not convert to str."));
	}

	pub fn write(&mut self, msg: &mut str) -> std::io::Result<()> {
		unsafe {
			let bytes = msg.as_bytes_mut();
			let mut stream = &self.stream;
			stream.write_all(bytes)?;
			stream.flush()
		}
	}

	pub fn start() -> std::io::Result<Client> {
		const SERVER_ADDR: &str = "127.0.0.1:2";
		let stream = TcpStream::connect(SERVER_ADDR)?;
		stream
			.set_nonblocking(true)
			.expect("Could not set non_blocking.");
		println!("Connected to {}", SERVER_ADDR);
		Ok(Client { stream })
	}
}
