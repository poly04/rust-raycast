
#[derive(Copy, Clone)]
pub enum TileEnum {
	Air,
	Solid,
	Edge
}

pub struct Tile {
	variant: TileEnum,
	dist_across: f32
}

pub struct Map {
	pub tiles: [[TileEnum; 11]; 11]
}

impl Map {
	pub fn new() -> Map {
		Map {
			tiles: [[TileEnum::Air; 11]; 11]
		}
	}

	//tiles are 10x10 pixels
	pub fn hits_solid(&self, x: f32, y: f32) -> bool {
		if x/10.0 < 0.0 || x/10.0 > 10.0 {
			return true;
		}

		if y/10.0 < 0.0 || y/10.0 > 10.0 {
			return true;
		}

		match self.tiles[(x/10.0) as usize][(y/10.0) as usize] {
			TileEnum::Solid => return true,
			TileEnum::Air => return false,
			TileEnum::Edge => return true
		}
	}

	pub fn set_solid(&mut self, x: i32, y: i32) {
		self.tiles[x as usize][y as usize] = TileEnum::Solid;
	}

	pub fn get_tile(&mut self, x: i32, y: i32) -> TileEnum {
		self.tiles[x as usize][y as usize]
	}

	pub fn get_tile_pixel(&mut self, x: f32, y: f32) -> TileEnum {

		if x/10.0 < 0.0 || x/10.0 > 10.0 {
			return TileEnum::Edge;
		}

		if y/10.0 < 0.0 || y/10.0 > 10.0 {
			return TileEnum::Edge;
		}

		return self.get_tile((x/10.0) as i32, (y/10.0) as i32);
	}
}