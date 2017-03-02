use math::*;
use cgmath::InnerSpace;
use types::Point3;
use types::N3Index;
use types::T4Index;
use super::tetrahedron_order::is_ordered_correctly;

#[derive(Debug, PartialEq)]
pub struct Tetrahedron {
    v: [N3Index; 4],

    n: [Option<T4Index>; 4]
}

impl Tetrahedron {
    #[inline]
    pub fn new(points: &Vec<Point3>, a: N3Index, b: N3Index, c: N3Index, d: N3Index) -> Tetrahedron {
        if is_ordered_correctly(&points[a.0], &points[b.0], &points[c.0], &points[d.0]) {
            Tetrahedron { v: [a, b, c, d], n: [None, None, None, None] }
        } else {
            Tetrahedron { v: [a, d, c, b], n: [None, None, None, None] }
        }
    }

    #[inline]
    pub fn new_exact(v: [N3Index; 4], n: [Option<T4Index>; 4]) -> Tetrahedron {
        Tetrahedron {
            v: v,
            n: n,
        }
    }

    #[inline]
    pub fn a<'a>(&self, points: &'a Vec<Point3>) -> &'a Point3 {
        &points[self.v[0].0]
    }

    #[inline]
    pub fn b<'a>(&self, points: &'a Vec<Point3>) -> &'a Point3 {
        &points[self.v[1].0]
    }

    #[inline]
    pub fn c<'a>(&self, points: &'a Vec<Point3>) -> &'a Point3 {
        &points[self.v[2].0]
    }

    #[inline]
    pub fn d<'a>(&self, points: &'a Vec<Point3>) -> &'a Point3 {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::Tetrahedron;

    #[test]
    fn points_are_put_in_clockwise_order() {
        let points = vec![Point3::new(0., 0., 0.), Point3::new(100., 0., 0.), Point3::new(0., 100., 0.), Point3::new(0., 0., 100.)];

        let tr = Tetrahedron::new(&points, N3Index(0), N3Index(1), N3Index(2), N3Index(3));

        assert_eq!(*tr.a(&points), points[0]);
        assert_eq!(*tr.b(&points), points[3]);
        assert_eq!(*tr.c(&points), points[2]);
        assert_eq!(*tr.d(&points), points[1]);

        let correctly_ordered = Tetrahedron::new(&points, N3Index(0), N3Index(3), N3Index(2), N3Index(1));

        assert_eq!(*correctly_ordered.a(&points), points[0]);
        assert_eq!(*correctly_ordered.b(&points), points[3]);
        assert_eq!(*correctly_ordered.c(&points), points[2]);
        assert_eq!(*correctly_ordered.d(&points), points[1]);
    }
}