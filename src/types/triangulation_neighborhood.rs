use types::Point2;
use types::N2Index;
use types::T3Index;
use types::Triangle;

pub struct TriangulationNeighborhood {
    triangle_neighborhood: Vec<Vec<(N2Index, Option<T3Index>, Option<T3Index>)>>,
}

impl TriangulationNeighborhood {
    pub fn new() -> TriangulationNeighborhood {
        TriangulationNeighborhood { triangle_neighborhood: Vec::new() }
    }

    pub fn register_triangle(&mut self, triangle: &Triangle, triangle_index: T3Index) {
        self.register_connection(triangle.index_a(), triangle.index_b(), triangle_index);
        self.register_connection(triangle.index_b(), triangle.index_c(), triangle_index);
        self.register_connection(triangle.index_c(), triangle.index_a(), triangle_index);
    }

    pub fn get_neighbor(&self, p1 : N2Index, p2: N2Index, triangle_index: T3Index) -> Option<T3Index> {
        let (smaller, larger) = Self::smaller_larger(p1, p2);

        let v = &self.triangle_neighborhood[smaller.0];

        for e in v.iter() {
            if e.0 == larger {
                if e.1 == Some(triangle_index) {
                    return e.2;
                } else {
                    e.1;
                }
            }
        }

        None
    }

    fn register_connection(&mut self, p1: N2Index, p2: N2Index, triangle_index: T3Index) {
        let (smaller, larger) = TriangulationNeighborhood::smaller_larger(p1, p2);

        if self.triangle_neighborhood.len() < larger.0 {
            self.triangle_neighborhood.resize(larger.0, Vec::new());
        }

        let v = &mut self.triangle_neighborhood[smaller.0];

        for i in 0..v.len() {
            let e = &mut v[i];
            if e.0 == larger {
                assert!(e.1.is_some());
                assert!(e.2.is_none());
                e.2 = Some(triangle_index);

                return;
            }
        }
        v.push((larger, Some(triangle_index), None));
    }

    fn smaller_larger(p1: N2Index, p2: N2Index) -> (N2Index, N2Index) {
        if p1 < p2 { (p1, p2) } else { (p2, p1) }
    }
}

#[cfg(test)]
mod tests {
    use types::Point2;
    use types::Triangle;
    use types::N2Index;
    use types::T3Index;
    use super::*;

    #[test]
    fn testing_neighborhood() {
        let mut pts = vec![Point2::new(0.0, 0.0), Point2::new(1.0, 0.0), Point2::new(0.0, 1.0), Point2::new(1.0, 1.0)];

        let t0 = Triangle::new(&pts, N2Index(0), N2Index(1), N2Index(2));
        let t1 = Triangle::new(&pts, N2Index(1), N2Index(2), N2Index(3));

        let mut neighborhood = TriangulationNeighborhood::new();

        neighborhood.register_triangle(&t0, T3Index(0));
        neighborhood.register_triangle(&t1, T3Index(1));

        assert_eq!(Option::None, neighborhood.get_neighbor(N2Index(0), N2Index(1), T3Index(0)));
        assert_eq!(Some(T3Index(1)), neighborhood.get_neighbor(N2Index(1), N2Index(2), T3Index(0)));
        assert_eq!(Some(T3Index(1)), neighborhood.get_neighbor(N2Index(2), N2Index(1), T3Index(0)));
        assert_eq!(Option::None, neighborhood.get_neighbor(N2Index(2), N2Index(0), T3Index(0)));

        assert_eq!(Option::None, neighborhood.get_neighbor(N2Index(1), N2Index(3), T3Index(1)));
        assert_eq!(Some(T3Index(0)), neighborhood.get_neighbor(N2Index(1), N2Index(2), T3Index(1)));
        assert_eq!(Some(T3Index(0)), neighborhood.get_neighbor(N2Index(2), N2Index(1), T3Index(1)));
        assert_eq!(Option::None, neighborhood.get_neighbor(N2Index(2), N2Index(3), T3Index(1)));


    }
}