use types::T3Index;

use types::Triangle;
use types::Point2;

use math;

#[derive(Debug, Eq, PartialEq)]
pub enum LocationResult {
    InElement(T3Index),
    OnEdge(T3Index),
}

#[inline]
pub fn locate_element_containing(elements: &Vec<Triangle>, nodes: &Vec<Point2>, p: &Point2) -> LocationResult {
    let mut ele_index = T3Index(0);

    loop {
        let ele: &Triangle = &elements[ele_index.0];

        let mut current_edge = 0;
        loop {
            if current_edge == 3 {
                break;
            }

            let edge = ele.edges_as_points_tuples(nodes)[current_edge];

            match math::on_which_side_point_lies(edge.0, edge.1, p) {
                math::PointLiesOnSide::Left => {
                    assert!(ele.get_neighbor_from_index(current_edge).is_some());

                    ele_index = ele.get_neighbor_from_index(current_edge).unwrap();
                    break;
                }
                math::PointLiesOnSide::OnLine => {
                    return LocationResult::OnEdge(ele_index);
                },
                math::PointLiesOnSide::Right => ()
            }
            current_edge += 1;
        }

        if current_edge == 3 && ele.is_point_inside(&nodes, &p) {
            return LocationResult::InElement(ele_index);
        }
    }
}

#[cfg(test)]
mod tests {
    use types::Point2;
    use types::Triangle;
    use types::N2Index;
    use types::T3Index;
    use types::Triangulation;
    use super::*;

    #[test]
    fn locator_test() {
        let mut pts = vec![Point2::new(0.0, 0.0), Point2::new(1.0, 0.0), Point2::new(0.0, 1.0), Point2::new(1.0, 1.0)];

        let t0 = Triangle::new(&pts, N2Index(0), N2Index(1), N2Index(2));
        let t1 = Triangle::new(&pts, N2Index(1), N2Index(2), N2Index(3));

        let mut triangles = vec![t0.clone(), t1.clone()];

        let triangulation = Triangulation::new_from_prebuilt_triangulation(pts.clone(), triangles);

        let center0 = t0.create_center_point(&pts);
        let center1 = t1.create_center_point(&pts);

        assert_eq! (LocationResult::InElement(T3Index(0)), locate_element_containing(triangulation.elements(), triangulation.nodes(), &center0));
        assert_eq!(LocationResult::InElement(T3Index(1)), locate_element_containing(triangulation.elements(), triangulation.nodes(), &center1));

        assert_eq! (LocationResult::OnEdge(T3Index(0)), locate_element_containing(triangulation.elements(), triangulation.nodes(), &Point2::new(0.5, 0.)));
    }
}