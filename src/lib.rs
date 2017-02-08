#[cfg(test)]
#[macro_use]
extern crate quickcheck;

extern crate cgmath;

pub mod math;
pub mod types;
pub mod io;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        //::cgmath::Point2::new();
        //::cgmath::Vector2::new();
    }
}
