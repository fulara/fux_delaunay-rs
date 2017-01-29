use types::Point2;
use types::Triangle;

pub struct TriangulationNeighborhood {
    triangle_neighborhood: Vec<Vec<(usize, Option<usize>, Option<usize>)>>,
}

impl TriangulationNeighborhood {
    pub fn new() -> TriangulationNeighborhood {
        TriangulationNeighborhood { triangle_neighborhood: Vec::new() }
    }

    pub fn register_triangle(&mut self, triangle: &Triangle, triangle_index: usize) {
        self.register_connection(triangle.index_a(), triangle.index_b(), triangle_index);
        self.register_connection(triangle.index_b(), triangle.index_c(), triangle_index);
        self.register_connection(triangle.index_c(), triangle.index_a(), triangle_index);
    }

    pub fn get_neighbor(&self, p1 : usize, p2: usize, triangle_index: usize) -> Option<usize> {
        let (smaller, larger) = Self::smaller_larger(p1, p2);

        let v = &self.triangle_neighborhood[smaller];

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

    fn register_connection(&mut self, p1: usize, p2: usize, triangle_index: usize) {
        let (smaller, larger) = TriangulationNeighborhood::smaller_larger(p1, p2);

        if self.triangle_neighborhood.len() < larger {
            self.triangle_neighborhood.resize(larger, Vec::new());
        }

        let v = &mut self.triangle_neighborhood[smaller];

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

    fn smaller_larger(p1: usize, p2: usize) -> (usize, usize) {
        if p1 < p2 { (p1, p2) } else { (p2, p1) }
    }
}

#[cfg(test)]
mod tests {
    use types::Point2;
    use types::Triangle;
    use super::*;

    #[test]
    fn testing_neighborhood() {
        let mut pts = vec![Point2::new(0.0, 0.0), Point2::new(1.0, 0.0), Point2::new(0.0, 1.0), Point2::new(1.0, 1.0)];

        let t0 = Triangle::new(&pts, 0, 1, 2);
        let t1 = Triangle::new(&pts, 1, 2, 3);

        let mut neighborhood = TriangulationNeighborhood::new();

        neighborhood.register_triangle(&t0, 0);
        neighborhood.register_triangle(&t1, 1);

        assert_eq!(Option::None, neighborhood.get_neighbor(0, 1, 0));
        assert_eq!(Some(1), neighborhood.get_neighbor(1, 2, 0));
        assert_eq!(Some(1), neighborhood.get_neighbor(2, 1, 0));
        assert_eq!(Option::None, neighborhood.get_neighbor(2, 0, 0));

        assert_eq!(Option::None, neighborhood.get_neighbor(1, 3, 1));
        assert_eq!(Some(0), neighborhood.get_neighbor(1, 2, 1));
        assert_eq!(Some(0), neighborhood.get_neighbor(2, 1, 1));
        assert_eq!(Option::None, neighborhood.get_neighbor(2, 3, 1));


    }
}