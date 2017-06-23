use types::Point3;
use types::Tetrahedron;
use types::N3Index;
use types::T4Index;
use algorithms3::element_locators::*;
use super::triangulation3_insertion;
use super::triangulation3_bw_insertion;
use super::triangulation3_utilities::find_corner_nodes3;
use super::triangulation3_initiation::create_initial_tetra_set;
use super::triangulation3_fake_nodes::*;

use super::triangulation3_neighborhood::Triangulation3Neighborhood;

pub struct Triangulation3 {
    nodes: Vec<Point3>,
    elements: Vec<Tetrahedron>,
    last_added_element_index: T4Index,
}

impl Triangulation3 {
    #[inline]
    pub fn new_from_prebuilt_triangulation(nodes: Vec<Point3>,
                                           elements: Vec<Tetrahedron>)
                                           -> Triangulation3 {
        let mut tr = Triangulation3 {
            nodes: nodes,
            elements: elements,
            last_added_element_index: T4Index(0),
        };

        Triangulation3Neighborhood::teach_triangles_of_neighborhood(&mut tr.elements);
        tr
    }

    #[inline]
    pub fn new(nodes: &[Point3]) -> Triangulation3 {
        println!("got here.");
        let mut nodes = Vec::from(nodes);
        let fake_indices = add_fake_nodes(&mut nodes);
        let corner_nodes: [usize; 8] = find_corner_nodes3(&nodes);
        let mut indices_except_corner: Vec<usize> = Vec::new();

        for i in 0..nodes.len() {
            if !corner_nodes.iter().any(|n| *n == i) {
                indices_except_corner.push(i);
            }
        }

        indices_except_corner.sort_by(|a, b| if nodes[*a].x < nodes[*b].x {
                                          ::std::cmp::Ordering::Less
                                      } else if nodes[*a].x > nodes[*b].x {
            ::std::cmp::Ordering::Greater
        } else {
            if nodes[*a].y < nodes[*b].y {
                ::std::cmp::Ordering::Less
            } else if nodes[*a].y > nodes[*b].y {
                ::std::cmp::Ordering::Greater
            } else {
                if nodes[*a].z < nodes[*b].z {
                    ::std::cmp::Ordering::Less
                } else if nodes[*a].z > nodes[*b].z {
                    ::std::cmp::Ordering::Greater
                } else {
                    panic!("Triangulation received equal nodes. node a: {:?}, node b: {:?}", nodes[*a], nodes[*b]);
                }
            }
        });

        let nodes = Vec::from(nodes);
        let mut eles = create_initial_tetra_set(&corner_nodes, &nodes);

        Triangulation3Neighborhood::teach_triangles_of_neighborhood(&mut eles);
        let mut triangulation = Triangulation3 {
            elements: eles,
            last_added_element_index: T4Index(0),
            nodes: nodes,
        };

        let mut x = 0 ;
        for index in indices_except_corner.into_iter() {
            println!("Invokking that... {:?}", x);
            if x == 600 {
                break;
            }
            x += 1;
            triangulation.insert_into_triangulation(N3Index(index));
        }

//        remove_fake_nodes(&mut triangulation, &fake_indices);

        triangulation
    }

    #[inline]
    pub fn nodes(&self) -> &Vec<Point3> {
        &self.nodes
    }

    #[inline]
    pub fn nodes_mut(&mut self) -> &mut Vec<Point3> {
        &mut self.nodes
    }

    #[inline]
    pub fn elements(&self) -> &Vec<Tetrahedron> {
        &self.elements
    }

    #[inline]
    pub fn elements_mut(&mut self) -> &mut Vec<Tetrahedron> {
        &mut self.elements
    }

    #[inline]
    pub fn insert_node(&mut self, p: &Point3) {
        self.nodes.push(*p);
        let new_node_index = N3Index(self.nodes.len() - 1);

        println!("new node index is: {:?} {:?} len now is:",
                 new_node_index,
                 self.nodes.len());
        self.insert_into_triangulation(new_node_index);
    }

    #[inline]
    fn insert_into_triangulation(&mut self, new_node_index: N3Index) {
        let location_result = locate_element_containing(self.last_added_element_index,
                                                        &self.elements,
                                                        &self.nodes,
                                                        &self.nodes[new_node_index.0]);

        match location_result {
            LocationResult::InElement(ele_index) => {
                self.last_added_element_index = ele_index;
                triangulation3_bw_insertion::insert_into_element_bw(self,
                                                                    ele_index,
                                                                    new_node_index);
                //let (t1_index, t2_index, t3_index, t4_index) =
                //triangulation3_insertion::insert_into_element(self, ele_index, new_node_index);
                //lawson_flipping::propagating_flip(self, new_node_index, t1_index);
                //lawson_flipping::propagating_flip(self, new_node_index, t2_index);
                //lawson_flipping::propagating_flip(self, new_node_index, t3_index);
            }
            LocationResult::OnFace(ele_index, _) => {
                self.last_added_element_index = ele_index;
                triangulation3_bw_insertion::insert_into_element_bw(self,
                                                                    ele_index,
                                                                    new_node_index);
            }
            LocationResult::OnFaces(ele_index, _, _) => {
                self.last_added_element_index = ele_index;
                triangulation3_bw_insertion::insert_into_element_bw(self,
                                                                    ele_index,
                                                                    new_node_index);
            }
        }
    }
}
