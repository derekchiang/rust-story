extern mod sdl;
use std::io::Timer;

pub mod graphics;
pub mod sprite;

static TARGET_FRAMERATE: int = 60;

/// An instance of the `rust-story` game with its own event loop.
pub struct Game {
	sprite: int
}

// TODO: destructor
impl Game {
	/// Starts running this games event loop, note that this will block indefinitely.
	/// This task will close SDL cleanly & return control to the caller when the
	/// `escape` key is pressed
	pub fn start(&self) {
		println!("initalizing sdl ...");
		
		// initialize all major subsystems
		// hide the mouse cursor in our drawing context
		sdl::init([sdl::InitEverything]);
		sdl::mouse::set_cursor_visible(false);

		self.event_loop();
	}

	pub fn stop(&self) {
		println!("quitting sdl ...");
		sdl::quit();
	}

	fn event_loop(&self) {
		let display = graphics::Graphics();
		
		// event loop control
		let last_update_time = sdl::sdl::get_ticks();
		let frame_delay = (1000 / TARGET_FRAMERATE);

		let mut running = true;
		let mut timer = Timer::new().unwrap();

		// load quote's sprite
		let quote;
		match sprite::Sprite::new() {
			Ok(loaded_sprite) => {
				quote = loaded_sprite;
				println!("sprite = ok");
			}
			Err(msg) => {
				println!("sprite err: {}", msg); 
				running = false; 
				fail!("nope");
			}
		}

		while running {
			let start_time_ms = sdl::sdl::get_ticks();

			// drain event queue once per frame
			// ideally should do in separate task
			match sdl::event::poll_event() {
				sdl::event::KeyEvent(keyCap,_,_,_) if keyCap == sdl::event::EscapeKey => {
					running = false;
				}
				_ => {}
			}


			display.blit_surface(quote.sprite_sheet, &quote.source_rect);
			display.switch_buffers();
			
			timer.sleep(frame_delay as u64);
		}

	}
}