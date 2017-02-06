extern crate fux_delaunay;

use fux_delaunay::io::abaqus_read::load_from_abaqus_format;

#[cfg(test)]
mod neighborhood_test {
    use fux_delaunay::io::abaqus_read::load_from_abaqus_format;
    use fux_delaunay::types::*;

    #[test]
    fn load_trivial_and_check_neighborhood() {
        let triangulation = load_from_abaqus_format("tests/data/trivial.inp").expect("load_trivial_and_check_neighborhood file loading fail");

        assert_eq!(560, triangulation.nodes().len());
        assert_eq!(814, triangulation.elements().len());

        for i in 0..triangulation.elements().len() {
            let e: &Triangle = &triangulation.elements()[i];

            let center = e.create_center_point(triangulation.nodes());

            //assert_eq!(T3Index(i), element_locators::find_element_containing_point(triangulation.elements(), triangulation.nodes(), &center));
        }
    }
}