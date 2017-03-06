use types::N3Index;

//this could be generized and moved to math. but not needed yet.
pub fn sort_3(mut p1: N3Index, mut p2: N3Index, mut p3: N3Index) -> (N3Index, N3Index, N3Index) {
    if p1 > p2 {
        ::std::mem::swap(&mut p1, &mut p2);
    }

    if p2 > p3 {
        ::std::mem::swap(&mut p2, &mut p3);
    }

    if p1 > p2 {
        ::std::mem::swap(&mut p1, &mut p2);
    }

    (p1, p2, p3)
}

#[cfg(test)]
mod tests {
    use types::N3Index;
    use super::*;

    #[quickcheck]
    fn sort3_test(a: usize, b: usize, c: usize) {
        let sorted = super::sort_3(N3Index(a),N3Index(b),N3Index(c));

        assert!(sorted.0 <= sorted.1);
        assert!(sorted.1 <= sorted.2);
    }
}