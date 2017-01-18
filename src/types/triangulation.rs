use types::Point2;
use types::Triangle;

pub struct Triangulation {
    nodes: Vec<Point2>,
    elements: Vec<Triangle>,
}

impl Triangulation {
    #[inline]
    pub fn new(nodes: Vec<Point2>, elements: Vec<Triangle>) -> Triangulation {
        Triangulation { nodes: nodes, elements: elements }
    }

    #[inline]
    pub fn nodes(&self) -> &Vec<Point2> {
        &self.nodes
    }

    #[inline]
    pub fn elements(&self) -> &Vec<Triangle> {
        &self.elements
    }
}