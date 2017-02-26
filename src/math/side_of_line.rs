use ::types::Point2;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum PointLiesOnLineSide {
    Left,
    Right,
    OnLine,
}

#[inline]
pub fn side_of_line(line_begin: &Point2, line_end: &Point2, point: &Point2) -> PointLiesOnLineSide {
    let c0r0 = line_end.x - line_begin.x;
    let c0r1 = line_end.y - line_begin.y;
    let c1r0 = point.x - line_begin.x;
    let c1r1 = point.y - line_begin.y;

    let cross = c0r0 * c1r1 - c1r0 * c0r1;

    if cross > 0. {
        PointLiesOnLineSide::Left
    } else if cross < 0. {
        PointLiesOnLineSide::Right
    } else {
        PointLiesOnLineSide::OnLine
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use types::Point2;

    #[test]
    fn on_which_side_point_lies_test() {
        let a = Point2::new(0., 0.);
        let b = Point2::new(1., 1.);
        let c = Point2::new(0.5, 0.5);
        let d = Point2::new(1., 0.);

        assert_eq!(PointLiesOnLineSide::OnLine, side_of_line(&a, &b, &c));
        assert_eq!(PointLiesOnLineSide::OnLine, side_of_line(&b, &a, &c));

        assert_eq!(PointLiesOnLineSide::Left, side_of_line(&a, &d, &c));
        assert_eq!(PointLiesOnLineSide::Right, side_of_line(&d, &a, &c));
    }
}