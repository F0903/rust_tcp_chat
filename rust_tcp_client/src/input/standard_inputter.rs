use crate::input::inputter::Inputter;
use std::io::{self, BufRead, Stdin};

pub struct StandardInputter {
	stdin: Stdin,
}

impl Inputter for StandardInputter {
	fn new() -> StandardInputter {
		StandardInputter { stdin: io::stdin() }
	}

	fn get(&mut self) -> String {
		let mut strbuf = String::new();
		let mut lock = self.stdin.lock();
		lock.read_line(&mut strbuf)
			.expect("Could not read to string.");
		strbuf
	}
}
