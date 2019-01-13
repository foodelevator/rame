use crate::Float;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec2 {
	pub x: Float,
	pub y: Float,
}

impl Vec2 {
	pub fn new(x: Float, y: Float) -> Vec2 {
		Vec2 { x, y }
	}
	pub fn right() -> Vec2 {
		Vec2 { x: 1 as Float, y: 0 as Float }
	}
	pub fn up() -> Vec2 {
		Vec2 { x: 0 as Float, y: 1 as Float }
	}
	pub fn left() -> Vec2 {
		Vec2 { x: -1 as Float, y: 0 as Float }
	}
	pub fn down() -> Vec2 {
		Vec2 { x: 0 as Float, y: -1 as Float }
	}
	pub fn zero() -> Vec2 {
		Vec2 { x: 0 as Float, y: 0 as Float }
	}
	pub fn mag_sq(&self) -> Float {
		self.x.powi(2) + self.y.powi(2)
	}
	pub fn mag(&self) -> Float {
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
