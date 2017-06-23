extern crate fux_delaunay;
extern crate rand;

use fux_delaunay::types::*;
use fux_delaunay::c_api::*;

#[test]
fn data_100x100x100() {
    let data = match deserialize_data3("tests/data/data_100X100X100.json") {
        Ok(d) => d,
        Err(_) => panic!("failed to open file."),
    };

    let mut nodes: Vec<Point3> = Vec::new();
    for (index, p) in data.into_iter().enumerate() {
        println!("index {:?}", index);
        nodes.push(Point3::new(p.x, p.y, p.z));
    }

    let _ = Triangulation3::new(&nodes);
}
