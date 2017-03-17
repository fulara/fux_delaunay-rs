use types::Point3;
use math::side_of_plane;
use math::SideOfPlane;

pub fn is_ordered_correctly(a: &Point3, b: &Point3, c: &Point3, d: &Point3) -> bool {
    SideOfPlane::Right == side_of_plane(a, b, c, d)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
