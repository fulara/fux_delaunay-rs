use types::*;

pub fn try_flip(triangulation: &mut Triangulation2,
                bottom_node_index: N2Index,
                bottom_element_index: T3Index)
                -> Option<(T3Index, T3Index)> {
    let (common1, common2, top_element_index) = {
        let tr: &Triangle = &triangulation.elements()[bottom_element_index.0];
        let (left_common_node, right_common_node) = tr.get_others_two_nodes(bottom_node_index);
        let neighbor_opt = tr.get_neighor_for_nodes(left_common_node, right_common_node);
        if neighbor_opt.is_none() {
            return None;
        }

        (left_common_node, right_common_node, neighbor_opt.unwrap())
    };

    {
        let node = &triangulation.nodes()[bottom_node_index.0];
        let neighbor = &triangulation.elements()[top_element_index.0];
        if !triangulation.is_inside_circumcircle(neighbor, node) {
            return None;
        }
    }

    let top_node_index = {
        let neighbor: &Triangle = &triangulation.elements()[top_element_index.0];
        neighbor.get_other_last_node(common1, common2)
    };

    let (c1_bottom_neighbor_index, c2_top_neighbor_index) = {
        let top_triangle: &Triangle = &triangulation.elements()[top_element_index.0];
        let bottom_triangle: &Triangle = &triangulation.elements()[bottom_element_index.0];

        (bottom_triangle.get_neighor_for_nodes(common1, bottom_node_index),
         top_triangle.get_neighor_for_nodes(common2, top_node_index))
    };

    perform_swap_update_connections(triangulation,
                                    bottom_element_index,
                                    top_element_index,
                                    c2_top_neighbor_index,
                                    common1,
                                    common2,
                                    bottom_node_index,
                                    top_node_index);
    perform_swap_update_connections(triangulation,
                                    top_element_index,
                                    bottom_element_index,
                                    c1_bottom_neighbor_index,
                                    common2,
                                    common1,
                                    top_node_index,
                                    bottom_node_index);

    Some((top_element_index, bottom_element_index))
}

pub fn propagating_flip(triangulation: &mut Triangulation2,
                        bottom_node_index: N2Index,
                        bottom_element_index: T3Index) {
    if let Some((left_ele, right_ele)) =
        try_flip(triangulation, bottom_node_index, bottom_element_index) {
        propagating_flip(triangulation, bottom_node_index, left_ele);
        propagating_flip(triangulation, bottom_node_index, right_ele);
    }
}

fn perform_swap_update_connections(triangulation: &mut Triangulation2,
                                   element_to_swap_index: T3Index,
                                   element_swapping_with: T3Index,
                                   changing_neighborhood_element_index: Option<T3Index>,
                                   common_node_being_swapped_out: N2Index,
                                   common_node: N2Index,
                                   last_element_node_index: N2Index,
                                   node_being_swapped_in: N2Index) {
    {
        let element_being_swapped: &mut Triangle = &mut triangulation.elements_mut()
                                                            [element_to_swap_index.0];

        element_being_swapped.update_neighbor(common_node_being_swapped_out,
                                              common_node,
                                              changing_neighborhood_element_index);
        element_being_swapped.update_neighbor(last_element_node_index,
                                              common_node_being_swapped_out,
                                              Some(element_swapping_with));

        element_being_swapped.swap_node(common_node_being_swapped_out, node_being_swapped_in);
    }
    {
        if let Some(changing_neighborhood_element_index) = changing_neighborhood_element_index {
            let changing_neighborhood_element: &mut Triangle =
                &mut triangulation.elements_mut()[changing_neighborhood_element_index.0];
            changing_neighborhood_element.update_neighbor(node_being_swapped_in,
                                                          common_node,
                                                          Some(element_to_swap_index));
        }
    }
    let element_being_swapped: &Triangle = &triangulation.elements()[element_to_swap_index.0];
    element_being_swapped.assert_order(triangulation.nodes());
}

#[cfg(test)]
mod tests {
    use types::*;
    use super::*;

