use math::*;
use cgmath::InnerSpace;
use types::Point2;

pub struct Triangle {
    a: usize,
    b: usize,
    c: usize,
}

impl Triangle {
    #[inline]
    pub fn new(points : &Vec<Point2>, a: usize, b: usize, c: usize) -> Triangle {
        if on_which_side_point_lies(&points[a], &points[b], &points[c]) == PointLiesOnSide::Left {
            Triangle { a: a, b: c, c: b }
        } else {
            Triangle { a: a, b: b, c: c }
        }
    }

    #[inline]
    pub fn a<'a>(&self, points: &'a Vec<Point2>) -> &'a Point2 {
        &points[self.a]
    }

    #[inline]
    pub fn b<'a>(&self, points: &'a Vec<Point2>) -> &'a Point2 {
        &points[self.b]
    }

    #[inline]
    pub fn c<'a>(&self, points: &'a Vec<Point2>) -> &'a Point2 {
        &points[self.c]
    }

    #[inline]
    pub fn index_a(&self) -> usize {
        self.a
    }

    #[inline]
    pub fn index_b(&self) -> usize {
        self.b
    }

    #[inline]
    pub fn index_c(&self) -> usize {
        self.c
    }

    #[inline]
    pub fn is_point_inside(&self, points : &Vec<Point2>, p : &Point2) -> bool
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
    pub fn create_center_point(&self, points : &Vec<Point2>) -> Point2 {
        let a = self.a(points);
        let b = self.b(points);
        let c = self.c(points);

        Point2::new((a.x + b.x + c.x)/3., (a.y + b.y + c.y)/3.)
    }

    #[inline]
    pub fn clone(&self) -> Triangle {
        Triangle { a : self.a, b : self.b, c : self.c }
    }
}

#[cfg(test)]
mod triangle {
    use super::*;
    use types::Point2;
    #[test]
    fn abc_get_and_point_order_check() {
        let points = vec![ Point2::new(0.,0.),  Point2::new(2.,2.), Point2::new(1.,0.) ];

        let tr = Triangle::new(&points, 0,1,2);
        let tr2 = Triangle::new(&points, 0,2,1);

        assert_eq!(*tr.a(&points), Point2::new(0.,0.));
        assert_eq!(*tr.b(&points), Point2::new(2.,2.));
        assert_eq!(*tr.c(&points), Point2::new(1.,0.));

        //triangle::new got different order, and yet the order is the same.
        assert_eq!(*tr2.a(&points), Point2::new(0.,0.));
        assert_eq!(*tr2.b(&points), Point2::new(2.,2.));
        assert_eq!(*tr2.c(&points), Point2::new(1.,0.));
    }

    #[test]
    fn points_are_put_in_clockwise_order() {
        let points = vec![ Point2::new(5.,5.),  Point2::new(-1.,70.), Point2::new(0.,0.) ];

        let tr = Triangle::new(&points, 0,1,2);

        assert_eq!(*tr.a(&points), Point2::new(5.,5.));
        assert_eq!(*tr.b(&points), Point2::new(0.,0.));
        assert_eq!(*tr.c(&points), Point2::new(-1.,70.));
    }

    #[test]
    fn is_point_inside() {
        let points = vec![ Point2::new(0.,0.),  Point2::new(1.,1.), Point2::new(2.,0.) ];
        let tr = Triangle::new(&points, 0,1,2);

        assert_eq!(true, tr.is_point_inside(&points, &Point2::new(0.5,0.5)));
        assert_eq!(true, tr.is_point_inside(&points, &Point2::new(0.1,0.1)));
        assert_eq!(true, tr.is_point_inside(&points, &Point2::new(0.9,0.9)));
        assert_eq!(true, tr.is_point_inside(&points, &points[0]));
        assert_eq!(true, tr.is_point_inside(&points, &points[1]));
        assert_eq!(true, tr.is_point_inside(&points, &points[2]));
        assert_eq!(true, tr.is_point_inside(&points, &Point2::new(0.,0.)));

        assert_eq!(false, tr.is_point_inside(&points, &Point2::new(0.5,1.1)));
        assert_eq!(false, tr.is_point_inside(&points, &Point2::new(-0.0000001,0.)));
        assert_eq!(false, tr.is_point_inside(&points, &Point2::new(1.1,1.1)));
        assert_eq!(false, tr.is_point_inside(&points, &Point2::new(-0.5,0.5)));

    }
}
