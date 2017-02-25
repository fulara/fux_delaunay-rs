use std::cmp::Ordering;

#[inline]
pub fn order_float(a: &&f64, b: &&f64) -> Ordering {
    if a < b {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}