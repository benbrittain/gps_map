use sdl2::render::Renderer;
use tile::Tile;

pub struct Mapview {
    renderer: Renderer,
    width: int, height: int,
    x_offset: int, y_offset: int,
    zoom: int,
    //tiles: Quadtree<Tile>
}

impl Mapview {
    pub fn new(render: Renderer, height: int, width: int, zoom: int) -> Mapview {
        Mapview {
            renderer: render,
            height: height,
            width: width,
            zoom: zoom,
            x_offset: 0,
            y_offset: 0
        }
    }

    pub fn zoom(&self) -> () {
        println!("zooming!");
    }

    pub fn render(&self) -> () {
        let t = Tile::new();
        let t1 = Tile::new();
        let t2 = Tile::new();
        let mut tiles = vec![];
        tiles.push(t);
        tiles.push(t1);
        tiles.push(t2);

        for tile in tiles.iter() {
            tile.render(&self.renderer)
        }
        self.renderer.present()


    }
}
