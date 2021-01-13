pub trait Inputter {
	fn new() -> Self;
	fn get(&mut self) -> String;
}
