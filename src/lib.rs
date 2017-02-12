#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]

#[cfg(test)]
extern crate quickcheck;

extern crate cgmath;

pub mod math;
pub mod types;
pub mod io;
pub mod algorithms;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        //::cgmath::Point2::new();
        //::cgmath::Vector2::new();
    }
}
