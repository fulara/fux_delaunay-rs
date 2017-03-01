use super::Triangulation2;

use types::*;

pub fn insert_into_element(triangulation: &mut Triangulation2, element_index: T3Index, new_node_index: N2Index) -> (T3Index, T3Index, T3Index) {
    //left and top will be created. original elements becomes right element.
    let index_of_left = T3Index(triangulation.elements().len());
    let index_of_top = T3Index(triangulation.elements().len() + 1);

    let index_of_right = element_index;


    let (original_elements_nodes, original_element_neighbors) =
        {
            let original_element: &Triangle = &triangulation.elements()[element_index.0];
            (*original_element.nodes(), *original_element.neighbors())
        };

    let left_element = Triangle::new(triangulation.nodes(), original_elements_nodes[0], original_elements_nodes[1], new_node_index);
    let top_element = Triangle::new(triangulation.nodes(), original_elements_nodes[1], original_elements_nodes[2], new_node_index);

    assert_eq!(*left_element.nodes(), [original_elements_nodes[0], original_elements_nodes[1], new_node_index]);
    //println!(" before assert: or_0{:?} or_1{:?} or_2{:?}  new_node{:?}", triangulation.nodes()[original_elements_nodes[0].0], triangulation.nodes()[original_elements_nodes[1].0], triangulation.nodes()[original_elements_nodes[2].0], triangulation.nodes()[new_node_index.0], );
    assert_eq!(*top_element.nodes(), [original_elements_nodes[1], original_elements_nodes[2], new_node_index]);

    update_neighborhood(triangulation, original_element_neighbors[0], original_elements_nodes[0], original_elements_nodes[1], index_of_left);
    update_neighborhood(triangulation, original_element_neighbors[1], original_elements_nodes[1], original_elements_nodes[2], index_of_top);

    triangulation.elements_mut().push(left_element);
    triangulation.elements_mut().push(top_element);

    set_neighbors(&mut triangulation.elements_mut()[index_of_left.0], [original_element_neighbors[0], Some(index_of_top), Some(index_of_right)]);
    set_neighbors(&mut triangulation.elements_mut()[index_of_top.0], [original_element_neighbors[1], Some(index_of_right), Some(index_of_left)]);

    //updated original element. first update neighbors.
    let original_element: &mut Triangle = &mut triangulation.elements_mut()[index_of_right.0];
    set_neighbors(original_element, [original_element_neighbors[2], Some(index_of_left), Some(index_of_top)]);

    original_element.update_nodes(original_elements_nodes[2], original_elements_nodes[0], new_node_index);

    (index_of_right, index_of_top, index_of_left)
}

fn set_neighbors(element: &mut Triangle, n: [Option<T3Index>; 3]) {
    for i in 0..3 {
        element.set_neighbor(i, n[i])
    }
}

fn update_neighborhood(triangulation: &mut Triangulation2, for_index: Option<T3Index>, n1: N2Index, n2: N2Index, update_with: T3Index) {
    if let Some(updated_element_index) = for_index {
        let updated_element: &mut Triangle = &mut triangulation.elements_mut()[updated_element_index.0];
        updated_element.update_neighbor(n1, n2, Some(update_with));
    }
}

//this can only be invoked in cases when that element does not have neighbor on the edge.
pub fn insert_in_edge(triangulation: &mut Triangulation2, element_index: T3Index, inserted_node_index: N2Index, edge_index: usize) -> (T3Index, T3Index) {
    let new_element_index = T3Index(triangulation.elements().len());
    //assumption is that the node is inserted in the middle between first and second nodes.
    let (second_node_index, third_node_index,second_neighbor_index) = {
        let element: &Triangle = &triangulation.elements()[element_index.0];
        let first_second_nodes = element.get_edge(edge_index);
        let last_node = element.get_other_last_node(first_second_nodes.0, first_second_nodes.1);
        let second_neighbor = element.get_neighor_for_nodes(first_second_nodes.1, last_node);
        (first_second_nodes.1, last_node, second_neighbor)
    };

    let mut new_element = Triangle::new(triangulation.nodes(), inserted_node_index, third_node_index, second_node_index);

    new_element.update_neighbor(inserted_node_index, third_node_index, Some(element_index));
    new_element.update_neighbor(second_node_index, third_node_index, second_neighbor_index);

    {
        let element: &mut Triangle = &mut triangulation.elements_mut()[element_index.0];

        element.update_neighbor(second_node_index, third_node_index, Some(new_element_index));
        element.swap_node(second_node_index, inserted_node_index);
    }

    triangulation.elements_mut().push(new_element);

    update_neighborhood(triangulation, second_neighbor_index, second_node_index, third_node_index, new_element_index);

    (element_index, new_element_index)
}

