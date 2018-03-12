
extern crate sfml;

use sfml::system::*;
use sfml::graphics::*;

use map::*;

pub struct Ray {
	pub rot: f32,
	pub position: Vector2f,
	pub distance: f32,
	pub relative_rot: f32,
	start_pos: Vector2f
}

impl Ray {
	pub fn new(pos: Vector2f) -> Ray {
		Ray {
			rot: 0.0,
			position: pos,
			distance: 0.0,
			relative_rot: 0.0,
			start_pos: pos
		}
	}

	pub fn move_forward(&mut self, amt: f32) {
		self.position += Vector2f::new(amt * self.rot.sin(), amt * self.rot.cos());
		self.distance += self.relative_rot.cos() * amt;
	}

	pub fn calculate_distance(&mut self) {
		let dist = ((self.start_pos.x - self.position.x).powf(2.0) + (self.start_pos.y - self.position.y).powf(2.0)).sqrt();
		self.distance = self.relative_rot.cos() * dist;
	}
}

pub struct XSlice {
	pub height: f32,
	pub colour: [Color; 64]
}

impl XSlice {
	pub fn new() -> XSlice {
		XSlice {
			height: 0.0,
			colour: [Color::rgb(0, 0, 0); 64]
		}
	}
}

pub struct Camera {
	pub position: Vector2f,
	pub rot: f32
}

impl Camera {
	pub fn new() -> Camera {
		Camera {
			position: Vector2f::new(5.0, 5.0),
			rot: 0.0
		}
	}

	pub fn calculate_ray(&self, map: &mut Map, x: i32) -> XSlice {
		let mut ray = Ray::new(self.position);
		ray.rot = map_range(x as f32, 0.0, 800.0, 0.0, 3.14159 / 2.0) + self.rot;
		ray.relative_rot = ((3.14159 / 4.0) - map_range(x as f32, 0.0, 800.0, 0.0, 3.14159 / 2.0));

		let mut amount = 0.1;
		while match map.get_tile_pixel(ray.position.x, ray.position.y) {
				TileEnum::Air => true,
				_ => false
		} {
			ray.move_forward(amount);
			amount += 0.01;
			amount = clamp(amount, 0.0, 1.0);
		}

		//ray.calculate_distance();

		let mut slice = XSlice::new();
		slice.height = (1.0 / ray.distance) * 3000.0;
		slice.colour = match map.get_tile_pixel(ray.position.x, ray.position.y) {
							TileEnum::Edge => [Color::rgb(0, 255, 255); 64],
							TileEnum::Solid => [Color::rgb(255, 0, 0); 64],
							_ => [Color::rgb(0, 0, 0); 64]
						 };

		return slice;
	}

	pub fn move_forward(&mut self, amt: f32, map: &mut Map) {
		let new_pos = self.position + Vector2f::new(amt * (self.rot + (3.14159 / 4.0)).sin(), amt * (self.rot + (3.14159 / 4.0)).cos());
		if map.hits_solid(new_pos.x, new_pos.y) {
			return;
		}
		self.position = new_pos;
	}

	pub fn strife(&mut self, amt: f32, map: &mut Map) {
		let new_pos = self.position + Vector2f::new(amt * (self.rot - (3.14159 / 4.0)).sin(), amt * (self.rot - (3.14159 / 4.0)).cos());
		if map.hits_solid(new_pos.x, new_pos.y) {
			return;
		}
		self.position = new_pos;
	}
}

pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
	if value < min {
		return min;
	} else if value > max {
		return max;
	}

	return value;
}

pub fn map_range(value: f32, a1: f32, a2: f32, b1: f32, b2: f32) -> f32 {
	let top = (value - a1) * (b2 - b1);
	let bottom = a2 - a1;
	let t = b1 + (top / bottom);
	return t;
}