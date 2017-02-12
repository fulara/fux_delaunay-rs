use ::types::*;
use ::math::*;

struct Flipper {}

impl Flipper {}


pub fn perform_flip(triangulation: &mut Triangulation, bottom_node_index: N2Index, bottom_element_index: T3Index) -> bool {
    let (common1, common2, top_element_index) =
    {
        let tr: &Triangle = &triangulation.elements()[bottom_element_index.0];
        let (left_common_node, right_common_node) = tr.get_others_two_nodes(bottom_node_index);
        let neighbor_opt = tr.get_neighor_for_nodes(left_common_node, right_common_node);
        if neighbor_opt.is_none() {
            return false;
        }

        (left_common_node, right_common_node, neighbor_opt.unwrap())
    };

    {
        let node = &triangulation.nodes()[bottom_node_index.0];
        let neighbor = &triangulation.elements()[top_element_index.0];
        if !triangulation.is_inside_circumcircle(neighbor, node) {
            return false;
        }
    }

    let top_node_index = {
        let neighbor: &Triangle = &triangulation.elements()[top_element_index.0];
        neighbor.get_other_last_node(common1, common2)
    };

    let (c1_bottom_neighbor_index, c2_top_neighbor_index) = {
        let top_triangle: &Triangle = &triangulation.elements()[top_element_index.0];
        let bottom_triangle: &Triangle = &triangulation.elements()[bottom_element_index.0];

        (bottom_triangle.get_neighor_for_nodes(common1, bottom_node_index), top_triangle.get_neighor_for_nodes(common2, top_node_index))
    };

    {
        {
            let bottom_element: &mut Triangle = &mut triangulation.elements_mut()[bottom_element_index.0];

            update_neighbor(bottom_element, common1, common2, c2_top_neighbor_index);
            update_neighbor(bottom_element, bottom_node_index, common1, Some(top_element_index));

            bottom_element.swap_node(common1, top_node_index);
        }
        {
            if let Some(c2_top_neighbor_index) = c2_top_neighbor_index {
                let c2_top_neighbor: &mut Triangle = &mut triangulation.elements_mut()[c2_top_neighbor_index.0];
                update_neighbor(c2_top_neighbor, top_node_index, common2, Some(bottom_element_index));
            }
        }
        let bottom_element: &Triangle = &triangulation.elements()[bottom_element_index.0];
        bottom_element.assert_order(triangulation.nodes());
    }

    {
        {
            let top_element: &mut Triangle = &mut triangulation.elements_mut()[top_element_index.0];

            update_neighbor(top_element, top_node_index, common2, Some(bottom_element_index));
            update_neighbor(top_element, common1, common2, c1_bottom_neighbor_index);

            top_element.swap_node(common2, bottom_node_index);
        }
        {
            if let Some(c1_bottom_neighbor_index) = c1_bottom_neighbor_index {
                let c1_bottom_neighbor: &mut Triangle = &mut triangulation.elements_mut()[c1_bottom_neighbor_index.0];
                update_neighbor(c1_bottom_neighbor, common1, bottom_node_index, Some(top_element_index));
            }
        }
        let top_element: &Triangle = &triangulation.elements()[top_element_index.0];
        top_element.assert_order(triangulation.nodes());
    }

    true
}

fn update_neighbor(for_element: &mut Triangle, n1: N2Index, n2: N2Index, update_with: Option<T3Index>) {
    let neighbor_index = for_element.get_neighbor_index(n1, n2);
    for_element.set_neighbor(neighbor_index, update_with);
}

#[cfg(test)]
mod tests {
    use ::types::*;
    use super::*;

    #[test]
    fn flip_attempt_when_triangle_has_no_neighbors() {
        let points: Vec<Point2> = vec!(Point2::new(2.5, 5.), Point2::new(3.5, 6.5), Point2::new(2., 6.5), Point2::new(3.5, 5.5));
        let eles: Vec<Triangle> = vec!(Triangle::new(&points, N2Index(0), N2Index(1), N2Index(3)));

        let mut triangulation = Triangulation::new_from_prebuilt_triangulation(points, eles);
        let x = perform_flip(&mut triangulation, N2Index(3), T3Index(0));

        assert_eq!(false, x);
    }

    #[test]
    fn flip_attempt_when_there_is_no_need_for_a_flip() {
        let points: Vec<Point2> = vec!(Point2::new(2.5, 5.), Point2::new(3.5, 6.5), Point2::new(2., 6.5), Point2::new(4., 5.5));
        let eles: Vec<Triangle> = vec!(Triangle::new(&points, N2Index(0), N2Index(1), N2Index(3)));

        let mut triangulation = Triangulation::new_from_prebuilt_triangulation(points, eles);
        let x = perform_flip(&mut triangulation, N2Index(3), T3Index(0));

        assert_eq!(false, x);
    }

    #[test]
    fn simple_flip_test() {
        let points: Vec<Point2> = vec!(Point2::new(2.5, 5.),
                                       Point2::new(3.5, 6.5),
                                       Point2::new(2., 6.5),
                                       Point2::new(3.5, 5.5),
                                       Point2::new(3.0, 7.5),
                                       Point2::new(1.5, 5.),
                                       Point2::new(3.5, 4.5),
                                       Point2::new(4.5, 6.0),
        );
        let eles: Vec<Triangle> = vec!(Triangle::new(&points, N2Index(0), N2Index(2), N2Index(1)),
                                       Triangle::new(&points, N2Index(0), N2Index(1), N2Index(3)),
                                       Triangle::new(&points, N2Index(2), N2Index(4), N2Index(1)),
                                       Triangle::new(&points, N2Index(5), N2Index(2), N2Index(0)),
                                       Triangle::new(&points, N2Index(0), N2Index(3), N2Index(6)),
                                       Triangle::new(&points, N2Index(3), N2Index(1), N2Index(7)),
        );

        let mut triangulation = Triangulation::new_from_prebuilt_triangulation(points, eles);

        assert_eq!(Triangle::new_exact([N2Index(0), N2Index(2), N2Index(1)], [Some(T3Index(3)), Some(T3Index(2)), Some(T3Index(1))]), triangulation.elements()[0]);
        assert_eq!(Triangle::new_exact([N2Index(0), N2Index(1), N2Index(3)], [Some(T3Index(0)), Some(T3Index(5)), Some(T3Index(4))]), triangulation.elements()[1]);


        let x = perform_flip(&mut triangulation, N2Index(3), T3Index(1));

        assert_eq!(true, x);

        assert_eq!(Triangle::new_exact([N2Index(0), N2Index(2), N2Index(3)], [Some(T3Index(3)), Some(T3Index(1)), Some(T3Index(4))]), triangulation.elements()[0]);
        assert_eq!(Triangle::new_exact([N2Index(2), N2Index(1), N2Index(3)], [Some(T3Index(2)), Some(T3Index(5)), Some(T3Index(0))]), triangulation.elements()[1]);
        assert_eq!(Triangle::new_exact([N2Index(2), N2Index(4), N2Index(1)], [None, None, Some(T3Index(1))]), triangulation.elements()[2]);
        assert_eq!(Triangle::new_exact([N2Index(5), N2Index(2), N2Index(0)], [None, Some(T3Index(0)), None]), triangulation.elements()[3]);
        assert_eq!(Triangle::new_exact([N2Index(0), N2Index(3), N2Index(6)], [Some(T3Index(0)), None, None]), triangulation.elements()[4]);
        assert_eq!(Triangle::new_exact([N2Index(3), N2Index(1), N2Index(7)], [Some(T3Index(1)), None, None]), triangulation.elements()[5]);

    }
}