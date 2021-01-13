use super::logger::Logger;

pub struct StandardLogger {}

impl Logger for StandardLogger {
	fn log(msg: String) {
		println!("{}", msg);
	}
}
