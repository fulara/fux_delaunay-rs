use ::primitives::Point2;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum PointLiesOnSide {
    Left,
    Right,
    OnLine,
}

#[inline]
pub fn on_which_side_point_lies(line_begin: &Point2, line_end: &Point2, point: &Point2) -> PointLiesOnSide {
    let c0r0 = line_end.x - line_begin.x;
    let c0r1 = line_end.y - line_begin.y;
    let c1r0 = point.x - line_begin.x;
    let c1r1 = point.y - line_begin.y;

    let cross = c0r0 * c1r1 - c1r0 * c0r1;

    println!("cross is {} c0r0 {} c0r1 {}", cross, c0r0, c0r1);

    if cross > 0. {
        PointLiesOnSide::Left
    } else if cross < 0. {
        PointLiesOnSide::Right
    } else {
        PointLiesOnSide::OnLine
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn on_which_side_point_lies_test() {
        let a = Point2::new(0., 0.);
        let b = Point2::new(1., 1.);
        let c = Point2::new(0.5, 0.5);
        let d = Point2::new(1., 0.);

        assert_eq!(PointLiesOnSide::OnLine, on_which_side_point_lies(&a, &b, &c));
        assert_eq!(PointLiesOnSide::OnLine, on_which_side_point_lies(&b, &a, &c));

        assert_eq!(PointLiesOnSide::Left, on_which_side_point_lies(&a, &d, &c));
        assert_eq!(PointLiesOnSide::Right, on_which_side_point_lies(&d, &a, &c));
    }
}