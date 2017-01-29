use types::Point2;
use types::Triangle;
use types::TriangulationNeighborhood;
use types::T3Index;

use ::math;

pub struct Triangulation {
    nodes: Vec<Point2>,
    elements: Vec<Triangle>,
    neighborhood: TriangulationNeighborhood,
}

impl Triangulation {
    #[inline]
    pub fn new(nodes: Vec<Point2>, elements: Vec<Triangle>) -> Triangulation {
        let mut tr = Triangulation { nodes: nodes, elements: elements, neighborhood: TriangulationNeighborhood::new() };

        for i in 0..tr.elements.len() {
            let triangle = &tr.elements[i];
            tr.neighborhood.register_triangle(triangle, T3Index(i));
        }

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

    #[inline]
    pub fn find_element_containing_point(&self, p: &Point2) -> T3Index {
        let mut ele_index = T3Index(0);

        loop {
            let ele: &Triangle = &self.elements[ele_index.0];
            if math::on_which_side_point_lies(ele.a(&self.nodes), ele.b(&self.nodes), p) == math::PointLiesOnSide::Left {
                assert!(self.neighborhood.get_neighbor(ele.index_a(), ele.index_b(),ele_index).is_some());

                ele_index = self.neighborhood.get_neighbor(ele.index_a(), ele.index_b(),ele_index).unwrap();
                continue;
            }

            if math::on_which_side_point_lies(ele.b(&self.nodes), ele.c(&self.nodes), p) == math::PointLiesOnSide::Left {
                assert!(self.neighborhood.get_neighbor(ele.index_b(), ele.index_c(),ele_index).is_some());

                ele_index = self.neighborhood.get_neighbor(ele.index_b(), ele.index_c(),ele_index).unwrap();
                continue;
            }

            if math::on_which_side_point_lies(ele.c(&self.nodes), ele.a(&self.nodes), p) == math::PointLiesOnSide::Left {
                assert!(self.neighborhood.get_neighbor(ele.index_c(), ele.index_a(),ele_index).is_some());

                ele_index = self.neighborhood.get_neighbor(ele.index_c(), ele.index_a(),ele_index).unwrap();
                continue;
            }

            if ele.is_point_inside(&self.nodes, &p) {
                return ele_index
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
    use super::*;

    #[test]
    fn finding_triangle_containing_point() {
        let mut pts = vec![Point2::new(0.0, 0.0), Point2::new(1.0, 0.0), Point2::new(0.0, 1.0), Point2::new(1.0, 1.0)];

        let t0 = Triangle::new(&pts, N2Index(0), N2Index(1), N2Index(2));
        let t1 = Triangle::new(&pts, N2Index(1), N2Index(2), N2Index(3));

        let mut triangles = vec![t0.clone(), t1.clone()];

        let triangulation = Triangulation::new(pts.clone(), triangles);

        let center0 = t0.create_center_point(&pts);
        let center1 = t1.create_center_point(&pts);

        assert_eq!(T3Index(0), triangulation.find_element_containing_point(&center0));
        assert_eq!(T3Index(1), triangulation.find_element_containing_point(&center1));
    }
}