use types::Point3;

#[inline]
fn squared_euclidean(a: &[f64], b: &[f64]) -> f64 {
    debug_assert!(a.len() == b.len());

    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y) * (x - y))
        .sum()
}

#[inline]
pub fn distance3_squared(l: &Point3, r: &Point3) -> f64 {
    return squared_euclidean(&[l.x, l.y, l.z], &[r.x, r.y, r.z]);
}


#[cfg(test)]
mod math_distance_tests {
    use super::*;

    #[test]
    fn distance3_test() {
        let one = Point3::new(1.0, 1.0, 1.0);
        let two = Point3::new(2.0, 2.0, 2.0);
        let zero = Point3::new(0., 0., 0.);

        assert_eq!(0.0, distance3_squared(&zero, &zero));
        assert_eq!(3.0, distance3_squared(&zero, &one));
        assert_eq!(12.0, distance3_squared(&zero, &two));
        assert_eq!(3.0, distance3_squared(&one, &two));
    }
}
