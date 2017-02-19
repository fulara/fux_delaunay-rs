use libc;
use std::slice::from_raw_parts;

use types::*;

#[derive(Debug, PartialEq)]
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

#[no_mangle]
pub fn generate_triangulation(points: *mut CApiPoint2, point_count: libc::int32_t, triangulation_data: *mut CApiTriangulation) {
    println!("points size is: {}  size: {} ", points as usize, point_count);
    let points: &[CApiPoint2] = unsafe { from_raw_parts(points, point_count as usize) };

    let mut nodes: Vec<Point2> = Vec::new();
    for p in points.into_iter() {
        nodes.push(Point2::new(p.x, p.y));
    }

    let triangulation = Triangulation::new(&nodes);

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