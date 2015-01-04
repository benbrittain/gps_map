use sdl2::render::Renderer;
use tile::Tile;
use rendertree::Rendertree;
use utils;

pub struct Mapview {
    renderer: Renderer,
    width: int, height: int,
    x_offset: int, y_offset: int,
    zoom: int,
    tiles: Rendertree
}

impl Mapview {
    pub fn new(render: Renderer, height: int, width: int, zoom: int) -> Mapview {
        Mapview {
            renderer: render,
            height: height,
            width: width,
            zoom: zoom,
            x_offset: 0,
            y_offset: 0,
            tiles: Rendertree::new_root()
        }
    }

    pub fn zoom(&self) -> () {
        println!("zooming!");
    }

    pub fn move_by(&mut self, dx: int, dy: int) -> () {
        self.x_offset = self.x_offset + dx;
        self.y_offset = self.y_offset + dy;
    }

    pub fn render(&mut self) -> () {
        let tiles = self.tiles.get_tiles(0.0, 0.0, self.width, self.height, self.zoom);

        for tile in tiles.iter() {
//            println!("{}", tile);
            tile.render(&self.renderer, self.x_offset, self.y_offset)
        }
        self.renderer.present()
    }

    pub fn center_at(&mut self, lat: f32, long: f32) -> () {
        let (x, y, z) = utils::ll_to_osm(lat, long, self.zoom);
        println!("aaa {}, {}", x, y);
        self.x_offset = x  - self.width / 2;
        self.y_offset = y  - self.height / 2;
        println!("aaa {}, {}" ,self.x_offset, self.y_offset);
    }
}
