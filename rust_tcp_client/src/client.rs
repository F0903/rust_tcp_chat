use log::info;
use std::io::{Read, Write};
use std::net::{IpAddr, TcpStream};

pub struct Client {
	stream: TcpStream,
}

//TODO: implement the log crate
impl Client {
	pub fn local_addr(&self) -> IpAddr {
		self.stream
			.local_addr()
			.expect("Could not get local_addr")
			.ip()
	}

	pub fn get_msg(&mut self) -> Option<(IpAddr, String)> {
		let bytes = self.read();
		if bytes.is_empty() {
			return None;
		}
		let full = std::str::from_utf8(&bytes).expect("Could not convert to str.");
		let mut split = full.split('|');

		let ip_port = split.next().expect("Could not get next item in split.");
		let msg = split.next().expect("Could not get next item in split.");
		Some((
			ip_port
				.split(':')
				.next()
				.unwrap()
				.parse::<IpAddr>()
				.expect("Could not get IpAddr from str"),
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
