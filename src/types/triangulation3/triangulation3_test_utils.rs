use types::Point3;

pub fn get_example_initial_point_set() -> Vec<Point3> {
    vec![Point3::new(0., 0., 0.),
         Point3::new(0., 1., 0.),
         Point3::new(1., 1., 0.),
         Point3::new(1., 0., 0.),
         Point3::new(0., 0., 1.),
         Point3::new(0., 1., 1.),
         Point3::new(1., 1., 1.),
         Point3::new(1., 0., 1.)]
}
