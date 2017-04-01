extern crate fux_delaunay;
extern crate rand;

use fux_delaunay::types::*;
use fux_delaunay::c_api::*;

#[test]
fn test_300x300_truktura_500_json() {
    let data = match deserialize_data2("tests/data/300x300_truktura_500.json") {
        Ok(d) => d,
        Err(_) => panic!("failed to open file."),
    };

    let mut nodes: Vec<Point2> = Vec::new();
    for p in data.into_iter() {
        nodes.push(Point2::new(p.x, p.y));
    }

    let _ = Triangulation2::new(&nodes);
}
