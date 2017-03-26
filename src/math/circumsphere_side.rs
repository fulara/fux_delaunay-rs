use types::Point3;
use cgmath::Matrix3;
use cgmath::Matrix4;
use cgmath::SquareMatrix;
use std::mem;
use types::fp;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum SphereSide {
    Inside,
    Outside,
}

//todo rewrite this.
pub fn circumsphere_side(p: &Point3, q: &Point3, r: &Point3, s: &Point3, t: &Point3) -> SphereSide {
    let ptx = p.x - t.x;
    let pty = p.y - t.y;
    let ptz = p.z - t.z;
    let pt2 = ptx * ptx + pty * pty + ptz * ptz;

    let qtx = q.x - t.x;
    let qty = q.y - t.y;
    let qtz = q.z - t.z;
    let qt2 = qtx * qtx + qty * qty + qtz * qtz;

    let rtx = r.x - t.x;
    let rty = r.y - t.y;
    let rtz = r.z - t.z;
    let rt2 = rtx * rtx + rty * rty + rtz * rtz;

    let stx = s.x - t.x;
    let sty = s.y - t.y;
    let stz = s.z - t.z;
    let st2 = stx * stx + sty * sty + stz * stz;

    let mut maxx = ptx.abs();
    let mut maxy = pty.abs();
    let mut maxz = ptz.abs();

    let aqtx = qtx.abs();
    let artx = rtx.abs();
    let astx = stx.abs();

    let aqty = qty.abs();
    let arty = rty.abs();
    let asty = sty.abs();

    let aqtz = qtz.abs();
    let artz = rtz.abs();
    let astz = stz.abs();

    if maxx < aqtx {
        maxx = aqtx
    };
    if maxx < artx {
        maxx = artx
    };
    if maxx < astx {
        maxx = astx
    };

    if maxy < aqty {
        maxy = aqty
    };
    if maxy < arty {
        maxy = arty
    };
    if maxy < asty {
        maxy = asty
    };

    if maxz < aqtz {
        maxz = aqtz
    };
    if maxz < artz {
        maxz = artz
    };
    if maxz < astz {
        maxz = astz
    };

    let mut eps = 1e-12 * maxx * maxy * maxz;
    if maxx > maxz {
        ::std::mem::swap(&mut maxx, &mut maxz);
    }
    if maxy > maxz {
        ::std::mem::swap(&mut maxy, &mut maxz);
    } else if maxy < maxx {
        ::std::mem::swap(&mut maxx, &mut maxy);
    }

    let det = Matrix4::new(ptx,
                           pty,
                           ptz,
                           pt2,
                           rtx,
                           rty,
                           rtz,
                           rt2,
                           qtx,
                           qty,
                           qtz,
                           qt2,
                           stx,
                           sty,
                           stz,
                           st2)
            .determinant();

    println!("det is: {:?} p: {:?} q: {:?} r: {:?} s: {:?} t: {:?}",
             det,
             p,
             q,
             r,
             s,
             t);

    eps *= maxz * maxz;
    if det > eps {
        return SphereSide::Inside;
    }
    return SphereSide::Outside;
}

#[cfg(test)]
mod circumsphere_side {
    use super::*;
    use types::Point3;

    use cgmath::Rad;
    use cgmath::{Rotation, Rotation2, Rotation3, Basis3};
    use std::f64;

    type Vector3 = ::cgmath::Vector3<f64>;

    use types::Tetrahedron;
    use types::N3Index;

    #[test]
    fn simple_example_test() {
        let nodes = vec![Point3::new(0., 0., 0.),
                         Point3::new(1., 0., 0.),
                         Point3::new(0., 1., 0.),
                         Point3::new(0., 0., 1.)];
        let center = Point3::new(0.25, 0.25, 0.25);
        let tetra = Tetrahedron::new(&nodes, N3Index(0), N3Index(1), N3Index(2), N3Index(3));

        assert_eq!(circumsphere_side(&nodes[0], &nodes[1], &nodes[2], &nodes[3], &center),
                   SphereSide::Inside);
        assert!(tetra.is_point_in_circumsphere(&center, &nodes));
    }

