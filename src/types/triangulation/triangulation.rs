use types::Point2;
use types::Triangle;
use types::TriangulationNeighborhood;
use types::N2Index;

use algorithms::element_locators::*;
use algorithms::lawson_flipping;

use super::triangulation_insertion;

pub struct Triangulation {
    nodes: Vec<Point2>,
    elements: Vec<Triangle>,
}

impl Triangulation {
    #[inline]
    pub fn new_from_prebuilt_triangulation(nodes: Vec<Point2>, elements: Vec<Triangle>) -> Triangulation {
        let mut tr = Triangulation { nodes: nodes, elements: elements };

        TriangulationNeighborhood::teach_triangles_of_neighborhood(&mut tr.elements);
        tr
    }

    #[inline]
    //4 bounding box points
    pub fn new(top_left: Point2, top_right: Point2, bottom_left: Point2, bottom_right: Point2) -> Triangulation {
        let nodes = vec![top_left, top_right, bottom_left, bottom_right];
        let mut elements = vec![Triangle::new(&nodes, N2Index(0), N2Index(3), N2Index(2)), Triangle::new(&nodes, N2Index(0), N2Index(1), N2Index(3))];

        TriangulationNeighborhood::teach_triangles_of_neighborhood(&mut elements);

        Triangulation {
            nodes: nodes,
            elements: elements
        }
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

        ::math::which_side_of_circumcircle(a, b, c, p) == ::math::CircleSide::Inside
    }

    #[inline]
    pub fn insert_node(&mut self, p: &Point2) {
        let location_result = locate_element_containing(&self.elements, &self.nodes, p);

        self.nodes.push(*p);
        let new_node_index = N2Index(self.nodes.len() - 1);

        match location_result {
            LocationResult::InElement(ele_index) => {
                let (t1_index, t2_index, t3_index) = triangulation_insertion::insert_into_element(self, ele_index, new_node_index);
                lawson_flipping::propagating_flip(self, new_node_index, t1_index);
                lawson_flipping::propagating_flip(self, new_node_index, t2_index);
                lawson_flipping::propagating_flip(self, new_node_index, t3_index);
            }
            LocationResult::OnEdge(ele_index, edge_index) => {
                let (neighbor_index, edge_node1, edge_node2)  = {
                    let ele : &Triangle = &self.elements[ele_index.0];
                    let neighbor_index = ele.get_neighbor_from_index(edge_index);
                    let edge = ele.get_edge(edge_index);

                    (neighbor_index, edge.0, edge.1)
                };


                if let Some(neighbor_index) = neighbor_index {
                    triangulation_insertion::insert_into_element(self, ele_index, new_node_index);
                    let neighbor_last_node = {
                        let neighbor : &Triangle = &self.elements[neighbor_index.0];
                        neighbor.get_other_last_node(edge_node1, edge_node2)
                    };
                    lawson_flipping::try_flip(self, neighbor_last_node, neighbor_index);
                }  else {
                    panic!("insert_node_on_edge not implemented for case when there is no neighbor");
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
        let triangulation: Triangulation = Triangulation::new(Point2::new(0., 1.), Point2::new(1., 1.), Point2::new(0., 0.), Point2::new(1., 0.));

        assert_eq!(2, triangulation.elements.len());
        assert_eq!(Triangle::new_exact([N2Index(0), N2Index(3), N2Index(2)], [Some(T3Index(1)), None, None]), triangulation.elements()[0]);
        assert_eq!(Triangle::new_exact([N2Index(0), N2Index(1), N2Index(3)], [None, None, Some(T3Index(0))]), triangulation.elements()[1]);
    }
}