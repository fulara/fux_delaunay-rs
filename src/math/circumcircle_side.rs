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
    } else if maxy < 1e76 /* sqrt(sqrt(max_double/16 [hadamard])) */ {
        let eps = 8.8878565762001373e-15 * maxx * maxy * (maxy * maxy);
        if det > eps {
            return CircleSide::Outside;
        }
        if det < -eps {
            return CircleSide::Inside;
        }
    }

    panic!("which_side_of_circumcircle unexpected");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::types::Point2;
    use ::types::Triangle;
    use ::types::N2Index;

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
        let nodes = vec!(Point2::new(0.,0.),Point2::new(1.,1.), Point2::new(2.,0.));

        let tr = Triangle::new(&nodes, N2Index(0), N2Index(1), N2Index(2));

        assert_eq!(CircleSide::Inside, which_side_of_circumcircle(&tr.a(&nodes), &tr.b(&nodes), &tr.c(&nodes), &Point2::new(0.5, 0.5)));
        assert_eq!(CircleSide::Outside, which_side_of_circumcircle(&tr.a(&nodes), &tr.b(&nodes), &tr.c(&nodes), &Point2::new(1.1, 1.1)));
    }
}