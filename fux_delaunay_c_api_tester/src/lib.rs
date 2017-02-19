#![feature(libc)]

extern crate fux_delaunay;
extern crate libc;

pub use fux_delaunay::c_api::*;
use std::slice::from_raw_parts;

extern {
    fn test_c_api(triangulation: *mut CApiTriangulation) -> libc::int32_t;
}

#[test]
fn it_works() {
    unsafe {
        let mut triangulation = CApiTriangulation { elements: 0 as *mut CApiElement3, element_count: 0 };

        test_c_api(&mut triangulation as *mut CApiTriangulation);
        let elements: &[CApiElement3] = unsafe { from_raw_parts(triangulation.elements, triangulation.element_count as usize) };

        assert_eq!(2, elements.len());
        assert_eq!(CApiElement3 { v: [0, 3, 2] }, elements[0]);
        assert_eq!(CApiElement3 { v: [0, 1, 3] }, elements[1]);
    }
}
