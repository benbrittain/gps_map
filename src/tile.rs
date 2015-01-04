extern crate curl;

use sdl2::render::Renderer;
use sdl2::rect::Rect;
use sdl2_image::LoadSurface;
use self::curl::http;

#[derive(Show, Copy, Clone)]
enum Status {
    Empty,
    Loading,
    Loaded
}

#[derive(Show, Clone, Copy)]
pub struct Tile {
    lat: f32,
    long: f32,
    zoom: int,
    status: Status,
}

impl Tile {
    pub fn new(lat: f32, long: f32) -> Tile {
        Tile {
            status: Status::Empty,
            lat: lat,
            long: long,
            zoom: 15
        }
    }

    pub fn get_tile(&mut self) -> () {
        self.status = Status::Loading;
//        let (x, y, z) = self.convert_ll_to_osm();
//        let url = format!("http://b.basemaps.cartocdn.com/light_all/{}/{}/{}.png", z, x, y);
//        let resp = http::handle().get(url).exec().unwrap();
//        println!("got response!");
        self.status = Status::Loaded;
    }

    pub fn render(&self, renderer: &Renderer, x_offset: int, y_offset: int) -> () {
        println!("{}", self.lat);
        println!("{}", self.long);
        let x = x_offset as i32;
        let y = y_offset as i32;
        println!("{}, {}", x, y);
        let dest = Rect::new(x as i32, y as i32, 256, 256); //256 because thats the size OSM gives

//        match self.status {
//            Status::Empty => {
//                self.get_tile();
//                self.status = Status::Loaded;
//            }
//            Status::Loaded => {
//                println!("loaded tile!");
//            }
//        }

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

        let _ = renderer.copy(&texture, None, Some(dest));
    }
}

