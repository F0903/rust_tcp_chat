use super::{Console, Outputtable};
use crossterm::{cursor, terminal, ExecutableCommand};
use std::collections::VecDeque;
use std::convert::TryInto;
use std::io::{stdin, stdout, Read, Stdin, Stdout, Write};

const BACKSPACE: u8 = 0x08;

pub struct StandardConsole<F: Fn(String) + 'static> {
	stdout: Stdout,
	stdin: Stdin,
	input_buf: VecDeque<u8>,
	on_confirm: F,
}

impl<F: Fn(String) + Send + Sync> StandardConsole<F> {
	pub fn new(on_confirm: F) -> StandardConsole<F> {
		terminal::enable_raw_mode().expect("Could not enable raw mode.");
		StandardConsole {
			stdout: stdout(),
			stdin: stdin(),
			input_buf: VecDeque::new(),
			on_confirm,
		}
	}

	fn move_line_down(&mut self) {
		self.stdout
			.execute(cursor::MoveToColumn(0))
			.expect("Could not move down.");
		self.stdout
			.write_all(&[b'\n'])
			.expect("Could not write newline.");
		self.stdout
			.execute(cursor::MoveToColumn(
				self.input_buf.len().try_into().unwrap_or(0),
			))
			.expect("Could not move right.");
	}

	fn clear_line(&mut self, line_char_count: usize) {
		for _i in 0..line_char_count {
			self.stdout
				.write_all(&[BACKSPACE, b' ', BACKSPACE])
				.expect("Could not write backspace.");
		}
	}

	pub fn get_line(&mut self) -> String {
		if self.input_buf.is_empty() {
			return String::from("");
		}
		let len = self.input_buf.len();
		let line = self.input_buf.drain(..).rev().collect::<Vec<u8>>();
		self.clear_line(len);
		String::from_utf8(line).expect("Could not convert to string.")
	}

	fn get_key(&mut self) -> u8 {
		let mut buf = [0; 1];
		self.stdin.read_exact(&mut buf).expect("Couldn't read.");
		buf[0]
	}

	pub fn check_for_input(&mut self) {
		let key = self.get_key();
		match key {
			BACKSPACE => {
				self.stdout
					.write_all(&[BACKSPACE, b' ', BACKSPACE])
					.expect("Couldn't write.");
				self.input_buf.pop_back();
			}
			b'\r' => {
				let line = self.get_line();
				let f = &self.on_confirm;
				f(line);
			}
			_ => {
				self.stdout.write_all(&[key]).expect("Couldn't write.");
				self.input_buf.push_front(key);
			}
		};
		self.stdout.flush().expect("Couldn't flush.");
	}
}

impl<F: Fn(String) + Send + Sync> Console for StandardConsole<F> {
	fn writeline<T: Outputtable + std::borrow::Borrow<T>>(&mut self, msg: T) {
		self.stdout
			.write_all(msg.bytes())
			.expect("Couldn't write line.");
		self.move_line_down();
		self.stdout.flush().expect("Could not flush.");
	}
}
