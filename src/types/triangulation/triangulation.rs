use types::Point2;
use types::Triangle;
use types::TriangulationNeighborhood;
use types::N2Index;
use types::T3Index;

use algorithms::element_locators::*;
use algorithms::lawson_flipping;

use super::triangulation_insertion;
use super::triangulation_utilities;

pub struct Triangulation {
    nodes: Vec<Point2>,
    elements: Vec<Triangle>,
    last_added_element_index: T3Index,
}

impl Triangulation {
    #[inline]
    pub fn new_from_prebuilt_triangulation(nodes: Vec<Point2>, elements: Vec<Triangle>) -> Triangulation {
        let mut tr = Triangulation { nodes: nodes, elements: elements, last_added_element_index: T3Index(0) };

        TriangulationNeighborhood::teach_triangles_of_neighborhood(&mut tr.elements);
        tr
    }

    #[inline]
    pub fn new(nodes: &[Point2]) -> Triangulation {
        let (top_left_index, top_right_index, bottom_left_index, bottom_right_index) = triangulation_utilities::find_corner_nodes(nodes);
        let mut indexes_except_corner: Vec<usize> = Vec::new();

        for i in 0..nodes.len() {
            if i != top_left_index && i != top_right_index && i != bottom_left_index && i != bottom_right_index {
                indexes_except_corner.push(i);
            }
        }

        indexes_except_corner.sort_by(|a, b| {
            if nodes[*a].x < nodes[*b].x {
                ::std::cmp::Ordering::Less
            } else if nodes[*a].x > nodes[*b].x {
                ::std::cmp::Ordering::Greater
            } else {
                if nodes[*a].y < nodes[*b].y {
                    ::std::cmp::Ordering::Less
                } else if nodes[*a].y > nodes[*b].y {
                    ::std::cmp::Ordering::Greater
                } else {
                    panic!("Triangulation received equal nodes. x {} y {}", nodes[*a].x, nodes[*a].y);
                }
            }
        });

        let nodes = Vec::from(nodes);
        let mut eles = vec![Triangle::new(&nodes, N2Index(top_left_index), N2Index(bottom_right_index), N2Index(bottom_left_index)),
                            Triangle::new(&nodes, N2Index(top_left_index), N2Index(top_right_index), N2Index(bottom_right_index))];

        TriangulationNeighborhood::teach_triangles_of_neighborhood(&mut eles);
        let mut triangulation = Triangulation {
            elements: eles,
            last_added_element_index: T3Index(0),
            nodes: nodes,
        };

        for index in indexes_except_corner.into_iter() {
            triangulation.insert_into_triangulation(N2Index(index));
        }

        triangulation
    }

    #[inline]
    pub fn nodes(&self) -> &Vec<Point2> {
        &self.nodes
    }

    #[inline]
    pub fn elements(&self) -> &Vec<Triangle> {
        &self.elements
    }

    #[inline]
    pub fn elements_mut(&mut self) -> &mut Vec<Triangle> {
        &mut self.elements
    }

    #[inline]
    pub fn is_inside_circumcircle(&self, tr: &Triangle, p: &Point2) -> bool {
        let a = tr.a(self.nodes());
        let b = tr.b(self.nodes());
        let c = tr.c(self.nodes());

        ::math::circumcircle_side(a, b, c, p) == ::math::CircleSide::Inside
    }

    #[inline]
    pub fn insert_node(&mut self, p: &Point2) {
        self.nodes.push(*p);
        let new_node_index = N2Index(self.nodes.len() - 1);

        self.insert_into_triangulation(new_node_index);
    }

    #[inline]
    fn insert_into_triangulation(&mut self, new_node_index: N2Index) {
        let location_result = locate_element_containing(self.last_added_element_index, &self.elements, &self.nodes, &self.nodes[new_node_index.0]);

        match location_result {
            LocationResult::InElement(ele_index) => {
                self.last_added_element_index = ele_index;
                let (t1_index, t2_index, t3_index) = triangulation_insertion::insert_into_element(self, ele_index, new_node_index);
                lawson_flipping::propagating_flip(self, new_node_index, t1_index);
                lawson_flipping::propagating_flip(self, new_node_index, t2_index);
                lawson_flipping::propagating_flip(self, new_node_index, t3_index);
            }
            LocationResult::OnEdge(ele_index, edge_index) => {
                self.last_added_element_index = ele_index;
                let (neighbor_index, edge_node1, edge_node2) = {
                    let ele: &Triangle = &self.elements[ele_index.0];
                    let neighbor_index = ele.get_neighbor_from_index(edge_index);
                    let edge = ele.get_edge(edge_index);

                    (neighbor_index, edge.0, edge.1)
                };


                if let Some(neighbor_index) = neighbor_index {
                    triangulation_insertion::insert_into_element(self, ele_index, new_node_index);
                    let neighbor_last_node = {
                        let neighbor: &Triangle = &self.elements[neighbor_index.0];
                        neighbor.get_other_last_node(edge_node1, edge_node2)
                    };
                    lawson_flipping::try_flip(self, neighbor_last_node, neighbor_index);
                } else {
                    let (ele1, ele2) = triangulation_insertion::insert_in_edge(self, ele_index, new_node_index, edge_index);

                    lawson_flipping::propagating_flip(self, new_node_index, ele1);
                    lawson_flipping::propagating_flip(self, new_node_index, ele2);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use types::Point2;
    use types::Triangle;
    use types::N2Index;
    use types::T3Index;
    use super::Triangulation;

    #[test]
    fn testing_bounding_box_creation() {
        let triangulation: Triangulation = Triangulation::new(&[Point2::new(0., 1.), Point2::new(1., 1.), Point2::new(0., 0.), Point2::new(1., 0.)]);

        assert_eq!(2, triangulation.elements.len());
        assert_eq!(Triangle::new_exact([N2Index(0), N2Index(3), N2Index(2)], [Some(T3Index(1)), None, None]), triangulation.elements()[0]);
        assert_eq!(Triangle::new_exact([N2Index(0), N2Index(1), N2Index(3)], [None, None, Some(T3Index(0))]), triangulation.elements()[1]);
    }
}