wit_bindgen_rust::export!("h3index.wit");
struct H3index;

use geo_types::Coordinate;
use h3ron::{H3Cell, Index, };
use std::u64;


impl h3index::H3index for H3index {
    fn h3index(lat: f64, lon: f64, res: u8) -> String {
		return format!("{:x}", H3Cell::from_coordinate(Coordinate::from((lat, lon)), res).unwrap().h3index());
    }
}
