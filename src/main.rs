#![feature(slicing_syntax)]

extern crate sdl2;
extern crate sdl2_image;

use sdl2_image::LoadSurface;

use std::io::{BufferedReader, File};
use std::path::Path;
use quadtree::{Point, Quadtree};
use map_view::{Mapview};
use tile::{Tile};

mod tile;
mod quadtree;
mod rendertree;
mod map_view;
mod utils;

pub fn main() {
    sdl2::init(sdl2::INIT_VIDEO);

    let height = 800;
    let width = 600;
    let zoom = 2;

    let window = match sdl2::video::Window::new("rust-sdl2 demo: Video",
                                                 sdl2::video::WindowPos::PosCentered,
                                                 sdl2::video::WindowPos::PosCentered,
                                                 height, width, sdl2::video::OPENGL) {
        Ok(window) => window,
        Err(err) => panic!(format!("failed to create window: {}", err))
    };

    let renderer = match sdl2::render::Renderer::from_window(window,
                                                             sdl2::render::RenderDriverIndex::Auto,
                                                             sdl2::render::ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err) => panic!(format!("failed to create renderer: {}", err))
    };

    let _ = renderer.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    let _ = renderer.clear();
    renderer.present();


    //---
//    println!("{}", utils::lat_to_y(-80.0));
//    println!("{}", utils::ll_to_osm(42.36, -71.09, 15));
    //---

    let mut map_view = Mapview::new(renderer, height, width, zoom);
    map_view.center_at(42.36, -71.09);


    let mut mouse_down = false;
//    'main : loop {
//        'event : loop {
//            match sdl2::event::poll_event() {
//                sdl2::event::Event::Quit(_) => break 'main,
//                sdl2::event::Event::KeyDown(_, _, key, _, _, _) => {
//                    if key == sdl2::keycode::KeyCode::Escape {
//                        break 'main
//                    }
//                },
//                sdl2::event::Event::MouseWheel(_, _, _, x, y) => {
//                    map_view.zoom()
//                },
//                sdl2::event::Event::MouseButtonUp(_, _, _, _, _, _) => {
//                    mouse_down = false;
//                },
//                sdl2::event::Event::MouseButtonDown(_, _, _, _, _, _) => {
//                    mouse_down = true;
//                },
//                sdl2::event::Event::MouseMotion(_, _, _, _, _, _, dx, dy) => {
//                    if mouse_down {
//                        map_view.move_by(dx, dy)
//                    }
//                },
//                sdl2::event::Event::None => break 'event,
//                _ => {}
//            }
//        }
          map_view.render();
//    }
    sdl2::quit();
}

//fn main() {
////    println!("initing quadtree");
//    let mut tree = Quadtree::new_with_bb(20, true);
//
//    let path = &Path::new("./data/gpsdata_simple.csv");
//    let file = File::open(path);
//    let mut reader = BufferedReader::new(file);
//
//    loop {
//        let line = reader.read_line();
//        match line {
//            Ok(string) => {
//                // TODO make this neater. trim space?
//                let data: Vec<&str>= string.split_str(", ").collect();
//                let lat: f32 = data[2].parse().unwrap();
//                let long: f32 = data[1].parse().unwrap();
//                let point = Point::new(lat, long);
//                tree.insert(point);
//            }
//            Err(_) => { break; }
//        }
//    }
//    //let points = tree.gather_data(Point::new(42.369705, -71.09388), 0.00003);
//    //println!("{}, {}", points, points.len());
//    map_view.render();
//}
