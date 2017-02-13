use super::Triangulation;

use types::*;

pub fn insert_into_element(triangulation: &mut Triangulation, element_index: T3Index, new_node_index: N2Index) {}

#[cfg(test)]
mod tests {
    use ::types::*;
    use super::*;

    #[test]
    fn fail_() {
        let nodes: Vec<Point2> = vec!(Point2::new(2.5, 5.),
                                      Point2::new(3.5, 6.5),
                                      Point2::new(2., 6.5),
                                      Point2::new(3.5, 5.5),
                                      Point2::new(3.0, 7.5),
                                      Point2::new(1.5, 5.),
                                      Point2::new(2.5, 6.));

        let eles: Vec<Triangle> = vec!(Triangle::new(&nodes, N2Index(0), N2Index(2), N2Index(1)),
                                       Triangle::new(&nodes, N2Index(0), N2Index(1), N2Index(3)),
                                       Triangle::new(&nodes, N2Index(2), N2Index(4), N2Index(1)),
                                       Triangle::new(&nodes, N2Index(5), N2Index(2), N2Index(0)));

        let mut triangulation = Triangulation::new_from_prebuilt_triangulation(nodes, eles);


        assert_eq!(6, triangulation.elements().len());
        assert_eq!(Triangle::new_exact([N2Index(0),N2Index(6),N2Index(2)], [None, None,None]), triangulation.elements()[0]);

    }
}