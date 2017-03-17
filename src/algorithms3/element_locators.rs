use types::T4Index;

use types::Tetrahedron;
use types::Point3;

use math;

#[derive(Debug, Eq, PartialEq)]
pub enum LocationResult {
    InElement(T4Index),
    OnFaces(T4Index, usize, usize),
    OnFace(T4Index, usize),
}


#[inline]
pub fn locate_element_containing(start_lookup_at: T4Index,
                                 elements: &[Tetrahedron],
                                 nodes: &[Point3],
                                 p: &Point3)
                                 -> LocationResult {
    let mut ele_index = start_lookup_at;

    loop {
        let ele: &Tetrahedron = &elements[ele_index.0];

        let mut current_face = 0;
        let mut on_face_found: Option<usize> = None;
        let mut on_faces_found: Option<(usize, usize)> = None;

        loop {
            if current_face == 4 {
                break;
            }

            let edge = ele.faces_as_points_tuples(nodes)[current_face];

            match math::side_of_plane(edge.0, edge.1, edge.2, p) {
                math::SideOfPlane::Left => {
                    assert!(ele.get_neighbor_from_index(current_face).is_some());

                    ele_index = ele.get_neighbor_from_index(current_face).unwrap();
                    break;
                }
                math::SideOfPlane::OnPlane => {
                    if on_faces_found.is_some() {
                        panic!("found node on more than 2 planes.");
                    }
                    if let Some(on_face_found) = on_face_found {
                        on_faces_found = Some((on_face_found, current_face));
                    } else {
                        on_face_found = Some(current_face)
                    }
                }
                math::SideOfPlane::Right => (),
            }
            current_face += 1;
        }

        if current_face == 4 {
            if let Some((face1, face2)) = on_faces_found {
                return LocationResult::OnFaces(ele_index,
                                               on_faces_found.unwrap().0,
                                               on_faces_found.unwrap().1);
            }

            if let Some(on_edge_found) = on_face_found {
                return LocationResult::OnFace(ele_index, on_edge_found);
            }

            return LocationResult::InElement(ele_index);
        }
    }
}


#[cfg(test)]
mod tests {
    use types::Point3;
    use types::Tetrahedron;
    use types::N3Index;
    use types::T4Index;
    use types::Triangulation3;
    use types::triangulation3_initiation::create_initial_tetra_set;
    use types::triangulation3_test_utils::get_example_initial_point_set;
    use std::collections::BTreeSet;
    use super::*;

    #[test]
    fn locator_single_element() {
        let pts = vec![Point3::new(0.0, 0.0, 0.0),
                       Point3::new(1.0, 0.0, 0.0),
                       Point3::new(0.0, 1.0, 0.0),
                       Point3::new(0.0, 0.0, 1.0)];

        let t0 = Tetrahedron::new(&pts, N3Index(0), N3Index(1), N3Index(2), N3Index(3));
        let eles = vec![t0.clone()];

        let point_inside = t0.create_center_point(&pts); // Point3::new(0.1, 0.1, 0.1);

        assert_eq!(LocationResult::InElement(T4Index(0)),
                   locate_element_containing(T4Index(0), &eles, &pts, &point_inside));
    }

    #[test]
    fn finding_element_test() {
        let pts = vec![Point3::new(0.0, 0.0, 0.0),
                       Point3::new(1.0, 0.0, 0.0),
                       Point3::new(0.0, 1.0, 0.0),
                       Point3::new(0.0, 0.0, 1.0),
                       Point3::new(0.0, 0.0, -1.0)];

        let t0 = Tetrahedron::new(&pts, N3Index(0), N3Index(1), N3Index(2), N3Index(3));
        let t1 = Tetrahedron::new(&pts, N3Index(0), N3Index(1), N3Index(2), N3Index(4));

        let eles = vec![t0.clone(), t1.clone()];

        let triangulation = Triangulation3::new_from_prebuilt_triangulation(pts.clone(), eles);

        let center0 = t0.create_center_point(&pts);
        let center1 = t1.create_center_point(&pts);

        assert_eq!(LocationResult::InElement(T4Index(0)),
                   locate_element_containing(T4Index(0),
                                             triangulation.elements(),
                                             triangulation.nodes(),
                                             &center0));
        assert_eq!(LocationResult::InElement(T4Index(1)),
                   locate_element_containing(T4Index(0),
                                             triangulation.elements(),
                                             triangulation.nodes(),
                                             &center1));

        assert_eq!(LocationResult::InElement(T4Index(0)),
                   locate_element_containing(T4Index(1),
                                             triangulation.elements(),
                                             triangulation.nodes(),
                                             &center0));
        assert_eq!(LocationResult::InElement(T4Index(1)),
                   locate_element_containing(T4Index(1),
                                             triangulation.elements(),
                                             triangulation.nodes(),
                                             &center1));

        perform_tests(&triangulation);
    }

