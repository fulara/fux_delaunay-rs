extern crate fux_delaunay;

use fux_delaunay::types::*;
use fux_delaunay::io::abaqus_3d_write::*;
use fux_delaunay::types::triangulation3_test_utils;

#[test]
fn trivial_test3_1() {
    let nodes = triangulation3_test_utils::get_example_initial_point_set();
    let eles = triangulation3_initiation::create_initial_tetra_set(&[0, 1, 2, 3, 4, 5, 6, 7],
                                                                   &nodes);

    let mut tr = Triangulation3::new_from_prebuilt_triangulation(nodes.clone(), eles.clone());

    for i in 0..5 {
        let to_insert = tr.elements()[i].create_center_point(tr.nodes());
        tr.insert_node(&to_insert);
    }

    write_3d_to_abaqus_format("tests/tests_results3/trivial_tests3_1.inp", &tr);
}

#[test]
fn trivial_test3_2() {
    let mut nodes = Vec::from(triangulation3_test_utils::get_example_initial_point_set());

    let tr = Triangulation3::new(&nodes);
    write_3d_to_abaqus_format("tests/tests_results3/trivial_tests3_2.inp", &tr);
}

#[test]
fn trivial_test3_3() {
    println!("before tr.");
    let mut nodes: Vec<_> = vec![Point3::new(0., 0., 0.),
                                 Point3::new(0., 1., 0.),
                                 Point3::new(1., 1., 0.),
                                 Point3::new(1., 0., 0.),
                                 Point3::new(0., 0., 1.),
                                 Point3::new(0., 1., 1.),
                                 Point3::new(1., 1., 1.),
                                 Point3::new(1., 0., 1.)];

    println!("before tr.");
    nodes.push(Point3::new(0.4, 0., 0.5));

    println!("before tr.");
    let tr = Triangulation3::new(&nodes);
    write_3d_to_abaqus_format("tests/tests_results3/trivial_tests3_3.inp", &tr);
}
