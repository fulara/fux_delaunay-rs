use types::Tetrahedron;
use types::Point3;
use types::N3Index;

pub fn create_initial_tetra_set(nodes : &[Point3]) -> Vec<Tetrahedron> {
    assert_nodes(nodes);

    vec!(Tetrahedron::new(nodes, N3Index(0),N3Index(1),N3Index(4),N3Index(3)),
         Tetrahedron::new(nodes, N3Index(1),N3Index(2),N3Index(3),N3Index(6)),
         Tetrahedron::new(nodes, N3Index(1),N3Index(4),N3Index(5),N3Index(6)),
         Tetrahedron::new(nodes, N3Index(3),N3Index(4),N3Index(6),N3Index(7)),
         Tetrahedron::new(nodes, N3Index(1),N3Index(3),N3Index(4),N3Index(6)))
}

fn assert_nodes(nodes : &[Point3]) {
    assert!(nodes.len() == 8);

    assert!(nodes[0].z == nodes[1].z);
    assert!(nodes[0].z == nodes[2].z);
    assert!(nodes[0].z == nodes[3].z);

    assert!(nodes[0].z != nodes[4].z);

    assert!(nodes[4].z == nodes[5].z);
    assert!(nodes[4].z == nodes[6].z);
    assert!(nodes[4].z == nodes[7].z);

    assert!(nodes[0].x == nodes[1].x);
    assert!(nodes[0].x == nodes[4].x);
    assert!(nodes[0].x == nodes[5].x);

    assert!(nodes[0].x != nodes[2].x);

    assert!(nodes[2].x == nodes[3].x);
    assert!(nodes[2].x == nodes[6].x);
    assert!(nodes[2].x == nodes[7].x);

    assert!(nodes[0].y == nodes[3].y);
    assert!(nodes[0].y == nodes[4].y);
    assert!(nodes[0].y == nodes[7].y);

    assert!(nodes[0].y != nodes[1].y);

    assert!(nodes[1].y == nodes[2].y);
    assert!(nodes[1].y == nodes[5].y);
    assert!(nodes[1].y == nodes[6].y);
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::triangulation3_test_utils::get_example_initial_point_set;

    #[test]
    fn example_set() {
        let nodes_set = get_example_initial_point_set();
        let tetras = create_initial_tetra_set(&nodes_set);

        //does this take makes sense? probably not.

        assert_eq!(5, tetras.len());
        assert_eq!(&[N3Index(0),N3Index(3),N3Index(4),N3Index(1)],tetras[0].nodes());
        assert_eq!(&[N3Index(1),N3Index(2),N3Index(3),N3Index(6)],tetras[1].nodes());
        assert_eq!(&[N3Index(1),N3Index(4),N3Index(5),N3Index(6)],tetras[2].nodes());
        assert_eq!(&[N3Index(3),N3Index(4),N3Index(6),N3Index(7)],tetras[3].nodes());
        assert_eq!(&[N3Index(1),N3Index(3),N3Index(4),N3Index(6)],tetras[4].nodes());

    }
}

