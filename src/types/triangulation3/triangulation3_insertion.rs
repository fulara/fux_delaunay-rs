use types::triangulation3::Triangulation3;
use types::Tetrahedron;
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

    let (original_elements_nodes, original_element_neighbors, original_elements_faces) = {
        let original_element: &Tetrahedron = &triangulation.elements()[element_index.0];
        (*original_element.nodes(),
         *original_element.neighbors(),
         original_element.faces_as_indices_tuples())
    };

    let cw1 = Tetrahedron::new(triangulation.nodes(),
                               original_elements_faces[1].0,
                               original_elements_faces[1].1,
                               original_elements_faces[1].2,
                               new_node_index);
    assert_eq!(*cw1.nodes(),
               [original_elements_faces[1].0,
                original_elements_faces[1].1,
                original_elements_faces[1].2,
                new_node_index]);
    let cw2 = Tetrahedron::new(triangulation.nodes(),
                               original_elements_faces[2].0,
                               original_elements_faces[2].1,
                               original_elements_faces[2].2,
                               new_node_index);
    assert_eq!(*cw2.nodes(),
               [original_elements_faces[2].0,
                original_elements_faces[2].1,
                original_elements_faces[2].2,
                new_node_index]);
    let cw3 = Tetrahedron::new(triangulation.nodes(),
                               original_elements_faces[3].0,
                               original_elements_faces[3].1,
                               original_elements_faces[3].2,
                               new_node_index);
    assert_eq!(*cw3.nodes(),
               [original_elements_faces[3].0,
                original_elements_faces[3].1,
                original_elements_faces[3].2,
                new_node_index]);

    update_neighborhood(triangulation,
                        original_element_neighbors[1],
                        original_elements_faces[1],
                        index_of_cw1);
    update_neighborhood(triangulation,
                        original_element_neighbors[2],
                        original_elements_faces[2],
                        index_of_cw2);

    update_neighborhood(triangulation,
                        original_element_neighbors[3],
                        original_elements_faces[3],
                        index_of_cw3);

    triangulation.elements_mut().push(cw1);
    triangulation.elements_mut().push(cw2);
    triangulation.elements_mut().push(cw3);

    set_neighbors(&mut triangulation.elements_mut()[index_of_bottom.0],
                  [original_element_neighbors[0],
                   Some(index_of_cw1),
                   Some(index_of_cw2),
                   Some(index_of_cw3)]);

    //override D with E in original element.
    triangulation.elements_mut()[index_of_bottom.0].set_node(3, new_node_index);

    set_neighbors(&mut triangulation.elements_mut()[index_of_cw1.0],
                  [original_element_neighbors[1],
                   Some(index_of_bottom),
                   Some(index_of_cw3),
                   Some(index_of_cw2)]);

    set_neighbors(&mut triangulation.elements_mut()[index_of_cw2.0],
                  [original_element_neighbors[2],
                   Some(index_of_cw3),
                   Some(index_of_bottom),
                   Some(index_of_cw2)]);

    set_neighbors(&mut triangulation.elements_mut()[index_of_cw3.0],
                  [original_element_neighbors[3],
                   Some(index_of_cw1),
                   Some(index_of_bottom),
                   Some(index_of_cw2)]);

    (index_of_bottom, index_of_cw1, index_of_cw2, index_of_cw3)
}

fn set_neighbors(element: &mut Tetrahedron, n: [Option<T4Index>; 4]) {
    for i in 0..4 {
        element.set_neighbor(i, n[i])
    }
}

fn update_neighborhood(triangulation: &mut Triangulation3,
                       for_index: Option<T4Index>,
                       face: (N3Index, N3Index, N3Index),
                       update_with: T4Index) {
    if let Some(updated_element_index) = for_index {
        let updated_element: &mut Tetrahedron = &mut triangulation.elements_mut()
                                                         [updated_element_index.0];
        updated_element.update_neighbor(face.0, face.1, face.2, Some(update_with));
    }
}

#[cfg(test)]
mod tests {
    use types::*;
    use algorithms3::element_locators::*;
    use super::*;
    use types::triangulation3_test_utils::get_example_initial_point_set;
    use types::triangulation3_initiation::create_initial_tetra_set;

    #[test]
    fn insert_into_single_element() {
        let nodes: Vec<Point3> = vec![Point3::new(0., 0., 0.),
                                      Point3::new(1., 1., 0.),
                                      Point3::new(2., 0., 0.),
                                      Point3::new(0.5, 0.5, 0.5),
                                      Point3::new(1., 0.3, 0.25)];

        let eles: Vec<Tetrahedron> =
            vec![Tetrahedron::new(&nodes, N3Index(0), N3Index(1), N3Index(2), N3Index(3))];

        let mut tr = Triangulation3::new_from_prebuilt_triangulation(nodes.clone(), eles);

        assert_eq!(LocationResult::InElement(T4Index(0)),
                   locate_element_containing(T4Index(0), tr.elements(), tr.nodes(), &nodes[4]));

        insert_into_element(&mut tr, T4Index(0), N3Index(4));

        assert_eq!(4, tr.elements().len());
        assert_eq!(Tetrahedron::new_exact([N3Index(0), N3Index(1), N3Index(2), N3Index(4)],
                                          [None,
                                           Some(T4Index(1)),
                                           Some(T4Index(2)),
                                           Some(T4Index(3))]),
                   tr.elements()[0]);
        assert_eq!(Tetrahedron::new_exact([N3Index(1), N3Index(0), N3Index(3), N3Index(4)],
                                          [None,
                                           Some(T4Index(0)),
                                           Some(T4Index(3)),
                                           Some(T4Index(2))]),
                   tr.elements()[1]);
        assert_eq!(Tetrahedron::new_exact([N3Index(3), N3Index(2), N3Index(1), N3Index(4)],
                                          [None,
                                           Some(T4Index(3)),
                                           Some(T4Index(0)),
                                           Some(T4Index(2))]),
                   tr.elements()[2]);
        assert_eq!(Tetrahedron::new_exact([N3Index(3), N3Index(0), N3Index(2), N3Index(4)],
                                          [None,
                                           Some(T4Index(1)),
                                           Some(T4Index(0)),
                                           Some(T4Index(2))]),
                   tr.elements()[3]);
    }

    #[test]
    fn testing_using_example_initial_point_set() {
        let nodes: Vec<Point3> = get_example_initial_point_set();
        let eles: Vec<Tetrahedron> = create_initial_tetra_set(&[0, 1, 2, 3, 4, 5, 6, 7], &nodes);

        for index in 0..eles.len() {
            let element_to_which_insert = eles[index].clone();
            let center = element_to_which_insert.create_center_point(&nodes);

            let mut nodes = nodes.clone();
            let mut eles = eles.clone();

            nodes.push(center);

            let mut tr = Triangulation3::new_from_prebuilt_triangulation(nodes.clone(),
                                                                         eles.clone());

            insert_into_element(&mut tr, T4Index(index), N3Index(nodes.len() - 1));

        }



    }
}
