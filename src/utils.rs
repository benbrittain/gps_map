use std::num::FloatMath;
use std::num::Float;
use std::f32::consts::PI;

pub fn y_to_lat(y: f32) -> f32 {
    (2.0* y.to_radians().exp2().atan() - PI/2.0).to_degrees()
}

pub fn lat_to_y(lat: f32) -> f32 {
    (PI/4.0 + lat.to_radians()/2.0).tan().ln().to_degrees()
}

pub fn ll_to_osm(lat: f32, long: f32, zoom: int) -> (int, int, int) {
    let n: f32 = 2.0.powi(zoom as i32);
    let lat_rad = lat.to_radians();

    let x_tile = (n * ((long + 180.0) / 360.0)).floor() as int;
    let y_tile = ((1.0 - (lat_rad.tan() + (1.0 / lat_rad.cos())).ln() / PI) / 2.0 * n).floor() as int;

    (x_tile, y_tile, zoom)
}
