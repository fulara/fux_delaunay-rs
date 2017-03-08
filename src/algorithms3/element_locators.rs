use types::T4Index;

use types::Tetrahedron;
use types::Point3;

use math;

#[derive(Debug, Eq, PartialEq)]
pub enum LocationResult {
    InElement(T4Index),
    OnEdge(T4Index, usize),
    OnFace(T4Index, usize),
}


#[inline]
pub fn locate_element_containing(start_lookup_at: T4Index, elements: &[Tetrahedron], nodes: &[Point3], p: &Point3) -> LocationResult {
    let mut ele_index = start_lookup_at;

    loop {
        let ele: &Tetrahedron = &elements[ele_index.0];

        let mut current_face = 0;
        let mut on_face_found: Option<usize> = None;

        loop {
            if current_face == 4 {
                break;
            }

            let edge = ele.edges_as_points_tuples(nodes)[current_face];

            println!("matching no {:?} face. point is: {:?} result is {:?} edges: {:?} {:?} {:?}", current_face, p, math::side_of_plane(edge.0, edge.1, edge.2, p), edge.0, edge.1, edge.2);
            match math::side_of_plane(edge.0, edge.1, edge.2, p) {
                math::SideOfPlane::Left => {
                    assert!(ele.get_neighbor_from_index(current_face).is_some());

                    ele_index = ele.get_neighbor_from_index(current_face).unwrap();
                    break;
                }
                math::SideOfPlane::OnPlane => {
                    on_face_found = Some(current_face);
                },
                math::SideOfPlane::Right => ()
            }
            current_face += 1;
        }

        if current_face == 4 {
            if let Some(on_edge_found) = on_face_found {
                return LocationResult::OnFace(ele_index, on_edge_found);
            }
            return LocationResult::InElement(ele_index);
            /*TODO if ele.is_point_inside(&nodes, &p) {
                return LocationResult::InElement(ele_index);
            } else {
            } */
        }
    }
}


#[cfg(test)]
mod tests {
    use types::Point3;
    use types::Tetrahedron;
    use types::N3Index;
    use types::T4Index;
    use types::Triangulation3;
    use super::*;

    #[test]
    fn locator_single_element() {
        let pts = vec![Point3::new(0.0, 0.0, 0.0),
                       Point3::new(1.0, 0.0, 0.0),
                       Point3::new(0.0, 1.0, 0.0),
                       Point3::new(0.0, 0.0, 1.0)];

        let t0 = Tetrahedron::new(&pts, N3Index(0), N3Index(1), N3Index(2), N3Index(3));
        let eles = vec![t0.clone()];

        println!("t0 is {:?}", t0);
        let point_inside = t0.create_center_point(&pts);// Point3::new(0.1, 0.1, 0.1);

        assert_eq!(LocationResult::InElement(T4Index(0)), locate_element_containing(T4Index(0), &eles, &pts, &point_inside));
    }

    #[test]
    fn finding_element_test() {
        let pts = vec![Point3::new(0.0, 0.0, 0.0),
                       Point3::new(1.0, 0.0, 0.0),
                       Point3::new(0.0, 1.0, 0.0),
                       Point3::new(0.0, 0.0, 1.0),
                       Point3::new(0.0, 0.0, -1.0)];

        let t0 = Tetrahedron::new(&pts, N3Index(0), N3Index(1), N3Index(2), N3Index(3));
        let t1 = Tetrahedron::new(&pts, N3Index(0), N3Index(1), N3Index(2), N3Index(4));

        let eles = vec![t0.clone(), t1.clone()];

        let triangulation = Triangulation3::new_from_prebuilt_triangulation(pts.clone(), eles);

        let center0 = t0.create_center_point(&pts);
        let center1 = t1.create_center_point(&pts);

        assert_eq! (LocationResult::InElement(T4Index(0)), locate_element_containing(T4Index(0), triangulation.elements(), triangulation.nodes(), &center0));
        assert_eq!(LocationResult::InElement(T4Index(1)), locate_element_containing(T4Index(0), triangulation.elements(), triangulation.nodes(), &center1));

        assert_eq! (LocationResult::InElement(T4Index(0)), locate_element_containing(T4Index(1), triangulation.elements(), triangulation.nodes(), &center0));
        assert_eq!(LocationResult::InElement(T4Index(1)), locate_element_containing(T4Index(1), triangulation.elements(), triangulation.nodes(), &center1));

        //todo finish testing this.

        //assert_eq! (LocationResult::OnEdge(T3Index(0), 2usize), locate_element_containing(T3Index(0),triangulation.elements(), triangulation.nodes(), &Point2::new(0.5, 0.)));
    }
}