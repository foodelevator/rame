#[derive(Debug)]
pub struct Error {
	
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
		write!(f, "Rame error")
	}
}

impl std::error::Error for Error {
	
}
