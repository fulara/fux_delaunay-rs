use types::T3Index;

use types::Triangle;
use types::Point2;

use math;

#[derive(Debug, Eq, PartialEq)]
pub enum LocationResult {
    InElement(T3Index),
    OnEdge(T3Index, usize),
}

#[inline]
pub fn locate_element_containing(start_lookup_at : T3Index, elements: &Vec<Triangle>, nodes: &Vec<Point2>, p: &Point2) -> LocationResult {
    let mut ele_index = start_lookup_at;

    loop {
        let ele: &Triangle = &elements[ele_index.0];

        let mut current_edge = 0;
        let mut on_edge_found: Option<usize> = None;

        loop {
            //println!("locating: curr index is: {:?} {:?}", ele_index, ele);
            //println!("current_edge: {:?}", current_edge);
            if current_edge == 3 {
                break;
            }

            let edge = ele.edges_as_points_tuples(nodes)[current_edge];

            match math::side_of_line(edge.0, edge.1, p) {
                math::PointLiesOnLineSide::Left => {
                    assert!(ele.get_neighbor_from_index(current_edge).is_some());

                    ele_index = ele.get_neighbor_from_index(current_edge).unwrap();
                    break;
                }
                math::PointLiesOnLineSide::OnLine => {
                    on_edge_found = Some(current_edge);
                },
                math::PointLiesOnLineSide::Right => ()
            }
            current_edge += 1;
        }

        if current_edge == 3 {
            if let Some(on_edge_found) = on_edge_found {
                return LocationResult::OnEdge(ele_index, on_edge_found);
            }
            if ele.is_point_inside(&nodes, &p) {
                return LocationResult::InElement(ele_index);
            } else {
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
    use types::Triangulation;
    use super::*;

    #[test]
    fn locator_test() {
        let pts = vec![Point2::new(0.0, 0.0), Point2::new(1.0, 0.0), Point2::new(0.0, 1.0), Point2::new(1.0, 1.0)];

        let t0 = Triangle::new(&pts, N2Index(0), N2Index(1), N2Index(2));
        let t1 = Triangle::new(&pts, N2Index(1), N2Index(2), N2Index(3));

        let triangles = vec![t0.clone(), t1.clone()];

        let triangulation = Triangulation::new_from_prebuilt_triangulation(pts.clone(), triangles);

        let center0 = t0.create_center_point(&pts);
        let center1 = t1.create_center_point(&pts);

        assert_eq! (LocationResult::InElement(T3Index(0)), locate_element_containing(T3Index(0),triangulation.elements(), triangulation.nodes(), &center0));
        assert_eq!(LocationResult::InElement(T3Index(1)), locate_element_containing(T3Index(0),triangulation.elements(), triangulation.nodes(), &center1));

        assert_eq! (LocationResult::OnEdge(T3Index(0), 2usize), locate_element_containing(T3Index(0),triangulation.elements(), triangulation.nodes(), &Point2::new(0.5, 0.)));
    }
}