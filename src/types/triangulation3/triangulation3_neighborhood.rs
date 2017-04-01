use types::N3Index;
use types::T4Index;
use types::Tetrahedron;
use std::collections::BTreeMap;

use algorithms3::sort_3::sort_3;

pub struct Triangulation3Neighborhood {
    neighbor_map: BTreeMap<(N3Index, N3Index, N3Index), T4Index>,
}

impl Triangulation3Neighborhood {
    pub fn teach_triangles_of_neighborhood(elements: &mut [Tetrahedron]) {
        let mut neighborhood = Triangulation3Neighborhood { neighbor_map: BTreeMap::new() };

        let len = elements.len();

        for index in 0..len {
            neighborhood.add_and_teach(elements, T4Index(index));
        }
    }

    pub fn teach_selected_elements_of_neighborhood(indices: &[T4Index],
                                                   elements: &mut [Tetrahedron]) {
        let mut neighborhood = Triangulation3Neighborhood { neighbor_map: BTreeMap::new() };

        for index in indices.iter() {
            neighborhood.add_and_teach(elements, *index);
        }
    }

    fn new() -> Triangulation3Neighborhood {
        Triangulation3Neighborhood { neighbor_map: BTreeMap::new() }
    }

    fn add_and_teach(&mut self, elements: &mut [Tetrahedron], index: T4Index) {
        let indices = elements[index.0].faces_as_indices_tuples();
        for &(n1, n2, n3) in indices.into_iter() {
            let sorted = sort_3(n1, n2, n3);

            let mut found = None;
            let mut should_insert = false;

            if let Some(val) = self.neighbor_map.get(&sorted) {
                if *val != index {
                    found = Some(*val);
                }
            } else {
                should_insert = true;
            }

            if should_insert {
                self.neighbor_map.insert(sorted, index);
            }

            if let Some(found) = found {
                {
                    let ele: &mut Tetrahedron = &mut elements[found.0];
                    ele.update_neighbor(n1, n2, n3, Some(index));
                }

                {
                    let ele: &mut Tetrahedron = &mut elements[index.0];
                    ele.update_neighbor(n1, n2, n3, Some(found));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use types::Point3;
    use types::Tetrahedron;
    use types::N3Index;
    use types::T4Index;
    use super::*;

    use types::triangulation3_test_utils::get_example_initial_point_set;
    use types::triangulation3::triangulation3_initiation::create_initial_tetra_set;

    #[test]
    fn testing_neighborhood() {
        let points = vec![Point3::new(0., 0., 0.),
                          Point3::new(100., 0., 0.),
                          Point3::new(0., 100., 0.),
                          Point3::new(0., 0., 100.),
                          Point3::new(100., 100., 0.)];

        let t0 = Tetrahedron::new(&points, N3Index(0), N3Index(1), N3Index(2), N3Index(3));
        let t1 = Tetrahedron::new(&points, N3Index(1), N3Index(2), N3Index(3), N3Index(4));
        let mut eles = [t0, t1];

        Triangulation3Neighborhood::teach_triangles_of_neighborhood(&mut eles);

        assert_eq!(Option::None,
                   eles[0].get_neighbor_for_indices(N3Index(0), N3Index(1), N3Index(2)));
        assert_eq!(Some(T4Index(1)),
                   eles[0].get_neighbor_for_indices(N3Index(1), N3Index(2), N3Index(3)));
        assert_eq!(Option::None,
                   eles[0].get_neighbor_for_indices(N3Index(2), N3Index(3), N3Index(0)));
        assert_eq!(Option::None,
                   eles[0].get_neighbor_for_indices(N3Index(3), N3Index(0), N3Index(1)));

        assert_eq!(Some(T4Index(1)),
                   eles[0].get_neighbor_for_indices(N3Index(3), N3Index(2), N3Index(1)));


        assert_eq!(Some(T4Index(0)),
                   eles[1].get_neighbor_for_indices(N3Index(1), N3Index(2), N3Index(3)));
        assert_eq!(None,
                   eles[1].get_neighbor_for_indices(N3Index(2), N3Index(3), N3Index(4)));
        assert_eq!(None,
                   eles[1].get_neighbor_for_indices(N3Index(3), N3Index(4), N3Index(1)));
        assert_eq!(None,
                   eles[1].get_neighbor_for_indices(N3Index(4), N3Index(1), N3Index(2)));
    }

    #[test]
    fn testing_neighborhood_with_initiation() {
        let nodes = get_example_initial_point_set();
        let mut eles: Vec<Tetrahedron> = create_initial_tetra_set(&[0, 1, 2, 3, 4, 5, 6, 7],
                                                                  &nodes);

        Triangulation3Neighborhood::teach_triangles_of_neighborhood(&mut eles);

        /* cheat sheet
        assert_eq!(&[N3Index(0),N3Index(3),N3Index(4),N3Index(1)],tetras[0].nodes());
        assert_eq!(&[N3Index(1),N3Index(2),N3Index(3),N3Index(6)],tetras[1].nodes());
        assert_eq!(&[N3Index(1),N3Index(4),N3Index(5),N3Index(6)],tetras[2].nodes());
        assert_eq!(&[N3Index(3),N3Index(4),N3Index(6),N3Index(7)],tetras[3].nodes());
        assert_eq!(&[N3Index(1),N3Index(3),N3Index(4),N3Index(6)],tetras[4].nodes());
        */

        assert_eq!(None,
                   eles[0].get_neighbor_for_indices(N3Index(0), N3Index(3), N3Index(4)));
        assert_eq!(Some(T4Index(4)),
                   eles[0].get_neighbor_for_indices(N3Index(3), N3Index(4), N3Index(1)));
        assert_eq!(None,
                   eles[0].get_neighbor_for_indices(N3Index(4), N3Index(1), N3Index(0)));
        assert_eq!(None,
                   eles[0].get_neighbor_for_indices(N3Index(1), N3Index(0), N3Index(3)));

        assert_eq!(None,
                   eles[1].get_neighbor_for_indices(N3Index(1), N3Index(2), N3Index(3)));
        assert_eq!(None,
                   eles[1].get_neighbor_for_indices(N3Index(2), N3Index(3), N3Index(6)));
        assert_eq!(Some(T4Index(4)),
                   eles[1].get_neighbor_for_indices(N3Index(3), N3Index(6), N3Index(1)));
        assert_eq!(None,
                   eles[1].get_neighbor_for_indices(N3Index(6), N3Index(1), N3Index(2)));

        assert_eq!(None,
                   eles[2].get_neighbor_for_indices(N3Index(1), N3Index(4), N3Index(5)));
        assert_eq!(None,
                   eles[2].get_neighbor_for_indices(N3Index(4), N3Index(5), N3Index(6)));
        assert_eq!(None,
                   eles[2].get_neighbor_for_indices(N3Index(5), N3Index(6), N3Index(1)));
        assert_eq!(Some(T4Index(4)),
                   eles[2].get_neighbor_for_indices(N3Index(6), N3Index(1), N3Index(4)));

        assert_eq!(Some(T4Index(4)),
                   eles[3].get_neighbor_for_indices(N3Index(3), N3Index(4), N3Index(6)));
        assert_eq!(None,
                   eles[3].get_neighbor_for_indices(N3Index(4), N3Index(6), N3Index(7)));
        assert_eq!(None,
                   eles[3].get_neighbor_for_indices(N3Index(6), N3Index(7), N3Index(3)));
        assert_eq!(None,
                   eles[3].get_neighbor_for_indices(N3Index(7), N3Index(3), N3Index(4)));

        assert_eq!(Some(T4Index(0)),
                   eles[4].get_neighbor_for_indices(N3Index(1), N3Index(3), N3Index(4)));
        assert_eq!(Some(T4Index(3)),
                   eles[4].get_neighbor_for_indices(N3Index(3), N3Index(4), N3Index(6)));
        assert_eq!(Some(T4Index(2)),
                   eles[4].get_neighbor_for_indices(N3Index(4), N3Index(6), N3Index(1)));
        assert_eq!(Some(T4Index(1)),
                   eles[4].get_neighbor_for_indices(N3Index(6), N3Index(1), N3Index(3)));
    }
}
