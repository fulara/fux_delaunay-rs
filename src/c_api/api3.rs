use libc;
use std::slice::from_raw_parts;

use rustc_serialize::json;

use types::*;
use std::fs::File;
use std::io::{Write, Read};

#[derive(Clone, Debug, PartialEq, RustcEncodable, RustcDecodable)]
#[repr(C)]
pub struct CApiPoint3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct CApiPoint3Data {
    pub nodes: *mut CApiPoint3,
    pub nodes_count: libc::size_t,
}

#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct CApiElement4 {
    pub v: [usize; 4],
}

#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct CApiTriangulation3 {
    pub elements: *mut CApiElement4,
    pub element_count: usize,
}

fn serialize_and_save_data3(nodes: &Vec<CApiPoint3>) {
    println!("serializing and saving data.");
    let mut data: File = File::create("input_points_from_c.json").unwrap();
    let _ = data.write(json::encode(&nodes).unwrap().as_bytes());
    println!("serialized!");
}

pub fn deserialize_data3(path_to_data: &str) -> ::std::io::Result<Vec<CApiPoint3>> {
    let mut file: File = try!(File::open(path_to_data));

    let mut buffor: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut buffor);

    Ok(json::decode(&String::from_utf8(buffor).unwrap()).unwrap())
}

/*
#[no_mangle]
pub fn generate_triangulation3(points: *mut CApiPoint3,
                               point_count: libc::int32_t,
                               triangulation_data: *mut CApiTriangulation3) {
    let points: &[CApiPoint3] = unsafe { from_raw_parts(points, point_count as usize) };
    serialize_and_save_data3(&Vec::from(points));

    let mut nodes: Vec<Point3> = Vec::new();
    for p in points.into_iter() {
        nodes.push(Point3::new(p.x, p.y, p.z));
    }

    let triangulation = Triangulation3::new(&nodes);

    let mut elements: Vec<CApiElement4> = Vec::new();

    for e in triangulation.elements().into_iter() {
        elements.push(CApiElement4 { v: [e.index_a().0, e.index_b().0, e.index_c().0,e.index_d().0] })
    }

    let element_count = elements.len();
    let elements_slice: Box<[CApiElement4]> = elements.into_boxed_slice();
    let raw_slice = Box::into_raw(elements_slice);

    unsafe {
        (*triangulation_data).elements = raw_slice as *mut CApiElement4;
        (*triangulation_data).element_count = element_count;
    }
} */