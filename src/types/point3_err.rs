use types::Point3;
use math::F64Err;

pub type Point3Err = ::cgmath::Point3<F64Err>;

pub fn point3_err_from_point3(p : &Point3) -> Point3Err {
    Point3Err::new(F64Err::new_errorless(p.x),F64Err::new_errorless(p.y),F64Err::new_errorless(p.z))
}