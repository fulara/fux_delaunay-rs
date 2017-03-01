use libc;
use std::slice::from_raw_parts;

use rustc_serialize::json;

use types::*;
use std::fs::File;
use std::io::{Write, Read};

#[derive(Clone, Debug, PartialEq, RustcEncodable, RustcDecodable)]
#[repr(C)]
pub struct CApiPoint2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct CApiPoint2Data {
    pub nodes: *mut CApiPoint2,
    pub nodes_count: libc::size_t,
}

#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct CApiElement3 {
    pub v: [usize; 3],
}

#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct CApiTriangulation {
    pub elements: *mut CApiElement3,
    pub element_count: usize,
}

fn serialize_and_save_data(nodes: &Vec<CApiPoint2>) {
    println!("serializing and saving data.");
    let mut data: File = File::create("input_points_from_c.json").unwrap();
    let _ = data.write(json::encode(&nodes).unwrap().as_bytes());
    println!("serialized!");
}

pub fn deserialize_data(path_to_data: &str) -> ::std::io::Result<Vec<CApiPoint2>> {
    let mut file: File = try!(File::open(path_to_data));

    let mut buffor: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut buffor);

    Ok(json::decode(&String::from_utf8(buffor).unwrap()).unwrap())
}

#[no_mangle]
pub fn generate_triangulation(points: *mut CApiPoint2, point_count: libc::int32_t, triangulation_data: *mut CApiTriangulation) {
    let points: &[CApiPoint2] = unsafe { from_raw_parts(points, point_count as usize) };
    serialize_and_save_data(&Vec::from(points));

    let mut nodes: Vec<Point2> = Vec::new();
    for p in points.into_iter() {
        nodes.push(Point2::new(p.x, p.y));
    }

    let triangulation = Triangulation2::new(&nodes);

    let mut elements: Vec<CApiElement3> = Vec::new();

    for e in triangulation.elements().into_iter() {
        elements.push(CApiElement3 {
            v: [e.index_a().0, e.index_b().0, e.index_c().0],
        })
    }

    let element_count = elements.len();
    let elements_slice: Box<[CApiElement3]> = elements.into_boxed_slice();
    let raw_slice = Box::into_raw(elements_slice);

    unsafe {
        (*triangulation_data).elements = raw_slice as *mut CApiElement3;
        (*triangulation_data).element_count = element_count;
    }
}