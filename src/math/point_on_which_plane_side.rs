use types::Point3;
use cgmath::Matrix3;
use cgmath::SquareMatrix;
use math::order_float;
use types::fp;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum PointLiesOnPlaneSide {
    Left,
    Right,
    OnPlane,
}

#[inline]
pub fn on_which_plane_side_point_lies(a: &Point3, b: &Point3, c: &Point3, p: &Point3) -> PointLiesOnPlaneSide {
    let ab = b - a;
    let ac = c - a;
    let ap = p - a;

    let det = Matrix3::new(ab.x, ab.y, ab.z,
                           ac.x, ac.y, ac.z,
                           ap.x, ap.y, ap.z).determinant();

    let arr_x = [ab.x, ac.x, ap.x];
    let arr_y = [ab.y, ac.y, ap.y];
    let arr_z = [ab.z, ac.z, ap.z];

    let max_val_x = arr_x.iter().max_by(order_float).unwrap();
    let min_val_x = arr_x.iter().min_by(order_float).unwrap();

    let max_val_y = arr_y.iter().max_by(order_float).unwrap();
    let min_val_y = arr_y.iter().min_by(order_float).unwrap();

    let max_val_z = arr_z.iter().max_by(order_float).unwrap();
    let min_val_z = arr_z.iter().min_by(order_float).unwrap();


    let max_x  = max_val_x.max(min_val_x.abs());
    let max_y  = max_val_y.max(min_val_y.abs());
    let max_z  = max_val_z.max(min_val_z.abs());


    let eps = max_x * max_y * max_z * fp::EPSILON;
    println!(" eps is: {} det is: {}", eps, det);
    if det < -eps {
        PointLiesOnPlaneSide::Left
    } else if det > eps {
        PointLiesOnPlaneSide::Right
    } else {
        PointLiesOnPlaneSide::OnPlane
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use types::Point3;

    #[test]
    fn testing_simplest_plane() {
        let a = Point3::new(0., 0., 0.);
        let b = Point3::new(0., 1., 0.);
        let c = Point3::new(1., 0., 0.);

        assert_eq!(PointLiesOnPlaneSide::OnPlane, on_which_plane_side_point_lies(&a, &b, &c, &Point3::new(0.5, 0.5, 0.)));
        assert_eq!(PointLiesOnPlaneSide::OnPlane, on_which_plane_side_point_lies(&a, &b, &c, &Point3::new(0.5, -5000., 0.)));
        assert_eq!(PointLiesOnPlaneSide::OnPlane, on_which_plane_side_point_lies(&a, &b, &c, &Point3::new(500., -2000., 0.)));

        assert_eq!(PointLiesOnPlaneSide::Right, on_which_plane_side_point_lies(&a, &b, &c, &Point3::new(0., 0., -1.)));
        assert_eq!(PointLiesOnPlaneSide::Right, on_which_plane_side_point_lies(&a, &b, &c, &Point3::new(50., 50., -1.)));

        assert_eq!(PointLiesOnPlaneSide::Left, on_which_plane_side_point_lies(&a, &b, &c, &Point3::new(0., 0., 1.)));
        assert_eq!(PointLiesOnPlaneSide::Left, on_which_plane_side_point_lies(&a, &b, &c, &Point3::new(50., 50., 1.)));
    }


    #[test]
    fn testing_random_plane() {
        let a = Point3::new(5., 10., 12.);
        let b = Point3::new(8., 10., 11.);
        let c = Point3::new(3., 3., 2.);

        let center = Point3::new((a.x + b.x + c.x) / 3., (a.y + b.y + c.y) / 3., (a.z + b.z + c.z) / 3.);

        println!("testing here.");
        assert_eq!(PointLiesOnPlaneSide::OnPlane, on_which_plane_side_point_lies(&a, &b, &c, &center));
        /*assert_eq!(PointLiesOnPlaneSide::OnPlane, on_which_plane_side_point_lies(&a, &b, &c, &Point3::new(0.5, -5000., 0.)));
        assert_eq!(PointLiesOnPlaneSide::OnPlane, on_which_plane_side_point_lies(&a, &b, &c, &Point3::new(500., -2000., 0.)));

        assert_eq!(PointLiesOnPlaneSide::Right, on_which_plane_side_point_lies(&a, &b, &c, &Point3::new(0., 0., -1.)));
        assert_eq!(PointLiesOnPlaneSide::Right, on_which_plane_side_point_lies(&a, &b, &c, &Point3::new(50., 50., -1.)));

        assert_eq!(PointLiesOnPlaneSide::Left, on_which_plane_side_point_lies(&a, &b, &c, &Point3::new(0., 0., 1.)));
        assert_eq!(PointLiesOnPlaneSide::Left, on_which_plane_side_point_lies(&a, &b, &c, &Point3::new(50., 50., 1.))); */
    }
}