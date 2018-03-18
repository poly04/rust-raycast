
extern crate sfml;

use sfml::system::*;
use sfml::graphics::*;

use map::*;

pub struct Ray {
	pub rot: f32,
	pub position: Vector2f,
	pub distance: f32,
	pub relative_rot: f32,
	start_pos: Vector2f,
	prev_near: Vector2f
}

impl Ray {
	pub fn new(pos: Vector2f) -> Ray {
		Ray {
			rot: 0.0,
			position: pos,
			distance: 0.0,
			relative_rot: 0.0,
			start_pos: pos,
			prev_near: Vector2f::new(0.0, 0.0)
		}
	}

	pub fn move_forward(&mut self) {
		let rot_vec = Vector2f::new(self.rot.sin(), self.rot.cos());

		let mut map_pos = Vector2f::new((self.position.x / 10.0).round(), (self.position.y / 10.0).round());

		//find distance to X gridline
		let mut new_xpos: f32 = 0.0;
		let mut x_dist: f32 = 0.0;
		let mut x_gridline_dist: f32 = 0.0;
		if rot_vec.x.signum() > 0.0 {
			map_pos.x = (self.position.x / 10.0).floor();
			new_xpos = (map_pos.x + 1.0) * 10.0;
			x_dist = new_xpos - self.position.x;
			x_gridline_dist = x_dist / self.rot.sin();
		} else {
			map_pos.x = (self.position.x / 10.0).ceil();
			new_xpos = (map_pos.x - 1.0) * 10.0;
			x_dist = new_xpos - self.position.x;
			x_gridline_dist = x_dist / self.rot.sin();
		}

		//let x_gridline_dist: f32 = x_dist / self.rot.sin();

		//find distance to Y gridline
		let mut new_ypos: f32 = 0.0;
		let mut y_dist: f32 = 0.0;
		let mut y_gridline_dist: f32 = 0.0;
		if rot_vec.y.signum() > 0.0 {
			map_pos.y = (self.position.y / 10.0).floor();
			new_ypos = (map_pos.y + 1.0) * 10.0;
			y_dist = new_ypos - self.position.y;
			y_gridline_dist = y_dist / self.rot.cos();
		} else {
			map_pos.y = (self.position.y / 10.0).ceil();
			new_ypos = (map_pos.y - 1.0) * 10.0;
			y_dist = new_ypos - self.position.y;
			y_gridline_dist = y_dist / self.rot.cos();
		}

		//let y_gridline_dist: f32 = y_dist / self.rot.cos();

		//move by whichever gridline is closer
		
		let mut new_pos: Vector2f;
		if y_gridline_dist > x_gridline_dist {
			new_pos = x_gridline_dist * rot_vec;
		} else {
			new_pos = y_gridline_dist * rot_vec;
		}

		self.position += new_pos;

		/*
		self.position += match x_gridline_dist < y_gridline_dist {
							true => x_gridline_dist * rot_vec,
							false => y_gridline_dist * rot_vec
						};
		*/


		

		//self.position += Vector2f::new(amt * rot_vec.x, amt * rot_vec.y);
	}

	

	pub fn calculate_distance(&mut self, x: i32) {
		
		let dist = ((self.start_pos.x - self.position.x).powf(2.0) + (self.start_pos.y - self.position.y).powf(2.0)).sqrt();
		self.distance = dist * self.relative_rot.cos();
	}
}

pub struct XSlice {
	pub height: f32,
	pub texture_xoffs: f32
}

impl XSlice {
	pub fn new() -> XSlice {
		XSlice {
			height: 0.0,
			texture_xoffs: 0.0
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
		let ray_pos = map_range(x as f32, 0.0, 800.0, -25.0, 25.0);
		let ray_left = map_range(x as f32, 0.0, 800.0, -400.0, 400.0).signum();
		let sight_line_angle = ray_left * ((3.14159 / 2.0) + self.rot);
		let min_angle: f32 = -45.0;
		let max_angle: f32 = 45.0;
		let ray_angle = map_range(x as f32, 0.0, 800.0, min_angle.to_radians(), max_angle.to_radians()) + self.rot;
		let ray_pos_x = self.position.x + (ray_angle.sin() * 10.0) + (ray_pos * sight_line_angle.sin());
		let ray_pos_y = self.position.y + (ray_angle.cos() * 10.0) + (ray_pos * sight_line_angle.cos());

		let mut ray = Ray::new(self.position);
		ray.rot = ray_angle;
		ray.relative_rot = ray_angle - self.rot;

		let rot_vec = Vector2f::new(ray.rot.sin(), ray.rot.cos());

		let mut map_x: i32 = match rot_vec.x > 0.0 {
							true => (ray.position.x / 10.0).floor() as i32,
							_ => ((ray.position.x / 10.0).ceil() - 1.0) as i32 };
		let mut map_y: i32 = match rot_vec.y > 0.0 {
							true => (ray.position.y / 10.0).floor() as i32,
							_ => ((ray.position.y / 10.0).ceil() - 1.0) as i32 };

		while match map.get_tile(map_x, map_y) {
				TileEnum::Air => true,
				_ => false
		} {
			ray.move_forward();

			map_x = match rot_vec.x > 0.0 {
							true => ((ray.position.x / 10.0).floor()) as i32,
							_ => ((ray.position.x / 10.0).ceil() - 1.0) as i32 };
			map_y = match rot_vec.y > 0.0 {
							true => ((ray.position.y / 10.0).floor()) as i32,
							_ => ((ray.position.y / 10.0).ceil() - 1.0) as i32 };
		}

		ray.calculate_distance(x);

		let mut slice = XSlice::new();
		slice.height = (1.0 / ray.distance) * 3000.0;

		let pos_in_block = Vector2f::new(((ray.position.x - (map_x * 10) as f32) / 10.0), ((ray.position.y - (map_y * 10) as f32) / 10.0));
		let mut x_offs: f32 = 0.0;
		if pos_in_block.x == 0.0 || pos_in_block.x == 1.0 {
			x_offs = pos_in_block.y;
		}

		if pos_in_block.y == 0.0 || pos_in_block.y == 1.0 {
			x_offs = pos_in_block.x;
		}
		//println!("Pos in block: {}, {}", pos_in_block.x, pos_in_block.y);

		slice.texture_xoffs = x_offs;

		return slice;
	}

	pub fn move_forward(&mut self, amt: f32, map: &mut Map) {
		let new_pos = self.position + Vector2f::new(amt * self.rot.sin(), amt * self.rot.cos());
		if map.hits_solid(new_pos.x, new_pos.y) {
			return;
		}
		self.position = new_pos;
	}

	pub fn strife(&mut self, amt: f32, map: &mut Map) {
		let angle = self.rot + (3.14159 / 2.0);
		let new_pos = self.position + Vector2f::new(amt * angle.sin(), amt * angle.cos());
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

pub fn step(rise: f32, run: f32, x: f32, y: f32, inverted: bool) -> Vector2f {
	let dx: f32 = match run > 0.0 {
				true => (x + 1.0).floor() - x,
				false => (x - 1.0).ceil() - x
			};
	let dy = dx * (rise / run);

	return Vector2f::new(match inverted {
							true => y + dy, 
							false => x + dx
						}, match inverted {
							true => x + dx,
							false => y + dy
						});
}