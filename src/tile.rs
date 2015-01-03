use sdl2::render::Renderer;
use sdl2_image::LoadSurface;

enum Status {
    Empty,
    Loaded
}

pub struct Tile {
    status: Status
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            status: Status::Empty
        }
    }
    pub fn render(&self, renderer: &Renderer) -> () {
        let png = &Path::new("./data/tiles/10895.png");
        let surface = match LoadSurface::from_file(png) {
            Ok(surface) => surface,
            Err(err) => panic!(format!("Failed to load png: {}", err))
        };
        let texture = match
            renderer.create_texture_from_surface(&surface) {
                Ok(texture) => texture,
                Err(err) => panic!(format!("Failed to create surface: {}", err))
            };
        println!("{}", 4);
        let _ = renderer.copy(&texture, None, None);
    }
}

