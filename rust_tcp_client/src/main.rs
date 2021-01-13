mod client;
mod input;
mod logging;

use input::inputter::Inputter;
use input::standard_inputter::StandardInputter as In;
use logging::logger::Logger;
use logging::standard_logger::StandardLogger as Log;

fn input(client: &mut client::Client<Log>) {
	loop {
		let mut inputter = In::new();
		let mut text = inputter.get();
		client.write(&mut text[..]).expect("Write failed.");
		Log::log(String::from("Sucessfully wrote to remote."));
	}
}

fn main() {
	let result = client::Client::start(Log::new());
	if result.is_err() {
		Log::log(format!(
			"Client could not start. Err: {}",
			result.err().unwrap()
		));
		return;
	}
	let mut client = result.unwrap();
	let in_thread = std::thread::spawn(move || input(&mut client));
	in_thread.join().unwrap();
}
