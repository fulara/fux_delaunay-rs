use ::types::Point2;
use cgmath::Matrix2;
use cgmath::SquareMatrix;
use std::mem;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum CircleSide {
    Inside,
    Outside,
}

pub fn which_side_of_circumcircle(p: &Point2, q: &Point2, r: &Point2, t: &Point2) -> CircleSide {
    //TODO the algorithm is taken from CGAL for now. rewrite.
    let qpx = q.x - p.x;
    let qpy = q.y - p.y;
    let rpx = r.x - p.x;
    let rpy = r.y - p.y;
    let tpx = t.x - p.x;
    let tpy = t.y - p.y;

    let tqx = t.x - q.x;
    let tqy = t.y - q.y;
    let rqx = r.x - q.x;
    let rqy = r.y - q.y;

    let det = Matrix2::new(qpx * tpy - qpy * tpx, tpx * tqx + tpy * tqy,
                           qpx * rpy - qpy * rpx, rpx * rqx + rpy * rqy).determinant();

    let mut maxx = qpx.abs();
    let mut maxy = qpy.abs();

    let arpx = rpx.abs();
    let arpy = rpy.abs();

    let atqx = tqx.abs();
    let atqy = tqy.abs();

    let atpx = tpx.abs();
    let atpy = tpy.abs();

    let arqx = rqx.abs();
    let arqy = rqy.abs();

    if maxx < arpx { maxx = arpx };
    if maxx < atpx { maxx = atpx };
    if maxx < atqx { maxx = atqx };
    if maxx < arqx { maxx = arqx };

    if maxy < arpy { maxy = arpy };
    if maxy < atpy { maxy = atpy };
    if maxy < atqy { maxy = atqy };
    if maxy < arqy { maxy = arqy };

    if maxx > maxy { mem::swap(&mut maxx, &mut maxy) };

    if maxx < 1e-73 {
        if maxx == 0. {
            //should be on boundary here, treat this case as OK.
            return CircleSide::Outside;
        }
    } else if maxy < 1e76 {
        //this is tricky one. I am assuming here that if everything is so close to the edge then its OUTSIDE the circle.
        if det >= -1e-6 {
            return CircleSide::Outside;
        } else {
            return CircleSide::Inside;
        }
    }

    panic!("which_side_of_circumcircle unexpected maxx{} maxy{} det{}", maxx, maxy, det);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::types::Point2;
    use ::types::Triangle;
    use ::types::N2Index;

    use cgmath::Rad;
    use cgmath::Vector2;
    use cgmath::{Matrix, Matrix2};
    use cgmath::{Rotation, Rotation2, Basis2};
    use cgmath::ApproxEq;
    use std::f64;

    #[test]
    fn testing_circumside() {
        let p1 = Point2::new(-0.99996, 6.4611);
        let p2 = Point2::new(2.1204, 9.5812);
        let p3 = Point2::new(5.0495, 7.5349);

        assert_eq!(CircleSide::Outside, which_side_of_circumcircle(&p1, &p2, &p3, &Point2::new(20., 20.)));
        assert_eq!(CircleSide::Outside, which_side_of_circumcircle(&p1, &p2, &p3, &Point2::new(5.2895, 6.7100)));

        assert_eq!(CircleSide::Inside, which_side_of_circumcircle(&p1, &p2, &p3, &Point2::new(5.2002, 6.6470)));
        assert_eq!(CircleSide::Inside, which_side_of_circumcircle(&p1, &p2, &p3, &Point2::new(0.0582, 4.2369)));

        assert_eq!(CircleSide::Outside, which_side_of_circumcircle(&p1, &p2, &p3, &Point2::new(-2.8154, -4.)));
    }

    #[test]
    fn testing_with_triangle() {
        let nodes = vec!(Point2::new(0., 0.), Point2::new(1., 1.), Point2::new(2., 0.));

        let tr = Triangle::new(&nodes, N2Index(0), N2Index(1), N2Index(2));

        assert_eq!(CircleSide::Inside, which_side_of_circumcircle(&tr.a(&nodes), &tr.b(&nodes), &tr.c(&nodes), &Point2::new(0.5, 0.5)));
        assert_eq!(CircleSide::Outside, which_side_of_circumcircle(&tr.a(&nodes), &tr.b(&nodes), &tr.c(&nodes), &Point2::new(1.1, 1.1)));
    }


    quickcheck! {
    fn quick_check_test(x: f64, y: f64, r: f64) -> bool {
        if r < 0.2 {
            return true;
        }

        let rotation: Basis2<f64> = Rotation2::from_angle(Rad(-f64::consts::FRAC_PI_2));
        let one_hundreth_pi_rotattion: Basis2<f64> = Rotation2::from_angle(Rad(-f64::consts::PI / 100.));

        let p = Point2::new(x + r, y);
        let p0 = rotation.rotate_point(Point2::new(r, 0.));
        let p1 = rotation.rotate_point(p0);
        let p2 = rotation.rotate_point(p1);

        let p0 = Point2::new(p0.x  + p.x, p0.y + p.y);
        let p1 = Point2::new(p1.x  + p.x, p1.y + p.y);
        let p2 = Point2::new(p2.x  + p.x, p2.y + p.y);


        for i in 1..20 {
            let multiplier = 0.1 * i as f64;

            let mut rotated_point = Point2::new(r * multiplier, 0.);

            for _ in 0 .. 200 {
                rotated_point = one_hundreth_pi_rotattion.rotate_point(rotated_point);

                let tested_point = Point2::new(rotated_point.x + p.x, rotated_point.y + p.y);
                let side = which_side_of_circumcircle(&p0, &p1, &p2, &tested_point);

                if multiplier < 0.99 {

                    if side != CircleSide::Inside {
                        return false;
                    }
                } else {
                      if side != CircleSide::Outside {
                        return false;
                    }
                }
            }
        }

        true
    }
    }
}