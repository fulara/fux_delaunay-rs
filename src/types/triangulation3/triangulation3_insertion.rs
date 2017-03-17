use types::triangulation3::Triangulation3;
use types::N3Index;
use types::T4Index;

pub fn insert_into_element(triangulation: &mut Triangulation3,
                           element_index: T4Index,
                           new_node_index: N3Index)
                           -> (T4Index, T4Index, T4Index, T4Index) {
    let index_of_bottom = element_index;
    let index_of_cw1 = T4Index(triangulation.elements().len());
    let index_of_cw2 = T4Index(triangulation.elements().len() + 1);
    let index_of_cw3 = T4Index(triangulation.elements().len() + 2);


    (index_of_bottom, index_of_cw1, index_of_cw2, index_of_cw3)
}

#[cfg(test)]
mod tests {
    use types::*;
    use super::*;

    #[test]
    fn insert_into_single_element() {
        let nodes: Vec<Point3> = vec![Point3::new(0., 0., 0.),
                                      Point3::new(1., 1., 0.),
                                      Point3::new(2., 0., 0.),
                                      Point3::new(3.5, 5.5),
                                      Point3::new(3.0, 7.5),
                                      Point3::new(1.5, 5.),
                                      Point3::new(2.5, 6.)];

        let eles: Vec<Triangle> = vec![Triangle::new(&nodes, N2Index(0), N2Index(2), N2Index(1)),
                                       Triangle::new(&nodes, N2Index(0), N2Index(1), N2Index(3)),
                                       Triangle::new(&nodes, N2Index(2), N2Index(4), N2Index(1)),
                                       Triangle::new(&nodes, N2Index(5), N2Index(2), N2Index(0))];

        let mut triangulation = Triangulation2::new_from_prebuilt_triangulation(nodes, eles);

        insert_into_element(&mut triangulation, T3Index(0), N2Index(6));

        assert_eq!(6, triangulation.elements().len());
        assert_eq!(Triangle::new_exact([N2Index(0), N2Index(2), N2Index(6)],
                                       [Some(T3Index(3)), Some(T3Index(5)), Some(T3Index(0))]),
                   triangulation.elements()[4]);
        assert_eq!(Triangle::new_exact([N2Index(2), N2Index(1), N2Index(6)],
                                       [Some(T3Index(2)), Some(T3Index(0)), Some(T3Index(4))]),
                   triangulation.elements()[5]);
        assert_eq!(Triangle::new_exact([N2Index(1), N2Index(0), N2Index(6)],
                                       [Some(T3Index(1)), Some(T3Index(4)), Some(T3Index(5))]),
                   triangulation.elements()[0]);
    }
}