    #[test]
    fn finding_element_using_initial_triangulation() {
        let nodes = get_example_initial_point_set();
        let elements = create_initial_tetra_set(&nodes);

        let triangulation = Triangulation3::new_from_prebuilt_triangulation(nodes, elements);

        perform_tests(&triangulation);
    }

    fn perform_tests(triangulation: &Triangulation3) {
        for elem_index in 0..triangulation.elements().len() {
            for face_index in 0..
                              triangulation.elements()[elem_index]
                                  .faces_as_points_tuples(triangulation.nodes())
                                  .len() {
                let e = &triangulation.elements()[elem_index];
                let face = e.faces_as_points_tuples(triangulation.nodes())[face_index];
                let x = (face.0.x + face.1.x + face.2.x) / 3.;
                let y = (face.0.y + face.1.y + face.2.y) / 3.;
                let z = (face.0.z + face.1.z + face.2.z) / 3.;

                let face_center = Point3::new(x, y, z);

                //have to start location at this elem index otherwise will find at the neighbors.
                assert_eq!(LocationResult::OnFace(T4Index(elem_index), face_index),
                           locate_element_containing(T4Index(elem_index),
                                                     triangulation.elements(),
                                                     triangulation.nodes(),
                                                     &face_center));

                //now we need two faces. so just iterate again to get the 2nd.
                for face_index_second in 0..face_index {
                    let face_2 = e.faces_as_points_tuples(triangulation.nodes())[face_index_second];

                    let common_nodes = find_common_nodes(e.faces_as_indices_tuples()[face_index],
                                                         e.faces_as_indices_tuples()
                                                             [face_index_second]);
                    let x = (triangulation.nodes()[(common_nodes.0).0].x +
                             triangulation.nodes()[(common_nodes.1).0].x) /
                            2.;
                    let y = (triangulation.nodes()[(common_nodes.0).0].y +
                             triangulation.nodes()[(common_nodes.1).0].y) /
                            2.;
                    let z = (triangulation.nodes()[(common_nodes.0).0].z +
                             triangulation.nodes()[(common_nodes.1).0].z) /
                            2.;

                    let edge_center = Point3::new(x, y, z);

                    assert_eq!(LocationResult::OnFaces(T4Index(elem_index),
                                                       face_index_second,
                                                       face_index),
                               locate_element_containing(T4Index(elem_index),
                                                         triangulation.elements(),
                                                         triangulation.nodes(),
                                                         &edge_center));
                }
            }
        }
    }

    fn find_common_nodes(face_1: (N3Index, N3Index, N3Index),
                         face_2: (N3Index, N3Index, N3Index))
                         -> (N3Index, N3Index) {
        let a: BTreeSet<_> = [face_1.0, face_1.1, face_1.2].iter().cloned().collect();
        let b: BTreeSet<_> = [face_2.0, face_2.1, face_2.2].iter().cloned().collect();

        let intersection: BTreeSet<_> = a.intersection(&b).collect();
        assert_eq!(2, intersection.len());

        {
            let mut iter = intersection.iter();
            let n1 = iter.next();
            let n2 = iter.next();
            (*(*n1.unwrap()), *(*n2.unwrap()))
        }
    }
}
