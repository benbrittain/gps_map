use std::fmt;

#[derive(Copy, Clone)]
pub struct Point {
    x: f32,
    y: f32,
//    extra data
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
struct BoundingBox {
    x0: f32, y0: f32,
    x1: f32, y1: f32
}
impl BoundingBox {
    pub fn new(x0: f32, y0: f32, x1: f32, y1: f32) -> BoundingBox {
        BoundingBox {
            x0: x0, y0: y0,
            x1: x1, y1: y1,
        }
    }
    fn contains(&self, point: Point) -> bool {
        point.x >= self.x0 && point.x < self.x1 && point.y >= self.y0 && point.y < self.y1
    }
    fn contains_in_radius(&self, center_point: Point, radius: f32) -> bool {
        (center_point.x + radius >= self.x0 &&
         center_point.y + radius >= self.y0) ||
        (center_point.x - radius <= self.x1 &&
         center_point.y - radius <= self.y1)
    }
}

#[derive(Show)]
pub struct Quadtree {
    points: Vec<Point>,
    bounds: BoundingBox,
    ne: Option<Box<Quadtree>>,
    nw: Option<Box<Quadtree>>,
    se: Option<Box<Quadtree>>,
    sw: Option<Box<Quadtree>>,
}

//impl fmt::Show for Quadtree {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        let mut string = "".to_string();
//        string = string + "points: " + self.points.to_string().as_slice();
//        match self.ne {
//            Some(ref child) => { string = string + "\n\tne: \n" }//+ child.to_string().as_slice()}
//            None => {()},
//        }
//        match self.nw {
//            Some(ref child) => { string = string + "nw: \n" + child.to_string().as_slice()}
//            None => {()},
//        }
//        //string = string + "points: " + self.points.to_string().as_slice();
//        write!(f, "{}", string)
////        write!(f, "<{}, {}>", self.x, self.y)
//    }
//}

impl Quadtree {
    pub fn new_with_bb() -> Quadtree {
        Quadtree {
            points: vec![],
            bounds: BoundingBox::new(-180.0, -180.0, 180.0, 180.0),
            ne: None,
            nw: None,
            se: None,
            sw: None,
        }
    }

    pub fn new(bb: BoundingBox) -> Quadtree {
        Quadtree {
            points: vec![],
            bounds: bb,
            ne: None,
            nw: None,
            se: None,
            sw: None,
        }
    }

    pub fn insert(&mut self, point: Point) -> () {
        if self.bounds.contains(point) {
            if self.points.len() <= 50 {
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

        let nw = BoundingBox::new(bb.x0, bb.y0, x_mid, y_mid);
        self.nw = Some(box Quadtree::new(nw));

        let ne = BoundingBox::new(x_mid, bb.y0, bb.x1, y_mid);
        self.ne = Some(box Quadtree::new(ne));

        let sw = BoundingBox::new(bb.x0, y_mid, x_mid, bb.y1);
        self.sw = Some(box Quadtree::new(sw));

        let se = BoundingBox::new(x_mid, y_mid, bb.x1, bb.y1);
        self.se = Some(box Quadtree::new(se));
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

