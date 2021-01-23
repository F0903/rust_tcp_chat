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
		let mut lock = self.stdin.lock();
		let mut string = String::new();
		let _ = lock.read_line(&mut string);
		string
	}
}
