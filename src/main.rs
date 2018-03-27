
extern crate sfml;

use sfml::graphics::*;
use sfml::system::*;
use sfml::window::*;

use std::f32;

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
    let texture = Image::from_file("bricks.bmp").unwrap();

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
    		camera.strife(-move_speed, &mut map);
    	} else if Key::D.is_pressed() {
    		camera.strife(move_speed, &mut map);
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
            let height_diff = (slice.height - height) / 2.0;

    		let texture_xoffs = (texture.size().x as f32) * slice.texture_xoffs;

    		for y in 0..(height as i32) {
    			//let mapped_y = map_range(y as f32, 0.0, height, 0.0, slice.height);
    			let texture_y = map_range((y as f32) + height_diff, 0.0, slice.height, 0.0, texture.size().y as f32);
    			//if slice.height < 600.0 && slice.height > 0.0 {
					img_arr[(((x + (y + (300 - (height/2.0) as i32))*800) * 4)    ) as usize] = texture.pixel_at(texture_xoffs.floor() as u32, texture_y.floor() as u32).r;
					img_arr[(((x + (y + (300 - (height/2.0) as i32))*800) * 4) + 1) as usize] = texture.pixel_at(texture_xoffs.floor() as u32, texture_y.floor() as u32).g;
					img_arr[(((x + (y + (300 - (height/2.0) as i32))*800) * 4) + 2) as usize] = texture.pixel_at(texture_xoffs.floor() as u32, texture_y.floor() as u32).b;
					img_arr[(((x + (y + (300 - (height/2.0) as i32))*800) * 4) + 3) as usize] = texture.pixel_at(texture_xoffs.floor() as u32, texture_y.floor() as u32).a;
    			//}
    			
    		}
    		
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

    	//test texture loading
    	//let texture = Texture::from_file("texture.bmp").unwrap();
    	//let sprite = Sprite::with_texture(&texture);


    	window.draw(&player_dot);
    	//window.draw(&sprite);
    	window.display();

    }
}
