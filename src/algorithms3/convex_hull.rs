use types::N3Index;
use types::Point3;
use types::Tetrahedron;

#[derive(Debug, PartialEq)]
pub enum ConvexHull {
    FourPoints([N3Index; 4]),
    FivePoints([N3Index; 5]),
}

pub fn convex_hull(node_indexes: &[N3Index; 5], nodes: &[Point3]) -> ConvexHull {
    for shift in 0..5 {
        let ns = [node_indexes[get_index(0 + shift)],
                  node_indexes[get_index(1 + shift)],
                  node_indexes[get_index(2 + shift)],
                  node_indexes[get_index(3 + shift)]];
        let tetra = Tetrahedron::new(nodes, ns[0], ns[1], ns[2], ns[3]);
        if !tetra.is_point_outside(&nodes[node_indexes[get_index(4 + shift)].0], nodes) {
            return ConvexHull::FourPoints(ns);
        }
    }

    return ConvexHull::FivePoints(*node_indexes);
}

fn get_index(i: usize) -> usize {
    i % 5
}

#[cfg(test)]
mod convex_hull_tests {
    use super::*;
    use types::*;
    #[test]
    fn test1() {
        let nodes = vec![Point3::new(0., 0., 0.),
                         Point3::new(1., 0., 0.),
                         Point3::new(0., 1., 0.),
                         Point3::new(0., 0., 1.),
                         Point3::new(1., 1., 1.)];
        let indexes = [N3Index(0), N3Index(1), N3Index(2), N3Index(3), N3Index(4)];

        assert_eq!(ConvexHull::FivePoints([N3Index(0), N3Index(1), N3Index(2), N3Index(3),
                                           N3Index(4)]),
                   convex_hull(&indexes, &nodes));
    }

    #[test]
    fn test2() {
        let nodes = vec![Point3::new(0., 0., 0.),
                         Point3::new(1., 0., 0.),
                         Point3::new(0., 1., 0.),
                         Point3::new(0., 0., 1.),
                         Point3::new(0.25, 0.25, 0.25)];

        let indexes = [N3Index(0), N3Index(1), N3Index(2), N3Index(3), N3Index(4)];

        assert_eq!(ConvexHull::FourPoints([N3Index(0), N3Index(1), N3Index(2), N3Index(3)]),
                   convex_hull(&indexes, &nodes));
    }
}
