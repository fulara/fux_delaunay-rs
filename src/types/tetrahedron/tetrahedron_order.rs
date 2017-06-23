use types::Point3;
use math::side_of_plane;
use math::SideOfPlane;

pub fn is_ordered_correctly(a: &Point3, b: &Point3, c: &Point3, d: &Point3) -> bool {
    let side = side_of_plane(a, b, c, d);
    return side != SideOfPlane::Left;
//    return side == SideOfPlane::Right;
    if SideOfPlane::Right == side_of_plane(a, b, c, d) {
        true
    } else if side == SideOfPlane::OnPlane {
        if SideOfPlane::Right == side_of_plane(b, a, d, c) {
            true
        } else if side == SideOfPlane::OnPlane {
            if SideOfPlane::Right == side_of_plane(d, c, b, a) {
                true
            } else if side == SideOfPlane::OnPlane {
                if SideOfPlane::Right == side_of_plane(d, b, a, c) {
                    true
                } else if side == SideOfPlane::OnPlane {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}

pub fn is_ordered_correctly2(a: &Point3, b: &Point3, c: &Point3, d: &Point3) -> bool {
    side_of_plane(a, b, c, d) != SideOfPlane::Left// && side_of_plane(b, c, d, a) != SideOfPlane::Left
    /*if SideOfPlane::Right == side_of_plane(a, b, c, d) {
        true
    } else if side == SideOfPlane::OnPlane {
        if SideOfPlane::Right == side_of_plane(b, a, d, c) {
            true
        } else if side == SideOfPlane::OnPlane {
            if SideOfPlane::Right == side_of_plane(d, c, b, a) {
                true
            } else if side == SideOfPlane::OnPlane {
                if SideOfPlane::Right == side_of_plane(d, b, a, c) {
                    true
                } else if side == SideOfPlane::OnPlane {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    } */
}

#[cfg(test)]
mod tetrahedron_order {
    use super::*;
    use types::Point3;

    #[test]
    fn test() {
        let p1 = Point3::new(0., 1.0999999999999996, 99.);
        let p2 = Point3::new(0., 3.6000000000000014, 99.);
        let p3 = Point3::new(-9.9, -9.9, 108.9);
        let p4 = Point3::new(0., 6.239999771118166, 94.79999999999907);

        let p5 = Point3::new(0., 7.6999999999999895, 99.);

        let p10 = Point3::new(0., 0., 0.);
        let p11 = Point3::new(100., 0., 0.);
        let p12 = Point3::new(0., 100., 0.);
        let p13 = Point3::new(0., 0., 100.);

        /*let tr = Tetrahedron::new(&points, N3Index(0), N3Index(1), N3Index(2), N3Index(3));

        assert_eq!(*tr.a(&points), points[0]);
        assert_eq!(*tr.b(&points), points[3]);
        assert_eq!(*tr.c(&points), points[2]);
        assert_eq!(*tr.d(&points), points[1]); */

        println!("is ordered correctly? : {:?} ",
                 is_ordered_correctly2(&p1, &p2, &p3, &p4));
        println!("is ordered correctly? : {:?} ",
                 is_ordered_correctly2(&p1, &p4, &p5, &p3));

        println!("is ordered correctly? : {:?} ",
                 is_ordered_correctly2(&p10, &p13, &p12, &p11));
    }
}
