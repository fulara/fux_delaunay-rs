extern crate fux_delaunay;
extern crate rand;

use fux_delaunay::types::*;
use fux_delaunay::io::abaqus_write;
use rand::distributions::{IndependentSample, Range};
use rand::{SeedableRng, StdRng};

#[test]
fn trivial_tests_1() {
    let mut triangulation = Triangulation::new(&[Point2::new(0., 10.), Point2::new(10., 10.), Point2::new(0., 0.), Point2::new(10., 0.)]);

    triangulation.insert_node(&Point2::new(2., 2.));

    abaqus_write::write_to_abaqus_format("tests/tests_results/trivial_tests_1.inp", &triangulation);
}

#[test]
fn trivial_tests_2() {
    let mut triangulation = Triangulation::new(&[Point2::new(0., 10.), Point2::new(10., 10.), Point2::new(0., 0.), Point2::new(10., 0.)]);

    triangulation.insert_node(&Point2::new(0., 2.));
    triangulation.insert_node(&Point2::new(2., 2.));
    triangulation.insert_node(&Point2::new(8., 2.));
    triangulation.insert_node(&Point2::new(10., 2.));
    triangulation.insert_node(&Point2::new(0., 8.));
    triangulation.insert_node(&Point2::new(2., 8.));
    triangulation.insert_node(&Point2::new(8., 8.));
    triangulation.insert_node(&Point2::new(10., 8.));

    abaqus_write::write_to_abaqus_format("tests/tests_results/trivial_tests_2.inp", &triangulation);
}

#[test]
fn trivial_tests_3() {
    let mut triangulation = Triangulation::new(&[Point2::new(0., 10.), Point2::new(10., 10.), Point2::new(0., 0.), Point2::new(10., 0.)]);

    triangulation.insert_node(&Point2::new(4., 4.));
    triangulation.insert_node(&Point2::new(6., 4.));
    triangulation.insert_node(&Point2::new(4., 6.));
    triangulation.insert_node(&Point2::new(6., 6.));

    triangulation.insert_node(&Point2::new(3., 3.));
    triangulation.insert_node(&Point2::new(5., 3.));
    triangulation.insert_node(&Point2::new(8., 3.));

    triangulation.insert_node(&Point2::new(3., 5.));
    triangulation.insert_node(&Point2::new(5., 5.));
    triangulation.insert_node(&Point2::new(8., 5.));

    triangulation.insert_node(&Point2::new(3., 8.));
    triangulation.insert_node(&Point2::new(5., 8.));
    triangulation.insert_node(&Point2::new(8., 8.));

    abaqus_write::write_to_abaqus_format("tests/tests_results/trivial_tests_3.inp", &triangulation);
}


#[test]
fn trivial_tests_4() {
    let mut triangulation = Triangulation::new(&[Point2::new(0., 10.), Point2::new(10., 10.), Point2::new(0., 0.), Point2::new(10., 0.)]);

    for x in 1..2500 {
        //for y in 1 .. 100 {
        let y = x;
        triangulation.insert_node(&Point2::new(x as f64 * 0.002, y as f64 * 0.002));
        //println!("adding x{} y{}", x, y);
        //}
    }

    abaqus_write::write_to_abaqus_format("tests/tests_results/trivial_tests_4.inp", &triangulation);
}

#[test]
fn trivial_tests_5() {
    let mut triangulation = Triangulation::new(&[Point2::new(0., 10.), Point2::new(10., 10.), Point2::new(0., 0.), Point2::new(10., 0.)]);

    let seed: &[_] = &[1];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let between = Range::new(0., 10.);

    for _ in 0..2000 {
        let x = between.ind_sample(&mut rng);
        let y = between.ind_sample(&mut rng);
        triangulation.insert_node(&Point2::new(x, y));
    }

    abaqus_write::write_to_abaqus_format("tests/tests_results/trivial_tests_5.inp", &triangulation);
}