    #[test]
    fn flip_attempt_when_triangle_has_no_neighbors() {
        let points: Vec<Point2> = vec![Point2::new(2.5, 5.),
                                       Point2::new(3.5, 6.5),
                                       Point2::new(2., 6.5),
                                       Point2::new(3.5, 5.5)];
        let eles: Vec<Triangle> = vec![Triangle::new(&points, N2Index(0), N2Index(1), N2Index(3))];

        let mut triangulation = Triangulation2::new_from_prebuilt_triangulation(points, eles);
        let x = try_flip(&mut triangulation, N2Index(3), T3Index(0));

        assert_eq!(None, x);
    }

    #[test]
    fn flip_attempt_when_there_is_no_need_for_a_flip() {
        let points: Vec<Point2> = vec![Point2::new(2.5, 5.),
                                       Point2::new(3.5, 6.5),
                                       Point2::new(2., 6.5),
                                       Point2::new(4., 5.5)];
        let eles: Vec<Triangle> = vec![Triangle::new(&points, N2Index(0), N2Index(1), N2Index(3))];

        let mut triangulation = Triangulation2::new_from_prebuilt_triangulation(points, eles);
        let x = try_flip(&mut triangulation, N2Index(3), T3Index(0));

        assert_eq!(None, x);
    }

    #[test]
    fn simple_flip_test() {
        let points: Vec<Point2> = vec![Point2::new(2.5, 5.),
                                       Point2::new(3.5, 6.5),
                                       Point2::new(2., 6.5),
                                       Point2::new(3.5, 5.5),
                                       Point2::new(3.0, 7.5),
                                       Point2::new(1.5, 5.),
                                       Point2::new(3.5, 4.5),
                                       Point2::new(4.5, 6.0)];
        let eles: Vec<Triangle> = vec![Triangle::new(&points, N2Index(0), N2Index(2), N2Index(1)),
                                       Triangle::new(&points, N2Index(0), N2Index(1), N2Index(3)),
                                       Triangle::new(&points, N2Index(2), N2Index(4), N2Index(1)),
                                       Triangle::new(&points, N2Index(5), N2Index(2), N2Index(0)),
                                       Triangle::new(&points, N2Index(0), N2Index(3), N2Index(6)),
                                       Triangle::new(&points, N2Index(3), N2Index(1), N2Index(7))];

        let mut triangulation = Triangulation2::new_from_prebuilt_triangulation(points, eles);

        assert_eq!(Triangle::new_exact([N2Index(0), N2Index(2), N2Index(1)],
                                       [Some(T3Index(3)), Some(T3Index(2)), Some(T3Index(1))]),
                   triangulation.elements()[0]);
        assert_eq!(Triangle::new_exact([N2Index(0), N2Index(1), N2Index(3)],
                                       [Some(T3Index(0)), Some(T3Index(5)), Some(T3Index(4))]),
                   triangulation.elements()[1]);


        let x = try_flip(&mut triangulation, N2Index(3), T3Index(1));

        assert_eq!(Some((T3Index(0), T3Index(1))), x);

        assert_eq!(Triangle::new_exact([N2Index(0), N2Index(2), N2Index(3)],
                                       [Some(T3Index(3)), Some(T3Index(1)), Some(T3Index(4))]),
                   triangulation.elements()[0]);
        assert_eq!(Triangle::new_exact([N2Index(2), N2Index(1), N2Index(3)],
                                       [Some(T3Index(2)), Some(T3Index(5)), Some(T3Index(0))]),
                   triangulation.elements()[1]);
        assert_eq!(Triangle::new_exact([N2Index(2), N2Index(4), N2Index(1)],
                                       [None, None, Some(T3Index(1))]),
                   triangulation.elements()[2]);
        assert_eq!(Triangle::new_exact([N2Index(5), N2Index(2), N2Index(0)],
                                       [None, Some(T3Index(0)), None]),
                   triangulation.elements()[3]);
        assert_eq!(Triangle::new_exact([N2Index(0), N2Index(3), N2Index(6)],
                                       [Some(T3Index(0)), None, None]),
                   triangulation.elements()[4]);
        assert_eq!(Triangle::new_exact([N2Index(3), N2Index(1), N2Index(7)],
                                       [Some(T3Index(1)), None, None]),
                   triangulation.elements()[5]);
    }
}
