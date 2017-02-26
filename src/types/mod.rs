pub mod fp;

mod triangle;
mod point2;
mod triangulation;
mod t3_index;
mod n2_index;

mod point3;
mod tetrahedron;

pub use self::fp::Fp;

pub use self::triangle::*;
pub use self::point2::*;
pub use self::n2_index::*;
pub use self::t3_index::*;
pub use self::triangulation::*;

pub use self::point3::*;
pub use self::tetrahedron::*;