use types::Point3;
use types::Tetrahedron;
use types::N3Index;
use types::T4Index;

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
}
