extern crate fux_delaunay;

use fux_delaunay::types::*;
use fux_delaunay::io::abaqus_write;

#[test]
fn trivial_tests_1() {

    let mut triangulation = Triangulation::new(Point2::new(0.,10.), Point2::new(10.,10.), Point2::new(0.,0.), Point2::new(10.,0.));

    triangulation.insert_node(&Point2::new(2.,2.));

    abaqus_write::write_to_abaqus_format("tests/tests_results/trivial_tests_1.inp", &triangulation);
}

#[test]
fn trivial_tests_2() {

    let mut triangulation = Triangulation::new(Point2::new(0.,10.), Point2::new(10.,10.), Point2::new(0.,0.), Point2::new(10.,0.));

    triangulation.insert_node(&Point2::new(2.,2.));
    triangulation.insert_node(&Point2::new(8.,2.));
    triangulation.insert_node(&Point2::new(2.,8.));
    triangulation.insert_node(&Point2::new(8.,8.));

    abaqus_write::write_to_abaqus_format("tests/tests_results/trivial_tests_2.inp", &triangulation);
}

#[test]
fn trivial_tests_3() {

    let mut triangulation = Triangulation::new(Point2::new(0.,10.), Point2::new(10.,10.), Point2::new(0.,0.), Point2::new(10.,0.));

    triangulation.insert_node(&Point2::new(4.,4.));
    triangulation.insert_node(&Point2::new(6.,4.));
    triangulation.insert_node(&Point2::new(4.,6.));
    triangulation.insert_node(&Point2::new(6.,6.));

    triangulation.insert_node(&Point2::new(3.,3.));
    triangulation.insert_node(&Point2::new(5.,3.));
    triangulation.insert_node(&Point2::new(8.,3.));

    triangulation.insert_node(&Point2::new(3.,5.));
    triangulation.insert_node(&Point2::new(5.,5.));
    triangulation.insert_node(&Point2::new(8.,5.));

    triangulation.insert_node(&Point2::new(3.,8.));
    triangulation.insert_node(&Point2::new(5.,8.));
    triangulation.insert_node(&Point2::new(8.,8.));

    abaqus_write::write_to_abaqus_format("tests/tests_results/trivial_tests_3.inp", &triangulation);
}