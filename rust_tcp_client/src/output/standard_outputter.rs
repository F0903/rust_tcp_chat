use super::outputter::*;
use crossterm::{cursor, terminal, ExecutableCommand};
use std::io::{stdout, Stdout, Write};

pub struct StandardOutputter {
	stdout: Stdout,
}

impl StandardOutputter {
	pub fn new() -> StandardOutputter {
		terminal::enable_raw_mode().expect("Could not enable raw mode.");
		StandardOutputter { stdout: stdout() }
	}

	fn move_to_bottom(&mut self) {
		let (_y, x) = cursor::position().expect("Could not get cursor position.");
		let (cols, _rows) = terminal::size().expect("Could not get size of console.");
		self.stdout
			.execute(cursor::MoveTo(cols, x))
			.expect("Could not execute MoveTo command.");
	}
}

impl Outputter for StandardOutputter {
	fn writeline<T: Outputtable + std::borrow::Borrow<T>>(&mut self, msg: T) {
		let mut out = &self.stdout;
		out.write_all(msg.bytes())
			.expect("Could not write to output.");
		out.write_all(&[b'\n']).expect("Could not write newline.");
		out.flush().expect("Could not flush stdout.");
	}
}
