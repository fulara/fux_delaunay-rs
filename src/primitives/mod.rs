use math::*;
pub type Point2 = ::cgmath::Point2<f64>;

struct Triangle {
    a: usize,
    b: usize,
    c: usize,
}

impl Triangle {
    #[inline]
    fn new(points : &Vec<Point2>, a: usize, b: usize, c: usize) -> Triangle {
        if on_which_side_point_lies(&points[a], &points[b], &points[c]) == PointLiesOnSide::Left {
            Triangle { a: a, b: c, c: b }
        } else {
            Triangle { a: a, b: b, c: c }
        }
    }

    #[inline]
    fn a<'a>(&self, points: &'a Vec<Point2>) -> &'a Point2 {
        &points[self.a]
    }

    #[inline]
    fn b<'a>(&self, points: &'a Vec<Point2>) -> &'a Point2 {
        &points[self.b]
    }

    #[inline]
    fn c<'a>(&self, points: &'a Vec<Point2>) -> &'a Point2 {
        &points[self.c]
    }
}

#[cfg(test)]
mod triangle {
    use super::*;
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
}


