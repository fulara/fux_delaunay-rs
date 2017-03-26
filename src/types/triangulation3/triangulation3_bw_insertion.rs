use types::Triangulation3;
use types::T4Index;
use types::N3Index;
use types::Tetrahedron;
use types::Point3;
use math::SphereSide;
use algorithms3::sort_3::sort_3;

use std::collections::HashMap;
use std::collections::HashSet;

pub fn insert_into_element(triangulation: &mut Triangulation3,
                           element_index: T4Index,
                           new_node_index: N3Index) {
    //let elements_to_remove;
}

fn find(tr: &Triangulation3, starting_element: T4Index, node: &Point3) -> Vec<T4Index> {
    assert!(tr.elements()[starting_element.0].is_point_in_circumsphere(node, tr.nodes()));

    let mut checked_elements = HashSet::new();
    let mut elements_to_check = Vec::new();
    let mut elements_containing_point_in_circum = Vec::new();

    checked_elements.insert(starting_element);
    elements_to_check.push(starting_element);
    elements_containing_point_in_circum.push(starting_element);

    loop {
        let ele_index = elements_to_check.pop().expect("Failed to pop elements_to_check");
        let ele: &Tetrahedron = &tr.elements()[ele_index.0];
        for n in ele.neighbors().iter() {
            if let Some(n_index) = *n {
                if checked_elements.contains(&n_index) {
                    continue;
                }

                checked_elements.insert(n_index);

                let neighbor: &Tetrahedron = &tr.elements()[n_index.0];

                if neighbor.is_point_in_circumsphere(node, tr.nodes()) {
                    elements_to_check.push(n_index);
                    elements_containing_point_in_circum.push(n_index);
                }
            }
        }


        if elements_to_check.len() == 0 {
            break;
        }
    }

    elements_containing_point_in_circum
}

fn select_faces_which_exist_only_once(tr: &Triangulation3,
                                      indices: &[T4Index])
                                      -> Vec<(N3Index, N3Index, N3Index)> {
    let mut face_counter = HashMap::new();

    for index in indices {
        let tetra: &Tetrahedron = &tr.elements()[index.0];

        for face in tetra.faces_as_indices_tuples().iter() {
            *face_counter.entry(sort_3(face.0, face.1, face.2)).or_insert(0) += 1;
        }
    }

    println!("face counter: {:?}", face_counter);

    face_counter.iter()
        .filter(|&(k, v)| *v == 1)
        .map(|(k, v)| *k)
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod bw_insertion {
    use super::*;

    use super::super::triangulation3_test_utils::get_example_initial_point_set;
    use super::super::triangulation3_initiation::create_initial_tetra_set;

    use types::*;
    #[test]
    fn testing_find_using_example_set() {
        let example_set = get_example_initial_point_set();
        let example_tr = create_initial_tetra_set(&example_set);
        let tr = Triangulation3::new_from_prebuilt_triangulation(example_set.clone(),
                                                                 example_tr.clone());

        for (index, tetra) in example_tr.iter().enumerate() {
            let tetra: &Tetrahedron = tetra;
            let center = tetra.create_center_point(&example_set);
            assert_eq!(5, find(&tr, T4Index(index), &center).len());
        }
    }

    #[test]
    fn testing_find_using_special_cases() {
        //the tests uses unreal scenario where 2 tetras are within each other - easier to test.
        let nodes = vec![Point3::new(0., 0., 0.),
                         Point3::new(1., 0., 0.),
                         Point3::new(0., 1., 0.),
                         Point3::new(0.3, 0.3, 2.),
                         Point3::new(0.3, 0.3, 1.)];

        let eles = vec![Tetrahedron::new(&nodes, N3Index(0), N3Index(1), N3Index(2), N3Index(3)),
                        Tetrahedron::new(&nodes, N3Index(0), N3Index(1), N3Index(2), N3Index(4))];

        let tr = Triangulation3::new_from_prebuilt_triangulation(nodes.clone(), eles.clone());

        //first point which belongs inside two tetras
        assert_eq!(2, find(&tr, T4Index(0), &Point3::new(0.3, 0.3, 0.5)).len());

        //this point only lives in the bigger tetra.
        assert_eq!(1, find(&tr, T4Index(0), &Point3::new(0.3, 0.3, 1.5)).len());
    }

    #[test]
    fn testing_face_uniquification_using_example_set() {
        let example_set = get_example_initial_point_set();
        let example_tr = create_initial_tetra_set(&example_set);
        let tr = Triangulation3::new_from_prebuilt_triangulation(example_set.clone(),
                                                                 example_tr.clone());

        let border_faces = select_faces_which_exist_only_once(&tr,
                                                              &[T4Index(0), T4Index(1),
                                                                T4Index(2), T4Index(3),
                                                                T4Index(4)]);

        //cube has 6 sides, on each side 2 faces.
        assert_eq!(12, border_faces.len());
    }
}
