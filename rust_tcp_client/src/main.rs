mod client;
mod console;

use console::{standard_console::StandardConsole, Console};
use std::sync::{Arc, Mutex};

const SERVER_ADDR: &str = "83.221.156.57:2";

fn main() {
	let input_client = Arc::new(Mutex::new(
		client::Client::start(SERVER_ADDR).expect("Couldnt start client."),
	));
	let read_client = input_client.clone();

	let console = Arc::new(Mutex::new(StandardConsole::new(move |mut x| {
		input_client
			.as_ref()
			.lock()
			.unwrap()
			.write(&mut x)
			.expect("Couldn't write to client.")
	})));

	let work_thread = std::thread::spawn(move || loop {
		let mut client = read_client.as_ref().lock().unwrap();
		let mut cons = console.as_ref().lock().unwrap();

		cons.check_for_input();
		if let Some((client_id, msg)) = client.get_msg() {
			/* if client_id == client.id() {
				continue;
			} */

			cons.writeline(format!(
				"[{}]: {}",
				client_id,
				msg.replace(&['\r', '\n'][..], "")
			));
		}
	});

	work_thread.join().expect("Work thread produced error: ");
}
