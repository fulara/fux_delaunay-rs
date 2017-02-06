use types::Point2;
use types::Triangle;
use types::TriangulationNeighborhood;
use types::T3Index;

pub struct Triangulation {
    nodes: Vec<Point2>,
    elements: Vec<Triangle>,
}

impl Triangulation {
    #[inline]
    pub fn new_from_prebuilt_triangulation(nodes: Vec<Point2>, elements: Vec<Triangle>) -> Triangulation {
        let mut neighborhood = TriangulationNeighborhood::new();
        let mut tr = Triangulation { nodes: nodes, elements: elements };

        for i in 0..tr.elements.len() {
            let triangle = &tr.elements[i];
            neighborhood.register_triangle(triangle, T3Index(i));
        }

        neighborhood.teach_triangles_of_neighborhood(&mut tr.elements);

        tr
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

#[cfg(test)]
mod tests {
    use types::Point2;
    use types::Triangle;
    use types::N2Index;
    use types::T3Index;
    use types::element_locators;
    use types::element_locators::LocationResult;
    use super::*;
}