    #[quickcheck]
    fn quick_check_test(x: f64, y: f64, z: f64, r: f64) {
        if r < 0.00001 {
            return;
        }

        if x == 0. && y == 0. && z == 0. {
            return;
        }

        quick_check_test_impl(x, y, z, r);
    }

    #[test]
    fn quick_check_error_case() {
        quick_check_test_impl(0., 0., 1., 86.4520368353185);
        quick_check_test_impl(0., 0., 1., 46.27116838241295);
    }

    fn quick_check_test_impl(x: f64, y: f64, z: f64, r: f64) {
        let x_rotation: Basis3<f64> = Rotation3::from_angle_x(Rad(-f64::consts::FRAC_PI_4));
        let y_rotation: Basis3<f64> = Rotation3::from_angle_y(Rad(-f64::consts::FRAC_PI_4));
        let z_rotation: Basis3<f64> = Rotation3::from_angle_z(Rad(-f64::consts::FRAC_PI_4));

        let rotated_point = Point3::new(r, r, r);

        let rotation_a = x_rotation.rotate_point(rotated_point);
        let rotation_b = x_rotation.rotate_point(rotation_a);
        let rotation_c = y_rotation.rotate_point(rotation_b);
        let rotation_d = y_rotation.rotate_point(rotation_c);

        let a = Point3::new(x + rotation_a.x, y + rotation_a.y, z + rotation_a.z);
        let b = Point3::new(x + rotation_b.x, y + rotation_b.y, z + rotation_b.z);
        let c = Point3::new(x + rotation_c.x, y + rotation_c.y, z + rotation_c.z);
        let d = Point3::new(x + rotation_d.x, y + rotation_d.y, z + rotation_d.z);
        let nodes = [a, b, c, d];

        let tetra = Tetrahedron::new(&nodes, N3Index(0), N3Index(1), N3Index(2), N3Index(3));

        assert_eq!(SphereSide::Inside,
                   circumsphere_side(&a, &b, &c, &d, &Point3::new(x, y, z)));


        let rotations: [Basis3<f64>; 3] = [Rotation3::from_angle_x(Rad(-f64::consts::PI / 50.)),
                                           Rotation3::from_angle_y(Rad(-f64::consts::PI / 50.)),
                                           Rotation3::from_angle_z(Rad(-f64::consts::PI / 50.))];

        for i in 1..10 {
            let multiplier = 0.2 * i as f64;

            let rotated_point = Point3::new(r * multiplier, r * multiplier, r * multiplier);

            for rotation in rotations.iter() {
                let mut rotated = rotation.rotate_point(rotated_point);
                for _ in 0..100 {
                    rotated = rotation.rotate_point(rotated);

                    let tested_point = Point3::new(x + rotated.x, y + rotated.y, z + rotated.z);

                    let side = circumsphere_side(&a, &b, &c, &d, &tested_point);

                    let expected = if multiplier <= 0.9 {
                        SphereSide::Inside
                    } else {
                        SphereSide::Outside
                    };

                    if side != expected {
                        panic!("Failure at Multiplier {:?}. \
                        Expected {:?} got {:?}, a'{:?}', b'{:?}', c'{:?}', d'{:?}', test'{:?}'",
                               multiplier,
                               expected,
                               side,
                               a,
                               b,
                               c,
                               d,
                               tested_point);
                    }

                    match expected {
                        SphereSide::Inside => {
                            assert!(tetra.is_point_in_circumsphere(&tested_point, &nodes))
                        }
                        SphereSide::Outside => {
                            assert!(!tetra.is_point_in_circumsphere(&tested_point, &nodes))
                        }
                    }
                }
            }
        }
    }
}
