use sdl2::rect;
use sdl2::render;

use sync::Arc;
use game::graphics;

use game::units;
use game::units::{AsPixel};

#[deriving(Hash,Eq)]
pub enum Motion {
	Walking,
	Standing,
	Interacting,
	Jumping,
	Falling
}
pub static MOTIONS: [Motion, ..5] = [Walking, Standing, Interacting, Jumping, Falling];


#[deriving(Hash,Eq)]
pub enum Facing {
	West,
	East
}
pub static FACINGS: [Facing, ..2] = [West, East];

#[deriving(Hash,Eq)]
pub enum Looking {
	Up,
	Down,
	Horizontal
}
pub static LOOKINGS: [Looking, ..3] = [Up, Down, Horizontal];

/// Any object which can be represented in 2D space
pub trait Drawable { 
	fn draw(&self, display: &graphics::Graphics); 
}

/// Any object which understands time and placement in 2D space.
pub trait Updatable : Drawable { 
	fn update(&mut self, elapsed_time: units::Millis); 
	fn set_position(&mut self, coords: (units::Game,units::Game));
}

/// Represents a static 32x32 2D character
pub struct Sprite {
	sprite_sheet: Arc<~render::Texture>, 
	source_rect: rect::Rect,
	size: (units::Tile, units::Tile),
	coords: (units::Game,units::Game),
}

impl Sprite {
	/// A new sprite which will draw itself at `coords`
	/// `sprite_at` is the index (row) where the sprite starts in `file_name`
	pub fn new(
		graphics: &mut graphics::Graphics, 
		coords: (units::Game,units::Game), // position on screen
		offset: (units::Tile,units::Tile), // source_x, source_y
		size: 	(units::Tile,units::Tile), // width, height
		file_name: ~str
	) -> Sprite {
		let (w,h) = size;
		let (x,y) = offset;
		let (units::Pixel(wi), units::Pixel(hi)) = (w.to_pixel(), h.to_pixel());
		let (units::Pixel(xi), units::Pixel(yi)) = (x.to_pixel(), y.to_pixel());
	
		let origin = rect::Rect::new(xi,yi,wi,hi);
		let sheet = graphics.load_image(file_name, true); // request graphics subsystem cache this sprite.

		let sprite = Sprite{
			sprite_sheet: sheet,
			source_rect: origin,
			size:	size,
			coords: coords,
		};

		sprite
	}
}

impl Drawable for Sprite {
	/// Draws selfs @ coordinates provided by 
	fn draw(&self, display: &graphics::Graphics) {
		let (w,h) = self.size;
		let (x,y) = self.coords;
		
		let (units::Pixel(wi), units::Pixel(hi)) = (w.to_pixel(), h.to_pixel());
		let (units::Pixel(xi), units::Pixel(yi)) = (x.to_pixel(), y.to_pixel());
	
		let dest_rect = rect::Rect::new(xi, yi, wi, hi);

		display.blit_surface(*(self.sprite_sheet.get()), &self.source_rect, &dest_rect);
	}
}

#[allow(unused_variable)]
impl Updatable for Sprite {
	fn update(&mut self, elapsed_time: units::Millis) {
		// no-op for static sprite.
	}

	fn set_position(&mut self, coords: (units::Game,units::Game)) {
		self.coords = coords;
	}
}

/// Represents a 32x32 2D character w/ a number of frames
/// Frames will be selected based on time-deltas supplied through update
pub struct AnimatedSprite {
	source_rect: rect::Rect,
	sprite_sheet: Arc<~render::Texture>, 

	priv coords: (units::Game, units::Game),
	priv offset: (units::Tile, units::Tile),
	priv size: 	 (units::Tile, units::Tile),
	priv current_frame: units::Frame,
	priv num_frames: units::Frame,
	priv fps: units::Fps,

	priv last_update: units::Millis
}

impl AnimatedSprite {
	/// Loads character sprites from `assets/MyChar.bmp`
	/// `source_rect` acts as a viewport of this sprite-sheet.
	///
	/// Returns an error message if sprite-sheet could not be loaded.
	pub fn new(
		graphics: &mut graphics::Graphics, 
		sheet_path: ~str, 
		offset: (units::Tile, units::Tile),
		size: 	(units::Tile, units::Tile),
		num_frames: units::Frame,
		fps: units::Fps
	) -> Result<AnimatedSprite, ~str> {
		// attempt to load sprite-sheet from `assets/MyChar.bmp`
		let (w,h) = size;
		let (x,y) = offset;
	
		let (units::Pixel(wi), units::Pixel(hi)) = (w.to_pixel(), h.to_pixel());	
		let (units::Pixel(xi), units::Pixel(yi)) = (x.to_pixel(), y.to_pixel());
		
		let origin = rect::Rect::new(xi, yi, wi, hi);
		
		let sheet = graphics.load_image(sheet_path, true); // request graphics subsystem cache this sprite.
		let sprite = AnimatedSprite{
			offset: offset,
			coords: (units::Game(0.0), units::Game(0.0)),
			size: size,
			
			fps: fps,
			current_frame: 0, 
			num_frames: num_frames, 	// our frames are drawin w/ a 0-idx'd window.
			last_update: units::Millis(0),
			
			sprite_sheet: sheet, 	// "i made this" -- we own this side of the Arc()
			source_rect: origin
		};

		return Ok(sprite);
	}
}

impl Updatable for AnimatedSprite {
	/// Reads current time-deltas and mutates state accordingly.
	fn update(&mut self, elapsed_time: units::Millis) {
		let frame_time = units::Millis(1000 / self.fps as int);	
		self.last_update = self.last_update + elapsed_time;

		// if we have missed drawing a frame
		if self.last_update > frame_time {		
			self.last_update = units::Millis(0);	// reset timer
			self.current_frame += 1;				// increment frame counter

			if self.current_frame < self.num_frames {
				self.source_rect.x += self.source_rect.w;
			} else {
				self.current_frame = 0;
				self.source_rect.x -= self.source_rect.w * (self.num_frames - 1) as i32;
			}
		}
	}

	fn set_position(&mut self, coords: (units::Game,units::Game)) {
		self.coords = coords;
	}
}

impl Drawable for AnimatedSprite {
	/// Draws selfs @ coordinates provided by 
	fn draw(&self, display: &graphics::Graphics) {
		let (w,h) = self.size;
		let (x,y) = self.coords;
		let (units::Pixel(wi), units::Pixel(hi)) = (w.to_pixel(), h.to_pixel());
		let (units::Pixel(xi), units::Pixel(yi)) = (x.to_pixel(), y.to_pixel());

		let dest_rect = rect::Rect::new(xi, yi, wi, hi);
		display.blit_surface(*(self.sprite_sheet.get()), &self.source_rect, &dest_rect);
	}
}