#[cfg(test)]
mod tests {
    use ::types::*;
    use super::*;

    #[test]
    fn insertion_in_element_test() {
        let nodes: Vec<Point2> = vec!(Point2::new(2.5, 5.),
                                      Point2::new(3.5, 6.5),
                                      Point2::new(2., 6.5),
                                      Point2::new(3.5, 5.5),
                                      Point2::new(3.0, 7.5),
                                      Point2::new(1.5, 5.),
                                      Point2::new(2.5, 6.));

        let eles: Vec<Triangle> = vec!(Triangle::new(&nodes, N2Index(0), N2Index(2), N2Index(1)),
                                       Triangle::new(&nodes, N2Index(0), N2Index(1), N2Index(3)),
                                       Triangle::new(&nodes, N2Index(2), N2Index(4), N2Index(1)),
                                       Triangle::new(&nodes, N2Index(5), N2Index(2), N2Index(0)));

        let mut triangulation = Triangulation2::new_from_prebuilt_triangulation(nodes, eles);

        insert_into_element(&mut triangulation, T3Index(0), N2Index(6));


        assert_eq!(6, triangulation.elements().len());
        assert_eq!(Triangle::new_exact([N2Index(0), N2Index(2), N2Index(6)], [Some(T3Index(3)), Some(T3Index(5)), Some(T3Index(0))]), triangulation.elements()[4]);
        assert_eq!(Triangle::new_exact([N2Index(2), N2Index(1), N2Index(6)], [Some(T3Index(2)), Some(T3Index(0)), Some(T3Index(4))]), triangulation.elements()[5]);
        assert_eq!(Triangle::new_exact([N2Index(1), N2Index(0), N2Index(6)], [Some(T3Index(1)), Some(T3Index(4)), Some(T3Index(5))]), triangulation.elements()[0]);
    }

    #[test]
    fn insertion_in_edge_test() {
        let nodes: Vec<Point2> = vec!(Point2::new(0., 0.),
                                      Point2::new(1., 1.),
                                      Point2::new(2., 0.),
                                      Point2::new(0., 1.),
                                      Point2::new(2., 1.),
                                      Point2::new(1., 0.));

        let eles: Vec<Triangle> = vec!(Triangle::new(&nodes, N2Index(0), N2Index(1), N2Index(2)),
                                       Triangle::new(&nodes, N2Index(0), N2Index(3), N2Index(1)),
                                       Triangle::new(&nodes, N2Index(1), N2Index(4), N2Index(2))
        );

        let mut triangulation = Triangulation2::new_from_prebuilt_triangulation(nodes, eles);

        assert_eq!(3, triangulation.elements().len());
        assert_eq!(Triangle::new_exact([N2Index(0), N2Index(1), N2Index(2)], [Some(T3Index(1)), Some(T3Index(2)), None]), triangulation.elements()[0]);
        assert_eq!(Triangle::new_exact([N2Index(0), N2Index(3), N2Index(1)], [None, None, Some(T3Index(0))]), triangulation.elements()[1]);
        assert_eq!(Triangle::new_exact([N2Index(1), N2Index(4), N2Index(2)], [None, None, Some(T3Index(0))]), triangulation.elements()[2]);

        insert_in_edge(&mut triangulation, T3Index(0), N2Index(5), 2);
        assert_eq!(4, triangulation.elements().len());
        assert_eq!(Triangle::new_exact([N2Index(5), N2Index(1), N2Index(2)], [Some(T3Index(3)), Some(T3Index(2)), None]), triangulation.elements()[0]);
        assert_eq!(Triangle::new_exact([N2Index(0), N2Index(3), N2Index(1)], [None, None, Some(T3Index(3))]), triangulation.elements()[1]);
        assert_eq!(Triangle::new_exact([N2Index(1), N2Index(4), N2Index(2)], [None, None, Some(T3Index(0))]), triangulation.elements()[2]);
        assert_eq!(Triangle::new_exact([N2Index(5), N2Index(0), N2Index(1)], [None,Some(T3Index(1)), Some(T3Index(0))]), triangulation.elements()[3]);
    }
}