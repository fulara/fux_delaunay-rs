pub trait RoundNPlaces {
    fn round_n(&self, round_to_place: i32) -> Self;
}

macro_rules! round_n_impl {
( $self_f:expr,$round_to_place:expr ) => {
    match $round_to_place {
        0 => $self_f.round(),
        1 => (*$self_f * 10.).round() / 10.,
        2 => (*$self_f * 100.).round() / 100.,
        3 => (*$self_f * 1000.).round() / 1000.,
        4 => (*$self_f * 10000.).round() / 10000.,
        5 => (*$self_f * 100000.).round() / 100000.,
        6 => (*$self_f * 1000000.).round() / 1000000.,
        7 => (*$self_f * 10000000.).round() / 10000000.,
        8 => (*$self_f * 100000000.).round() / 100000000.,
        9 => (*$self_f * 1000000000.).round() / 1000000000.,
        10 => (*$self_f * 10000000000.).round() / 10000000000.,
        11 => (*$self_f * 100000000000.).round() / 100000000000.,
        12 => (*$self_f * 1000000000000.).round() / 1000000000000.,
        13 => (*$self_f * 10000000000000.).round() / 10000000000000.,
        14 => (*$self_f * 100000000000000.).round() / 100000000000000.,
        _ => panic!("")
    }
}
}

impl RoundNPlaces for f64 {
    fn round_n(&self, round_to_place: i32) -> f64 {
        round_n_impl!(self, round_to_place)
    }
}

impl RoundNPlaces for f32 {
    fn round_n(&self, round_to_place: i32) -> f32 {
        round_n_impl!(self, round_to_place)
    }
}

#[cfg(test)]
mod tests {
    use math::round::RoundNPlaces;

    fn float_eq(l: f64, r: f64) {
        if (l - r).abs() > 0.0000001 {
            panic!("float_eq failed for l: {}  r: {}", l, r)
        }
    }

    #[test]
    fn testing_01() {
        float_eq(0., 0.11111111111111.round_n(0));
        float_eq(0.1, 0.11111111111111.round_n(1));
        float_eq(0.11, 0.11111111111111.round_n(2));
        float_eq(0.111, 0.11111111111111.round_n(3));
        float_eq(0.1111, 0.11111111111111.round_n(4));
        float_eq(0.11111, 0.11111111111111.round_n(5));
        float_eq(0.111111, 0.11111111111111.round_n(6));
        float_eq(0.1111111, 0.11111111111111.round_n(7));
        float_eq(0.11111111, 0.11111111111111.round_n(8));
        float_eq(0.111111111, 0.11111111111111.round_n(9));
        float_eq(0.1111111111, 0.11111111111111.round_n(10));
        float_eq(0.11111111111, 0.11111111111111.round_n(11));
        float_eq(0.111111111111, 0.11111111111111.round_n(12));
    }

    #[test]
    fn testing_09() {
        float_eq(1., 0.99999999999999.round_n(0));
        float_eq(1., 0.99999999999999.round_n(1));
        float_eq(1., 0.99999999999999.round_n(2));
        float_eq(1., 0.99999999999999.round_n(3));
        float_eq(1., 0.99999999999999.round_n(4));
        float_eq(1., 0.99999999999999.round_n(5));
        float_eq(1., 0.99999999999999.round_n(6));
        float_eq(1., 0.99999999999999.round_n(7));
        float_eq(1., 0.99999999999999.round_n(8));
        float_eq(1., 0.99999999999999.round_n(9));
        float_eq(1., 0.99999999999999.round_n(10));
        float_eq(1., 0.99999999999999.round_n(11));
        float_eq(1., 0.99999999999999.round_n(12));
    }
}
