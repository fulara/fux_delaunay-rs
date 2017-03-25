use types::Triangulation3;
use types::T4Index;
use types::N3Index;
use types::Tetrahedron;
use types::Point3;
use math::SphereSide;

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
            assert_eq!(find(&tr, T4Index(index), &center).len(), 5);
        }
    }
}
