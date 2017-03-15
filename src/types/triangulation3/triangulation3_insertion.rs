use types::triangulation3::Triangulation3;
use types::N3Index;
use types::T4Index;

pub fn insert_into_element(triangulation : &mut Triangulation3, element_index : T4Index, new_node_index : N3Index) -> (T4Index,T4Index,T4Index, T4Index) {
    let index_of_bottom = element_index;
    let index_of_cw1 = T4Index(triangulation.elements().len());
    let index_of_cw2 = T4Index(triangulation.elements().len() + 1);
    let index_of_cw3 = T4Index(triangulation.elements().len() + 2);


    (index_of_bottom, index_of_cw1, index_of_cw2, index_of_cw3)
}

#[cfg(test)]
mod tests {
    #[test]
    fn testing() {

    }
}