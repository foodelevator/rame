pub mod application;
pub mod error;
pub mod events;
pub mod input;
pub mod layers;
pub mod logging;
pub mod vecs;
pub mod window;

#[cfg(test)]
mod tests {
	#[allow(unused_imports)]
	use super::*;
	#[test]
	fn it_works() {
		assert!(true && !false);
	}
}
