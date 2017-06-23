use types::Triangulation3;
use types::Point3;
use types::Tetrahedron;
use types::N3Index;
use types::T4Index;

use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub fn add_fake_nodes(nodes: &mut Vec<Point3>) -> Vec<N3Index> {
    let (mins, maxes) = find_min_maxes(nodes);
    let diffs = Point3::new(maxes.x - mins.x, maxes.y - mins.y, maxes.z - mins.z);
    let factor = 0.1;
    let fake_nodes = create_fake_nodes(Point3::new(mins.x - diffs.x * factor,
                                                   mins.y - diffs.y * factor,
                                                   mins.z - diffs.z * factor),
                                       Point3::new(maxes.x + diffs.x * factor,
                                                   maxes.y + diffs.y * factor,
                                                   maxes.z + diffs.z * factor));

    let mut fake_indices = Vec::new();
    for fake_node in fake_nodes {
        fake_indices.push(N3Index(nodes.len()));
        nodes.push(fake_node);
    }

    fake_indices
}

fn find_min_maxes(nodes: &[Point3]) -> (Point3, Point3) {
    let (mut mins, mut maxes) = (nodes[0], nodes[0]);

    for node in nodes {
        if mins.x < node.x {
            mins.x = node.x
        }

        if mins.y < node.y {
            mins.y = node.y
        }

        if mins.z < node.z {
            mins.z = node.z
        }

        if maxes.x > node.x {
            maxes.x = node.x
        }

        if maxes.y > node.y {
            maxes.y = node.y
        }

        if maxes.z > node.z {
            maxes.z = node.z
        }
    }

    (mins, maxes)
}

fn create_fake_nodes(low_values: Point3, high_values: Point3) -> Vec<Point3> {
    vec![Point3::new(low_values.x, low_values.y, low_values.z),
         Point3::new(low_values.x, high_values.y, low_values.z),
         Point3::new(high_values.x, high_values.y, low_values.z),
         Point3::new(high_values.x, low_values.y, low_values.z),
         Point3::new(low_values.x, low_values.y, high_values.z),
         Point3::new(low_values.x, high_values.y, high_values.z),
         Point3::new(high_values.x, high_values.y, high_values.z),
         Point3::new(high_values.x, low_values.y, high_values.z)]
}


pub fn remove_fake_nodes(triangulation: &mut Triangulation3, fake_indices: &[N3Index]) {
    let mut eles_to_remove = BTreeSet::new();

    for (index, ele) in triangulation.elements().iter().enumerate() {
        let ele: &Tetrahedron = ele;

        for ele_node in ele.nodes() {
            if fake_indices.iter().any(|n| n == ele_node) {
                eles_to_remove.insert(index);
            }
        }
    }

    let mut iteration = 0;

    for index_to_remove in eles_to_remove {
        let mut eles = Vec::new();
        let index_to_remove = index_to_remove - iteration;
        iteration += 1;

        for (index, ele) in triangulation.elements().iter().enumerate() {
            if index == index_to_remove {
                continue;
            }

            let mut tetra: Tetrahedron = ele.clone();
            let neighbors = *tetra.neighbors();
            for (neighbor_index, neighbor_opt) in neighbors.iter().enumerate() {
                if let &Some(neighbor) = neighbor_opt {
                    if neighbor.0 > index_to_remove {
                        tetra.set_neighbor(neighbor_index, Some(T4Index(neighbor.0 - 1)));
                    }
                }
            }

            eles.push(tetra);
        }

        *triangulation.elements_mut() = eles;
    }

    for _ in 0..8 {
        triangulation.nodes_mut().pop();
    }
}
