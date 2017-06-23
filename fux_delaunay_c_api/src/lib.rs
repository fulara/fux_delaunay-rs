#![feature(libc)]

extern crate fux_delaunay;
extern crate libc;

pub use fux_delaunay::c_api::*;
use std::slice::from_raw_parts;

extern {
    fn test_c_api(triangulation: *mut CApiTriangulation2) -> libc::int32_t;
    fn test_c_api3(triangulation: *mut CApiTriangulation3) -> libc::int32_t;
}

#[test]
fn it_works2() {
    unsafe {
        let mut triangulation = CApiTriangulation2 { elements: 0 as *mut CApiElement3, element_count: 0 };

        test_c_api(&mut triangulation as *mut CApiTriangulation2);
        let elements: &[CApiElement3] = from_raw_parts(triangulation.elements, triangulation.element_count as usize);

        assert_eq!(2, elements.len());
        assert_eq!(CApiElement3 { v: [0, 3, 2] }, elements[0]);
        assert_eq!(CApiElement3 { v: [0, 1, 3] }, elements[1]);
    }
}

#[test]
fn it_works3() {
    unsafe {
        let mut triangulation = CApiTriangulation3 { elements: 0 as *mut CApiElement4, element_count: 0 };

        test_c_api3(&mut triangulation as *mut CApiTriangulation3);
        let elements: &[CApiElement4] = from_raw_parts(triangulation.elements, triangulation.element_count as usize);

        assert_eq!(6, elements.len());
        //assert_eq!(CApiElement3 { v: [0, 3, 2] }, elements[0]);
        //assert_eq!(CApiElement3 { v: [0, 1, 3] }, elements[1]);
    }
}
