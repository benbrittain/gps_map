use std::fmt;
use tile::Tile;

#[derive(Copy, Clone)]
pub struct Point {
    x: f32,
    y: f32,
//  extra data
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point {
            x: x,
            y: y,
        }
    }
}

impl fmt::Show for Point{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}, {}>", self.x, self.y)
    }
}

#[derive(Show)]
pub struct BoundingBox {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32, 
    pub y1: f32
}
impl BoundingBox {
    pub fn new(x0: f32, y0: f32, x1: f32, y1: f32) -> BoundingBox {
        BoundingBox {
            x0: x0, y0: y0,
            x1: x1, y1: y1,
        }
    }
    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.x0 && point.x < self.x1 && point.y >= self.y0 && point.y < self.y1
    }
    pub fn contains_in_radius(&self, center_point: Point, radius: f32) -> bool {
        (center_point.x + radius >= self.x0 &&
         center_point.y + radius >= self.y0) ||
        (center_point.x - radius <= self.x1 &&
         center_point.y - radius <= self.y1)
    }
    pub fn contains_in_x_y(&self, center_point: Point, width: f32, height: f32) -> bool {
        (center_point.x + width >= self.x0 &&
         center_point.y + height >= self.y0) ||
        (center_point.x - width <= self.x1 &&
         center_point.y - height <= self.y1)
    }
}

#[derive(Show)]
pub struct Quadtree {
    points: Vec<Point>,
    bounds: BoundingBox,
    tile: Option<Tile>,
    ne: Option<Box<Quadtree>>,
    nw: Option<Box<Quadtree>>,
    se: Option<Box<Quadtree>>,
    sw: Option<Box<Quadtree>>,
    count: uint,
}

impl Quadtree {
    pub fn new(bb: BoundingBox, count: uint, tile_rendering: bool) -> Quadtree {
        let tile = if tile_rendering {
            let lat = (bb.x1 + bb.x0) / 2.0;
            let long = (bb.y1 + bb.y0) / 2.0;
            Some(Tile::new(lat, long))
        } else {
            None
        };

        Quadtree {
            points: vec![],
            bounds: bb,
            count: count,
            tile: tile,
            ne: None,
            nw: None,
            se: None,
            sw: None,
        }
    }

    pub fn new_with_bb(count: uint, tile_rendering: bool) -> Quadtree {
        let bounds = BoundingBox::new(-180.0, -180.0, 180.0, 180.0);
        Quadtree::new(bounds, count, tile_rendering)
    }

    pub fn insert(&mut self, point: Point) -> () {
        if self.bounds.contains(point) {
            if self.points.len() <= self.count {
                self.points.push(point);
                return
            }
            match self.nw {
                None => { self.subdivide() }
                Some(_) => {}
            }
            match self.nw {
                Some(ref mut child) => { child.insert(point) }
                None => {}
            }
            match self.ne {
                Some(ref mut child) => { child.insert(point) }
                None => {}
            }
            match self.sw {
                Some(ref mut child) => { child.insert(point) }
                None => {}
            }
            match self.se {
                Some(ref mut child) => { child.insert(point) }
                None => {}
            }
        }
    }

    pub fn subdivide(&mut self) -> () {
        let ref bb = self.bounds;
        let x_mid = (bb.x1 + bb.x0) / 2.0;
        let y_mid = (bb.y1 + bb.y0) / 2.0;

        let tr = match self.tile {
            Some(_) => true,
            None => false
        };

        let nw = BoundingBox::new(bb.x0, bb.y0, x_mid, y_mid);
        self.nw = Some(box Quadtree::new(nw, self.count, tr));

        let ne = BoundingBox::new(x_mid, bb.y0, bb.x1, y_mid);
        self.ne = Some(box Quadtree::new(ne, self.count, tr));

        let sw = BoundingBox::new(bb.x0, y_mid, x_mid, bb.y1);
        self.sw = Some(box Quadtree::new(sw, self.count, tr));

        let se = BoundingBox::new(x_mid, y_mid, bb.x1, bb.y1);
        self.se = Some(box Quadtree::new(se, self.count, tr));
    }

    pub fn gather_data(&self, center_point: Point, range: f32) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];
        for point in self.points.iter() {
            if center_point.x + range >= point.x &&
                center_point.x - range < point.x &&
                center_point.y + range >= point.y &&
                center_point.y - range < point.y {
                points.push(*point);
            }
        }

        if self.bounds.contains_in_radius(center_point, range) {
            match self.nw {
                None => {()}
                Some(ref child) => {
                    points.push_all(child.gather_data(center_point, range).as_slice())
                }
            }
            match self.ne {
                None => {()}
                Some(ref child) => {
                    points.push_all(child.gather_data(center_point, range).as_slice())
                }
            }
            match self.se {
                None => {()}
                Some(ref child) => {
                    points.push_all(child.gather_data(center_point, range).as_slice())
                }
            }
            match self.sw {
                None => {()}
                Some(ref child) => {
                    points.push_all(child.gather_data(center_point, range).as_slice())
                }
            }
        }
        points
    }
}

