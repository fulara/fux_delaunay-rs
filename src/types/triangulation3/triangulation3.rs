use types::Point3;
use types::Tetrahedron;
use types::N3Index;
use types::T4Index;
use algorithms3::element_locators::*;
use super::triangulation3_insertion;

use super::triangulation3_neighborhood::Triangulation3Neighborhood;

pub struct Triangulation3 {
    nodes: Vec<Point3>,
    elements: Vec<Tetrahedron>,
    last_added_element_index: T4Index,
}

impl Triangulation3 {
    #[inline]
    pub fn new_from_prebuilt_triangulation(nodes: Vec<Point3>,
                                           elements: Vec<Tetrahedron>)
                                           -> Triangulation3 {
        let mut tr = Triangulation3 {
            nodes: nodes,
            elements: elements,
            last_added_element_index: T4Index(0),
        };

        Triangulation3Neighborhood::teach_triangles_of_neighborhood(&mut tr.elements);
        tr
    }

    #[inline]
    pub fn nodes(&self) -> &Vec<Point3> {
        &self.nodes
    }

    #[inline]
    pub fn elements(&self) -> &Vec<Tetrahedron> {
        &self.elements
    }

    #[inline]
    pub fn elements_mut(&mut self) -> &mut Vec<Tetrahedron> {
        &mut self.elements
    }

    #[inline]
    pub fn insert_node(&mut self, p: &Point3) {
        self.nodes.push(*p);
        let new_node_index = N3Index(self.nodes.len() - 1);

        self.insert_into_triangulation(new_node_index);
    }

    #[inline]
    fn insert_into_triangulation(&mut self, new_node_index: N3Index) {
        let location_result = locate_element_containing(self.last_added_element_index,
                                                        &self.elements,
                                                        &self.nodes,
                                                        &self.nodes[new_node_index.0]);

        match location_result {
            LocationResult::InElement(ele_index) => {
                self.last_added_element_index = ele_index;
                let (t1_index, t2_index, t3_index, t4_index) =
                    triangulation3_insertion::insert_into_element(self, ele_index, new_node_index);
                //lawson_flipping::propagating_flip(self, new_node_index, t1_index);
                //lawson_flipping::propagating_flip(self, new_node_index, t2_index);
                //lawson_flipping::propagating_flip(self, new_node_index, t3_index);
            }
            _ => unimplemented!(),
        }
    }
}
