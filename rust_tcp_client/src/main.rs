mod client;
mod input;

use input::inputter::Inputter;
use input::standard_inputter::StandardInputter;
use std::sync::{Arc, Mutex};

const SERVER_ADDR: &str = "83.221.156.57:2";

fn main() {
	let input_client = Arc::new(Mutex::new(
		client::Client::start(SERVER_ADDR).expect("Couldnt start client."),
	));
	let read_client = input_client.clone();

	let input_thread = std::thread::spawn(move || {
		let mut inputter = StandardInputter::new();
		loop {
			inputter.get_callback(|x| {
				input_client
					.as_ref()
					.lock()
					.unwrap()
					.write(x)
					.expect("Could not write.");
			});
		}
	});

	let read_thread = std::thread::spawn(move || loop {
		let mut client = read_client.as_ref().lock().unwrap();
		if let Some((addr, msg)) = client.get_msg() {
			if addr == client.local_addr() {
				continue;
			}
			println!("[{}]: {}", addr, msg.replace("\r\n", ""));
		}
	});

	input_thread.join().expect("Input thread produced error: ");
	read_thread.join().expect("Read thread produced error: ");
}
