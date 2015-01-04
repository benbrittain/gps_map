use std::fmt;
use tile::Tile;
use quadtree::{Point, BoundingBox};

#[derive(Show)]
pub struct Rendertree {
    bounds: BoundingBox,
    tile: Tile,
    depth: int,
    ne: Option<Box<Rendertree>>,
    nw: Option<Box<Rendertree>>,
    se: Option<Box<Rendertree>>,
    sw: Option<Box<Rendertree>>,
}

impl Rendertree {
    pub fn new(bb: BoundingBox, depth: int) -> Rendertree {
        let lat = (bb.x1 + bb.x0) / 2.0;
        let long = (bb.y1 + bb.y0) / 2.0;
        let tile = Tile::new(lat, long);
        Rendertree {
            bounds: bb,
            tile: tile,
            depth: depth,
            ne: None,
            nw: None,
            se: None,
            sw: None,
        }
    }

    pub fn new_root() -> Rendertree {
        let bounds = BoundingBox::new(-180.0, -90.0, 180.0, 90.0);
        Rendertree::new(bounds, 0)
    }

    pub fn subdivide(&mut self) -> () {
        let ref bb = self.bounds;
        let x_mid = (bb.x1 + bb.x0) / 2.0;
        let y_mid = (bb.y1 + bb.y0) / 2.0;
        println!("subdividing: {}, {}", x_mid, y_mid);
        let depth = self.depth + 1;

        match self.nw {
            Some(_) => {return ();}
            None => {}
        }

        let nw = BoundingBox::new(bb.x0, bb.y0, x_mid, y_mid);
        self.nw = Some(box Rendertree::new(nw, depth));

        let ne = BoundingBox::new(x_mid, bb.y0, bb.x1, y_mid);
        self.ne = Some(box Rendertree::new(ne, depth));

        let sw = BoundingBox::new(bb.x0, y_mid, x_mid, bb.y1);
        self.sw = Some(box Rendertree::new(sw, depth));

        let se = BoundingBox::new(x_mid, y_mid, bb.x1, bb.y1);
        self.se = Some(box Rendertree::new(se, depth));
    }

    pub fn get_tiles(&mut self, x: f32, y: f32, width: int, height: int, zoom: int) -> Vec<Tile> {
        let mut tiles: Vec<Tile> = vec![];

        let point = Point::new(x, y);
        tiles.push(self.tile);
        println!("depth: {}, zl: {}", self.depth, zoom);

        if (self.depth < zoom) {
            self.subdivide();
            match self.nw {
                Some(ref mut child) => {
                    tiles.push_all(child.get_tiles(x, y, zoom, width, height).as_slice())
                }
                None => {}
            }
        }


//        }
//        match self.ne {
//            Some(ref child) => {
//                tiles.push_all(child.get_tiles().as_slice())
//            }
//            None => {}
//        }
//        match self.sw {
//            Some(ref child) => {
//                tiles.push_all(child.get_tiles().as_slice())
//            }
//            None => {}
//        }
//        match self.se {
//            Some(ref child) => {
//                tiles.push_all(child.get_tiles().as_slice())
//            }
//            None => {}
//        }
        tiles
    }

}

