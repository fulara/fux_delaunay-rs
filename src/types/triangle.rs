use math::*;
use cgmath::InnerSpace;
use types::Point2;
use types::n2_index::N2Index;
use types::t3_index::T3Index;

pub struct Triangle {
    v: [N2Index; 3],

    n: [Option<T3Index>; 3]
}

impl Triangle {
    #[inline]
    pub fn new(points: &Vec<Point2>, a: N2Index, b: N2Index, c: N2Index) -> Triangle {
        if on_which_side_point_lies(&points[a.0], &points[b.0], &points[c.0]) == PointLiesOnSide::Left {
            Triangle { v: [a, c, b], n: [None, None, None] }
        } else {
            Triangle { v: [a, b, c], n: [None, None, None] }
        }
    }

    #[inline]
    pub fn a<'a>(&self, points: &'a Vec<Point2>) -> &'a Point2 {
        &points[self.v[0].0]
    }

    #[inline]
    pub fn b<'a>(&self, points: &'a Vec<Point2>) -> &'a Point2 {
        &points[self.v[1].0]
    }

    #[inline]
    pub fn c<'a>(&self, points: &'a Vec<Point2>) -> &'a Point2 {
        &points[self.v[2].0]
    }

    #[inline]
    pub fn index_a(&self) -> N2Index {
        self.v[0]
    }

    #[inline]
    pub fn index_b(&self) -> N2Index {
        self.v[1]
    }

    #[inline]
    pub fn index_c(&self) -> N2Index {
        self.v[2]
    }

    #[inline]
    pub fn edges_as_points_tuples<'a>(&self, points: &'a Vec<Point2>) -> [(&'a Point2, &'a Point2); 3] {
        [(self.a(points),self.b(points)),(self.b(points),self.c(points)),(self.c(points),self.a(points))]
    }

    #[inline]
    pub fn is_point_inside(&self, points: &Vec<Point2>, p: &Point2) -> bool
    {
        let v0 = self.c(points) - self.a(points);
        let v1 = self.b(points) - self.a(points);
        let v2 = p - self.a(points);

        let dot00 = v0.dot(v0);
        let dot01 = v0.dot(v1);
        let dot02 = v0.dot(v2);
        let dot11 = v1.dot(v1);
        let dot12 = v1.dot(v2);

        let inv_denom = 1. / (dot00 * dot11 - dot01 * dot01);
        let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
        let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;

        (u >= 0.) && (v >= 0.) && (u + v <= 1.)
    }

    #[inline]
    pub fn get_neighbor_index(&self, n1: N2Index, n2: N2Index) -> usize {
        //TODO this could all be eliminated if the elements and neighboring were ordered correctly,
        if n1 == self.v[0] {
            if n2 == self.v[1] {
                return 0;
            } else if n2 == self.v[2] {
                return 2;
            }
        } else if n1 == self.v[1] {
            if n2 == self.v[2] {
                return 1;
            } else if n2 == self.v[0] {
                return 0;
            }
        } else if n1 == self.v[2] {
            if n2 == self.v[0] {
                return 2;
            } else if n2 == self.v[1] {
                return 1;
            }
        }

        panic!("get_neighbor_index invoked with indices not belonging to this element.");
    }

    #[inline]
    pub fn get_neighbor(&self, neighbor_index : usize) -> Option<T3Index> {
        self.n[neighbor_index]
    }

    #[inline]
    pub fn set_neighbor(&mut self, index: usize, neighbor: T3Index) {
        self.n[index] = Some(neighbor);
    }

    #[inline]
    pub fn create_center_point(&self, points: &Vec<Point2>) -> Point2 {
        let a = self.a(points);
        let b = self.b(points);
        let c = self.c(points);

        Point2::new((a.x + b.x + c.x) / 3., (a.y + b.y + c.y) / 3.)
    }

    #[inline]
    pub fn clone(&self) -> Triangle {
        Triangle { v: self.v, n: self.n }
    }
}

#[cfg(test)]
mod triangle {
    use super::*;
    use types::Point2;
    use types::n2_index::N2Index;

    #[test]
    fn abc_get_and_point_order_check() {
        let points = vec![Point2::new(0., 0.), Point2::new(2., 2.), Point2::new(1., 0.)];

        let tr = Triangle::new(&points, N2Index(0), N2Index(1), N2Index(2));
        let tr2 = Triangle::new(&points, N2Index(0), N2Index(2), N2Index(1));

        assert_eq!(*tr.a(&points), Point2::new(0., 0.));
        assert_eq!(*tr.b(&points), Point2::new(2., 2.));
        assert_eq!(*tr.c(&points), Point2::new(1., 0.));

        //triangle::new got different order, and yet the order is the same.
        assert_eq!(*tr2.a(&points), Point2::new(0., 0.));
        assert_eq!(*tr2.b(&points), Point2::new(2., 2.));
        assert_eq!(*tr2.c(&points), Point2::new(1., 0.));
    }

    #[test]
    fn points_are_put_in_clockwise_order() {
        let points = vec![Point2::new(5., 5.), Point2::new(-1., 70.), Point2::new(0., 0.)];

        let tr = Triangle::new(&points, N2Index(0), N2Index(1), N2Index(2));

        assert_eq!(*tr.a(&points), Point2::new(5., 5.));
        assert_eq!(*tr.b(&points), Point2::new(0., 0.));
        assert_eq!(*tr.c(&points), Point2::new(-1., 70.));
    }

    #[test]
    fn is_point_inside() {
        let points = vec![Point2::new(0., 0.), Point2::new(1., 1.), Point2::new(2., 0.)];
        let tr = Triangle::new(&points, N2Index(0), N2Index(1), N2Index(2));

        assert_eq!(true, tr.is_point_inside(&points, &Point2::new(0.5, 0.5)));
        assert_eq!(true, tr.is_point_inside(&points, &Point2::new(0.1, 0.1)));
        assert_eq!(true, tr.is_point_inside(&points, &Point2::new(0.9, 0.9)));
        assert_eq!(true, tr.is_point_inside(&points, &points[0]));
        assert_eq!(true, tr.is_point_inside(&points, &points[1]));
        assert_eq!(true, tr.is_point_inside(&points, &points[2]));
        assert_eq!(true, tr.is_point_inside(&points, &Point2::new(0., 0.)));

        assert_eq!(false, tr.is_point_inside(&points, &Point2::new(0.5, 1.1)));
        assert_eq!(false, tr.is_point_inside(&points, &Point2::new(-0.0000001, 0.)));
        assert_eq!(false, tr.is_point_inside(&points, &Point2::new(1.1, 1.1)));
        assert_eq!(false, tr.is_point_inside(&points, &Point2::new(-0.5, 0.5)));
    }

    #[test]
    fn get_neighbor_index() {
        let points = vec![Point2::new(0., 0.), Point2::new(2., 2.), Point2::new(1., 0.), Point2::new(0., -1.), Point2::new(-1., 0.), Point2::new(2., 0.)];

        let t0 = Triangle::new(&points, N2Index(0), N2Index(1), N2Index(2));
        assert_eq!(0, t0.get_neighbor_index(N2Index(0), N2Index(1)));
        assert_eq!(0, t0.get_neighbor_index(N2Index(1), N2Index(0)));

        assert_eq!(1, t0.get_neighbor_index(N2Index(1), N2Index(2)));
        assert_eq!(1, t0.get_neighbor_index(N2Index(2), N2Index(1)));

        assert_eq!(2, t0.get_neighbor_index(N2Index(0), N2Index(2)));
        assert_eq!(2, t0.get_neighbor_index(N2Index(2), N2Index(0)));
    }
}
