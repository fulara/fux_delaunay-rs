use std::ops::*;
use ::cgmath::*;
use cgmath::num_traits::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct F64Err {
    val: f64,
    err: f64,
}

impl F64Err {
    pub fn new_errorless(val: f64) -> F64Err {
        F64Err {
            val: val,
            err: 0.
        }
    }

    pub fn new(val: f64) -> F64Err {
        F64Err {
            val: val,
            err: val
        }
    }

    pub fn new_exact(val: f64, err: f64) -> F64Err {
        F64Err {
            val: val,
            err: err
        }
    }

    pub fn val(&self) -> f64 {
        self.val
    }

    pub fn err(&self) -> f64 {
        self.err
    }
}

impl Mul for F64Err {
    type Output = F64Err;
    fn mul(self, rhs: Self) -> Self::Output {
        F64Err {
            val: self.val * rhs.val,
            err: self.val.abs() * rhs.err + rhs.val.abs() * self.err
        }
    }
}

impl MulAssign for F64Err {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl Div for F64Err {
    type Output = F64Err;
    fn div(self, rhs: Self) -> Self::Output {
        F64Err {
            val: self.val / rhs.val,
            err: self.err / rhs.val.abs() + rhs.err * self.val / (rhs.val * rhs.val)
        }
    }
}

impl DivAssign for F64Err {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs
    }
}

impl Add for F64Err {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        F64Err {
            val: self.val + rhs.val,
            err: self.val.abs().max(rhs.val.abs()) + self.err + rhs.err
        }
    }
}

impl AddAssign for F64Err {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Sub for F64Err {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        F64Err {
            val: self.val - rhs.val,
            err: self.val.abs().max(rhs.val.abs()) + self.err + rhs.err
        }
    }
}

impl SubAssign for F64Err {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl Rem for F64Err {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        unimplemented!();
    }
}

impl RemAssign for F64Err {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs
    }
}

impl PartialOrd for F64Err {
    fn partial_min(self, other: Self) -> Self {
        if self.val < other.val {
            self
        } else {
            other
        }
    }
    fn partial_max(self, other: Self) -> Self {
        if self.val > other.val {
            self
        } else {
            other
        }
    }
}

impl ::std::cmp::PartialOrd for F64Err {
    fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
        self.val.partial_cmp(&other.val)
    }
}

impl Zero for F64Err {
    fn zero() -> Self {
        Self::new_errorless(0.)
    }

    fn is_zero(&self) -> bool {
        self.val.is_zero()
    }
}

impl One for F64Err {
    fn one() -> Self {
        Self::new_errorless(1.)
    }
}

impl ToPrimitive for F64Err {
    /// Converts the value of `self` to an `isize`.
    #[inline]
    fn to_isize(&self) -> Option<isize> {
        self.val.to_isize()
    }

    /// Converts the value of `self` to an `i8`.
    #[inline]
    fn to_i8(&self) -> Option<i8> {
        self.val.to_i8()
    }

    /// Converts the value of `self` to an `i16`.
    #[inline]
    fn to_i16(&self) -> Option<i16> {
        self.val.to_i16()
    }

    /// Converts the value of `self` to an `i32`.
    #[inline]
    fn to_i32(&self) -> Option<i32> {
        self.val.to_i32()
    }

    #[inline]
    fn to_i64(&self) -> Option<i64> {
        self.val.to_i64()
    }

    /// Converts the value of `self` to a `usize`.
    #[inline]
    fn to_usize(&self) -> Option<usize> {
        self.val.to_usize()
    }

    /// Converts the value of `self` to an `u8`.
    #[inline]
    fn to_u8(&self) -> Option<u8> {
        self.val.to_u8()
    }

    /// Converts the value of `self` to an `u16`.
    #[inline]
    fn to_u16(&self) -> Option<u16> {
        self.val.to_u16()
    }

    /// Converts the value of `self` to an `u32`.
    #[inline]
    fn to_u32(&self) -> Option<u32> {
        self.val.to_u32()
    }

    /// Converts the value of `self` to an `u32`.
    #[inline]
    fn to_u64(&self) -> Option<u64> {
        self.val.to_u64()
    }

    /// Converts the value of `self` to an `f32`.
    #[inline]
    fn to_f32(&self) -> Option<f32> {
        self.val.to_f32()
    }

    /// Converts the value of `self` to an `f64`.
    #[inline]
    fn to_f64(&self) -> Option<f64> {
        self.val.to_f64()
    }
}

impl NumCast for F64Err {
    fn from<T>(t : T) -> Option<Self> {
        //todo finish me.
        Some(Self::new(0.))
    }
}

impl Num for F64Err {
    type FromStrRadixErr = ParseFloatError;
    fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseFloatError> {
        //todo finish me.
        Ok(Self::new(0.))
    }
}

impl BaseNum for F64Err {}

impl ApproxEq for F64Err {

}

impl Neg for F64Err {

}

impl Float for F64Err {

}

impl BaseFloat for F64Err {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplication_error() {
        let left = F64Err::new_errorless(2.);
        let right = F64Err::new_errorless(2.);

        let res = left * right;

        assert_eq!(4., res.val());
        assert_eq!(0., res.err());
    }

    #[test]
    fn addition_error() {
        let left = F64Err::new_errorless(2.);
        let right = F64Err::new_errorless(3.);

        let res = left + right;

        assert_eq!(5., res.val());
        assert_eq!(3., res.err());
    }

    #[test]
    fn addition_then_multipl() {
        let left = F64Err::new_errorless(2.);
        let right = F64Err::new_errorless(3.);

        let res_add = left + right;
        let res_mul = res_add * left;

        assert_eq!(10., res_mul.val());
        assert_eq!(3. * 2., res_mul.err());

        let res_mul = res_mul * res_mul;

        assert_eq!(100., res_mul.val());
        assert_eq!((3. * 2.) * 10. + (3. * 2.) * 10., res_mul.err());
    }

    #[test]
    fn subtract_error() {
        let left = F64Err::new_errorless(2.);
        let right = F64Err::new_errorless(3.);

        let res = left - right;

        assert_eq!(-1., res.val());
        assert_eq!(3., res.err());

        let res = res - right;

        assert_eq!(-4., res.val());
        assert_eq!(6., res.err());
    }
}