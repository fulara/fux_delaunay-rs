use types::Point2;

pub fn find_corner_nodes(nodes: &[Point2]) -> (usize, usize, usize, usize) {
    let (mut top_left_index, mut top_right_index, mut bottom_left_index, mut bottom_right_index) =
        (0usize, 0usize, 0usize, 0usize);

    for i in 1..nodes.len() {
        let candidate = nodes[i];
        let current_top_left = nodes[top_left_index];
        if current_top_left.x > candidate.x || current_top_left.y < candidate.y {
            top_left_index = i;
        }

        let current_top_right = nodes[top_right_index];
        if current_top_right.x < candidate.x || current_top_right.y < candidate.y {
            top_right_index = i;
        }

        let current_bottom_left = nodes[bottom_left_index];
        if current_bottom_left.x > candidate.x || current_bottom_left.y > candidate.y {
            bottom_left_index = i;
        }

        let current_bottom_right = nodes[bottom_right_index];
        if current_bottom_right.x < candidate.x || current_bottom_right.y > candidate.y {
            bottom_right_index = i;
        }
    }

    (top_left_index, top_right_index, bottom_left_index, bottom_right_index)
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::Point2;

    #[test]
    fn test() {
        let nodes = [Point2::new(5., 5.),
                     Point2::new(0., 0.),
                     Point2::new(0., 10.),
                     Point2::new(10., 10.),
                     Point2::new(10., 0.)];

        assert_eq!((2, 3, 1, 4), find_corner_nodes(&nodes));
    }
}
