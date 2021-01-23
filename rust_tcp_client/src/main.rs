mod client;
mod input;

use input::inputter::Inputter;
use input::standard_inputter::StandardInputter as In;
use log::*;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

fn input(client: &mut client::Client) {
	let mut inputter = In::new();
	let mut text = inputter.get();
	client.write(&mut text).expect("Write failed.");
	println!("Sucessfully wrote to remote.");
}

fn main() {
	let result = client::Client::start().expect("Couldnt start client.");

	let read_client = Arc::new(Mutex::new(result));
	let input_client = read_client.clone();

	let read_thread = std::thread::spawn(move || loop {
		{
			read_client
				.as_ref()
				.lock()
				.unwrap()
				.deref_mut()
				.read(|x| println!("Received: {}", x));
		}
	});

	let input_thread = std::thread::spawn(move || loop {
		{
			input(input_client.as_ref().lock().unwrap().deref_mut());
		}
	});

	read_thread.join().expect("Read thread produced error: ");
	input_thread.join().expect("Input thread produced error: ");
}
