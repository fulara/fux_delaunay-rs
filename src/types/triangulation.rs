use types::Point2;
use types::Triangle;

use ::math;

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

    #[inline]
    pub fn find_element_containing_point(&self, p: &Point2) -> usize {
        let ele_index = 0;

        loop {
            let ele: &Triangle = &self.elements[ele_index];
            if math::on_which_side_point_lies(ele.a(&self.nodes), ele.b(&self.nodes), p) == math::PointLiesOnSide::Left {
            }

            if math::on_which_side_point_lies(ele.b(&self.nodes), ele.c(&self.nodes), p) == math::PointLiesOnSide::Left {
            }

            if math::on_which_side_point_lies(ele.c(&self.nodes), ele.a(&self.nodes), p) == math::PointLiesOnSide::Left {
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
    use super::*;

    #[test]
    fn finding_triangle_containing_point() {
        let mut pts = vec![Point2::new(0.0, 0.0), Point2::new(1.0, 0.0), Point2::new(0.0, 1.0), Point2::new(1.0, 1.0)];

        let t0 = Triangle::new(&pts, N2Index(0),N2Index(1),N2Index(2));
        let t1 = Triangle::new(&pts, N2Index(1),N2Index(2),N2Index(3));

        let mut triangles = vec![t0.clone(), t1.clone()];

        let triangulation = Triangulation::new(pts.clone(), triangles);

        let center0 = t0.create_center_point(&pts);
        let center1 = t1.create_center_point(&pts);

        assert_eq!(0, triangulation.find_element_containing_point(&center0));
        //assert_eq!(1, triangulation.find_element_containing_point(&center1));

        assert_eq!(true, false);
    }
}