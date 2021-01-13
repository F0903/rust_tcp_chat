pub trait Logger {
	fn new() -> Self;
	fn log(msg: String);
}
