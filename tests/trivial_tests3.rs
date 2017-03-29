extern crate fux_delaunay;

use fux_delaunay::types::*;
use fux_delaunay::io::abaqus_3d_write::*;
use fux_delaunay::types::triangulation3_test_utils;

#[test]
fn trivial_test3_1() {
    let nodes = triangulation3_test_utils::get_example_initial_point_set();
    let eles = triangulation3_initiation::create_initial_tetra_set(&nodes);

    let mut tr = Triangulation3::new_from_prebuilt_triangulation(nodes.clone(), eles.clone());

    for i in 0..5 {
        let to_insert = tr.elements()[i].create_center_point(tr.nodes());
        tr.insert_node(&to_insert);
    }

    write_3d_to_abaqus_format("tests/tests_results3/trivial_tests3_1.inp", &tr);
}
