use types::Point3;
use math::side_of_plane;
use math::SideOfPlane;

pub fn is_ordered_correctly(a: &Point3, b: &Point3, c: &Point3, d: &Point3) -> bool {
    let side = side_of_plane(a, b, c, d);
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

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
