use crate::logging::logger::Logger;
use std::io::{Read, Write};
use std::net::TcpStream;

pub struct Client<T: Logger> {
	_logger: T,
	stream: TcpStream,
}

impl<T: Logger> Client<T> {
	//TODO: Make client receive stuff aswell
	pub fn read(&mut self) {
		let mut buffer = [0; 1024];
		while let Ok(_read) = self.stream.read(&mut buffer) {
			T::log(
				String::from_utf8(buffer.to_vec())
					.unwrap_or_else(|_err| format!("Could not unwrap. Err: {}", _err)),
			);
		}
	}

	pub fn write(&mut self, msg: &mut str) -> std::io::Result<()> {
		unsafe {
			let bytes = msg.as_bytes_mut();
			let mut stream = &self.stream;
			stream.write_all(bytes)?;
			stream.flush()
		}
	}

	pub fn start(logger: T) -> std::io::Result<Client<T>> {
		const SERVER_ADDR: &str = "127.0.0.1:2";
		let stream = TcpStream::connect(SERVER_ADDR)?;
		T::log(format!("Connected to {}", SERVER_ADDR));
		Ok(Client {
			stream,
			_logger: logger,
		})
	}
}
