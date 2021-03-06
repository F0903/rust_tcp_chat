pub mod standard_console;

pub trait Outputtable {
	fn bytes(&self) -> &[u8];
}

impl Outputtable for str {
	fn bytes(&self) -> &[u8] {
		self.as_bytes()
	}
}

impl Outputtable for String {
	fn bytes(&self) -> &[u8] {
		self.as_bytes()
	}
}

pub trait Console {
	fn writeline<T: Outputtable + std::borrow::Borrow<T>>(&mut self, msg: T);
}
