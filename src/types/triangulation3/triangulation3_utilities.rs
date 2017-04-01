use types::Point3;

pub fn find_corner_nodes3(nodes: &[Point3])
                          -> [usize;8] {
    let (mut top_left_front_index,
         mut top_right_front_index,
         mut bottom_left_front_index,
         mut bottom_right_front_index,
         mut top_left_back_index,
         mut top_right_back_index,
         mut bottom_left_back_index,
         mut bottom_right_back_index) =
        (0usize, 0usize, 0usize, 0usize, 0usize, 0usize, 0usize, 0usize);

    for (index, candidate) in nodes.iter().enumerate() {
        let current_top_left_front = nodes[top_left_front_index];
        if current_top_left_front.x > candidate.x || current_top_left_front.y > candidate.y ||
           current_top_left_front.z > candidate.z {
            top_left_front_index = index;
        }

        let current_top_right_front = nodes[top_right_front_index];
        if current_top_right_front.x > candidate.x || current_top_right_front.y < candidate.y ||
           current_top_left_front.z > candidate.z {
            top_right_front_index = index;
        }

        let current_bottom_left_front = nodes[bottom_left_front_index];
        if current_bottom_left_front.x < candidate.x || current_bottom_left_front.y < candidate.y ||
           current_top_left_front.z > candidate.z {
            bottom_left_front_index = index;
        }

        let current_bottom_right_front = nodes[bottom_right_front_index];
        if current_bottom_right_front.x < candidate.x ||
           current_bottom_right_front.y > candidate.y ||
           current_top_left_front.z > candidate.z {
            bottom_right_front_index = index;
        }

        let current_top_left_back = nodes[top_left_back_index];
        if current_top_left_back.x > candidate.x || current_top_left_back.y > candidate.y ||
            current_top_left_back.z < candidate.z {
            top_left_back_index = index;
        }

        let current_top_right_back = nodes[top_right_back_index];
        if current_top_right_back.x > candidate.x || current_top_right_back.y < candidate.y ||
            current_top_left_back.z < candidate.z {
            top_right_back_index = index;
        }

        let current_bottom_left_back = nodes[bottom_left_back_index];
        if current_bottom_left_back.x < candidate.x || current_bottom_left_back.y < candidate.y ||
            current_top_left_back.z < candidate.z {
            bottom_left_back_index = index;
        }

        let current_bottom_right_back = nodes[bottom_right_back_index];
        if current_bottom_right_back.x < candidate.x ||
            current_bottom_right_back.y > candidate.y ||
            current_top_left_back.z < candidate.z {
            bottom_right_back_index = index;
        }
    }

    [top_left_front_index,
     top_right_front_index,
     bottom_left_front_index,
     bottom_right_front_index,
     top_left_back_index,
     top_right_back_index,
     bottom_left_back_index,
     bottom_right_back_index]
}

#[cfg(test)]
mod triangulation3_utilities_test {
    use super::*;
    use types::Point3;

    #[test]
    fn test() {
        let nodes =  super::super::triangulation3_test_utils::get_example_initial_point_set();

        assert_eq!([0, 1, 2, 3, 4, 5, 6, 7], find_corner_nodes3(&nodes));
    }
}
