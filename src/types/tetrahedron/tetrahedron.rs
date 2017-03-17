use math::*;
use cgmath::InnerSpace;
use types::Point3;
use types::N3Index;
use types::T4Index;

use algorithms3::sort_3::sort_3;

use super::tetrahedron_order::is_ordered_correctly;

#[derive(Debug, PartialEq, Clone)]
pub struct Tetrahedron {
    v: [N3Index; 4],

    n: [Option<T4Index>; 4],
}

impl Tetrahedron {
    #[inline]
    pub fn new(points: &[Point3], a: N3Index, b: N3Index, c: N3Index, d: N3Index) -> Tetrahedron {
        if is_ordered_correctly(&points[a.0], &points[b.0], &points[c.0], &points[d.0]) {
            Tetrahedron {
                v: [a, b, c, d],
                n: [None, None, None, None],
            }
        } else {
            Tetrahedron {
                v: [a, d, c, b],
                n: [None, None, None, None],
            }
        }
    }

    #[inline]
    pub fn new_exact(v: [N3Index; 4], n: [Option<T4Index>; 4]) -> Tetrahedron {
        Tetrahedron { v: v, n: n }
    }

    #[inline]
    pub fn a<'a>(&self, points: &'a [Point3]) -> &'a Point3 {
        &points[self.v[0].0]
    }

    #[inline]
    pub fn b<'a>(&self, points: &'a [Point3]) -> &'a Point3 {
        &points[self.v[1].0]
    }

    #[inline]
    pub fn c<'a>(&self, points: &'a [Point3]) -> &'a Point3 {
        &points[self.v[2].0]
    }

    #[inline]
    pub fn d<'a>(&self, points: &'a [Point3]) -> &'a Point3 {
        &points[self.v[3].0]
    }

    #[inline]
    pub fn index_a(&self) -> N3Index {
        self.v[0]
    }

    #[inline]
    pub fn index_b(&self) -> N3Index {
        self.v[1]
    }

    #[inline]
    pub fn index_c(&self) -> N3Index {
        self.v[2]
    }

    #[inline]
    pub fn index_d(&self) -> N3Index {
        self.v[3]
    }

    #[inline]
    pub fn nodes(&self) -> &[N3Index; 4] {
        &self.v
    }

    #[inline]
    pub fn neighbors(&self) -> &[Option<T4Index>; 4] {
        &self.n
    }

    #[inline]
    pub fn faces_as_indices_tuples(&self) -> [(N3Index, N3Index, N3Index); 4] {
        [(self.index_a(), self.index_b(), self.index_c()),
         (self.index_b(), self.index_a(), self.index_d()),
         (self.index_d(), self.index_c(), self.index_b()),
         (self.index_d(), self.index_a(), self.index_c())]
    }

    #[inline]
    pub fn faces_as_points_tuples<'a>(&self,
                                      points: &'a [Point3])
                                      -> [(&'a Point3, &'a Point3, &'a Point3); 4] {
        [(self.a(points), self.b(points), self.c(points)),
         (self.b(points), self.a(points), self.d(points)),
         (self.d(points), self.c(points), self.b(points)),
         (self.d(points), self.a(points), self.c(points))]
    }

    #[inline]
    pub fn is_made_of(&self, nodes: [N3Index; 4]) -> bool {
        for n3_index in nodes.iter() {
            let mut found = false;

            for i in 0..self.v.len() {
                if self.v[i] == *n3_index {
                    found = true;
                    break;
                }
            }

            if found != true {
                return false;
            }
        }

        return true;
    }

    #[inline]
    pub fn get_neighbor_index(&self, n1: N3Index, n2: N3Index, n3: N3Index) -> usize {
        //TODO this could all be eliminated if the elements and neighboring were ordered correctly,
        let sorted_input = sort_3(n1, n2, n3);
        for edge_index in 0..self.faces_as_indices_tuples().len() {
            let edge = self.faces_as_indices_tuples()[edge_index];
            let sorted_edge = sort_3(edge.0, edge.1, edge.2);

            if sorted_edge.0 == sorted_input.0 && sorted_edge.1 == sorted_input.1 &&
               sorted_edge.2 == sorted_input.2 {
                return edge_index;
            }
        }
        panic!("get_neighbor_index invoked with indices not belonging to this element. \
            n1: '{:?}' n2: '{:?}' n3: '{:?}' self.v '{:?}'",
               n1,
               n2,
               n3,
               self.v);
    }

    #[inline]
    pub fn get_neighbor_for_indices(&self,
                                    n1: N3Index,
                                    n2: N3Index,
                                    n3: N3Index)
                                    -> Option<T4Index> {
        self.n[self.get_neighbor_index(n1, n2, n3)]
    }

    #[inline]
    pub fn get_neighbor_from_index(&self, index: usize) -> Option<T4Index> {
        self.n[index]
    }

    #[inline]
    pub fn set_neighbor(&mut self, index: usize, neighbor: Option<T4Index>) {
        self.n[index] = neighbor;
    }

    #[inline]
    pub fn update_neighbor(&mut self,
                           n1: N3Index,
                           n2: N3Index,
                           n3: N3Index,
                           update_with: Option<T4Index>) {
        let neighbor_index = self.get_neighbor_index(n1, n2, n3);
        self.set_neighbor(neighbor_index, update_with);
    }

    #[inline]
    pub fn create_center_point(&self, nodes: &[Point3]) -> Point3 {
        let a = self.a(nodes);
        let b = self.b(nodes);
        let c = self.c(nodes);
        let d = self.d(nodes);

        Point3::new((a.x + b.x + c.x + d.x) / 4.,
                    (a.y + b.y + c.y + d.y) / 4.,
                    (a.z + b.z + c.z + d.z) / 4.)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::Tetrahedron;

    #[test]
    fn points_are_put_in_clockwise_order() {
        let points = vec![Point3::new(0., 0., 0.),
                          Point3::new(100., 0., 0.),
                          Point3::new(0., 100., 0.),
                          Point3::new(0., 0., 100.)];

        let tr = Tetrahedron::new(&points, N3Index(0), N3Index(1), N3Index(2), N3Index(3));

        assert_eq!(*tr.a(&points), points[0]);
        assert_eq!(*tr.b(&points), points[3]);
        assert_eq!(*tr.c(&points), points[2]);
        assert_eq!(*tr.d(&points), points[1]);

        let correctly_ordered =
            Tetrahedron::new(&points, N3Index(0), N3Index(3), N3Index(2), N3Index(1));

        assert_eq!(*correctly_ordered.a(&points), points[0]);
        assert_eq!(*correctly_ordered.b(&points), points[3]);
        assert_eq!(*correctly_ordered.c(&points), points[2]);
        assert_eq!(*correctly_ordered.d(&points), points[1]);
    }

    #[test]
    fn get_neighbor_index_test() {
        let points = vec![Point3::new(0., 0., 0.),
                          Point3::new(100., 0., 0.),
                          Point3::new(0., 100., 0.),
                          Point3::new(0., 0., 100.)];

        let tr = Tetrahedron::new(&points, N3Index(0), N3Index(1), N3Index(2), N3Index(3));

        assert_eq!(0, tr.get_neighbor_index(N3Index(0), N3Index(3), N3Index(2)));
        assert_eq!(0, tr.get_neighbor_index(N3Index(0), N3Index(2), N3Index(3)));
        assert_eq!(0, tr.get_neighbor_index(N3Index(3), N3Index(0), N3Index(2)));
        assert_eq!(0, tr.get_neighbor_index(N3Index(3), N3Index(2), N3Index(0)));
        assert_eq!(0, tr.get_neighbor_index(N3Index(2), N3Index(0), N3Index(3)));
        assert_eq!(0, tr.get_neighbor_index(N3Index(2), N3Index(3), N3Index(0)));
    }
}
