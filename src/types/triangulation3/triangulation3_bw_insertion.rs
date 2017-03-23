use types::Triangulation3;
use types::T4Index;
use types::N3Index;
use math::SphereSide;

pub fn insert_into_element(triangulation: &mut Triangulation3,
                           element_index: T4Index,
                           new_node_index: N3Index) {
    //let elements_to_remove;
}

fn find(tr: &Triangulation3, starting_element: T4Index, node: N3Index) {
    assert_eq!(SphereSide::Inside,
               tr.elements()[starting_element.0].is_point_in_circumsphere(&tr.nodes()[node.0],
                                                                          tr.nodes()));
}

#[cfg(test)]
mod tests {
    #[test]
    fn tests() {}
}
