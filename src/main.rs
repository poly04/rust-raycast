
extern crate sfml;

use sfml::graphics::*;
use sfml::system::*;
use sfml::window::*;

mod map;
mod camera;
use map::Map;
use camera::*;

fn main() {
    let desktop = VideoMode::desktop_mode();
    let mut window = RenderWindow::new(VideoMode::new(800, 600, desktop.bits_per_pixel),
    								   "Ray Caster",
    								   Style::CLOSE,
    								   &ContextSettings::default());

    window.set_framerate_limit(60);

    let mut map = Map::new();
    map.set_solid(7, 5);
    map.set_solid(9, 5);
    map.set_solid(8, 3);

    let mut camera = Camera::new();

    let mut player_dot = CircleShape::new(3.0, 4);
    
    player_dot.set_fill_color(&Color::rgb(0, 0, 255));
    let mut render_target = RenderTexture::new(800, 600, false).unwrap();
    let mut img_arr: Vec<u8> = vec![0 as u8; 800*600*4];

    while window.is_open() {

    	for event in window.poll_event() {
    		match event {
    			Event::Closed => window.close(),

    			_ => {}
    		}
    	}

    	let rot_speed = 3.14159 / 32.0;
    	if Key::Left.is_pressed() {
    		camera.rot -= rot_speed;
    	} else if Key::Right.is_pressed() {
    		camera.rot += rot_speed;
    	}

    	let move_speed = 1.0;
    	if Key::W.is_pressed() {
    		camera.move_forward(move_speed, &mut map);
    	} else if Key::S.is_pressed() {
    		camera.move_forward(-move_speed, &mut map);
    	}
    	if Key::A.is_pressed() {
    		camera.strife(move_speed, &mut map);
    	} else if Key::D.is_pressed() {
    		camera.strife(-move_speed, &mut map);
    	}

    	player_dot.set_position(camera.position);

    	let mut solid_rect = RectangleShape::new();
    	solid_rect.set_fill_color(&Color::rgb(255, 0, 0));
    	solid_rect.set_size(Vector2f::new(10.0, 10.0));

    	
    	//render_target.set_active(true);
    	//render_target.clear(&Color::rgb(0, 0, 0));
    	img_arr = vec![0 as u8; 800*600*4];
    	for x in 0..800 {
    		let slice = camera.calculate_ray(&mut map, x);
    		let height = clamp(slice.height, 0.0, 600.0);

    		for y in 0..(height as i32) { 
    			img_arr[(((x + (y + (300 - (height/2.0) as i32))*800) * 4)    ) as usize] = slice.colour[0].r;
    			img_arr[(((x + (y + (300 - (height/2.0) as i32))*800) * 4) + 1) as usize] = slice.colour[0].g;
    			img_arr[(((x + (y + (300 - (height/2.0) as i32))*800) * 4) + 2) as usize] = slice.colour[0].b;
    			img_arr[(((x + (y + (300 - (height/2.0) as i32))*800) * 4) + 3) as usize] = 255;
    		}

    		/*
    		for y in 0..(height as i32) {
    			img.set_pixel(x as u32, ((y as f32) + (300.0 - (height / 2.0))) as u32, &Color::rgb(map_range(y as f32, 0.0, height, 0.0, 255.0) as u8, 
    													  map_range(y as f32, 0.0, height, 0.0, 255.0) as u8,
    													  map_range(y as f32, 0.0, height, 0.0, 255.0) as u8));
    		}
    		*/

    		/*
    		let mut ystrip = Image::new(1, height as u32);
    		for y in 0..(height as i32) {
    			ystrip.set_pixel(0, y as u32, &Color::rgb(map_range(y as f32, 0.0, height, 0.0, 255.0) as u8, 
    													  map_range(y as f32, 0.0, height, 0.0, 255.0) as u8,
    													  map_range(y as f32, 0.0, height, 0.0, 255.0) as u8));
    		}
    		let text = Texture::from_image(&ystrip).unwrap();
    		let mut spr = Sprite::new();
    		spr.set_texture(&text, true);
    		spr.set_origin(Vector2f::new(0.0, height/2.0));
    		spr.set_position(Vector2f::new(x as f32, 300.0));
    		render_target.draw(&spr);
    		*/
    	}
    	let mut img = Image::create_from_pixels(800, 600, &img_arr).unwrap();
    	let text = Texture::from_image(&img).unwrap();
    	let mut raycast_spr = Sprite::new();
    	raycast_spr.set_texture(&text, true);
    	raycast_spr.set_position(Vector2f::new(0.0, 0.0));
    	/*
    	render_target.display();
    	render_target.set_active(false);
    	let mut raycast_spr = Sprite::new();
    	raycast_spr.set_texture(render_target.texture(), true);
    	raycast_spr.set_position(Vector2f::new(0.0, 0.0));
    	*/

    	window.clear(&Color::rgb(0, 0, 0));

    	window.draw(&raycast_spr);
    	//window.draw(&raycast_spr);

    	let mut bg_rect = RectangleShape::new();
    	bg_rect.set_fill_color(&Color::rgb(0, 0, 0));
    	bg_rect.set_size(Vector2f::new(120.0, 120.0));
    	bg_rect.set_position(Vector2f::new(0.0, 0.0));
    	window.draw(&bg_rect);

    	for x in 0..12 {
    		for y in 0..12 {
    			if map.hits_solid((x*10) as f32, (y*10) as f32) {
    				solid_rect.set_position(Vector2f::new((x*10) as f32, (y*10) as f32));
    				window.draw(&solid_rect);
    			}
    		}
    	}

    	window.draw(&player_dot);
    	window.display();

    }
}
