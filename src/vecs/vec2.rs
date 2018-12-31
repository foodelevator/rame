#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec2 {
	pub x: f64,
	pub y: f64,
}

impl Vec2 {
	pub fn new(x: f64, y: f64) -> Vec2 {
		Vec2 { x, y }
	}
	pub fn right() -> Vec2 {
		Vec2 { x: 1.0, y: 0.0 }
	}
	pub fn up() -> Vec2 {
		Vec2 { x: 0.0, y: 1.0 }
	}
	pub fn left() -> Vec2 {
		Vec2 { x: -1.0, y: 0.0 }
	}
	pub fn down() -> Vec2 {
		Vec2 { x: 0.0, y: -1.0 }
	}
	pub fn zero() -> Vec2 {
		Vec2 { x: 0.0, y: 0.0 }
	}
	pub fn mag_sq(&self) -> f64 {
		self.x.powi(2) + self.y.powi(2)
	}
	pub fn mag(&self) -> f64 {
		self.mag_sq().sqrt()
	}
	pub fn normalized(&self) -> Vec2 {
		let mag = self.mag();
		Vec2 {
			x: self.x / mag,
			y: self.y / mag,
		}
	}
	// TODO: implement ops
}
