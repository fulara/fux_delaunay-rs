# delaunay_rust
Project to generate delaunay conforming triangulation in 2d and 3d.

## 2d
2d triangulation is currently constrained to be in a form of rectangle. You could generate a triangulation for something else but to do that, you'd have to remove the border nodes after triangulating.  

To generate triangulation:  
trivial_tests_1 - generates simple triangulation with 4 points, and then also insert one points after that. Example below also showcases the usage of exporting the triangulation to abaqus. When creating a triangulation the best approach would be just to pass all the nodes into the `Triangulation::new` method. You can also build the triangulation incrementally, by invoking the insert_node method.
```
#[test]
fn trivial_tests_1() {
    // Triangulation::new requires at least  four nodes! They will make a border of the triangulation.
    let mut triangulation = Triangulation::new(&[Point2::new(0., 10.), Point2::new(10., 10.), Point2::new(0., 0.), Point2::new(10., 0.)]);

    triangulation.insert_node(&Point2::new(2., 2.));

    abaqus_write::write_to_abaqus_format("tests/tests_results/trivial_tests_1.inp", &triangulation);
}
```
To get the information about created elements you can iterate over the `triangulation.elements()` vec.  
```
#[inline]
pub fn elements(&self) -> &Vec<Triangle> {
    &self.elements
}
```
Where Triangle is a structure:
```
#[derive(Debug, PartialEq)]
pub struct Triangle {
    v: [N2Index; 3],
    n: [Option<T3Index>; 3]
}
```
### 2d C api
Crate exposes also C api. See Subproject _c_api. To use C-api you have to use the header file `fux_delaunay_c_api.h` provided by the _c_api subproject. Examplary usagein C:  
```
#include <stdlib.h>

#include "fux_delaunay_c_api.h"

void example_c() {
    CApiTriangulation *triangulation_data
    CApiPoint2* points = malloc(sizeof(CApiPoint2) * 4);
    set_point(&points[1], 1, 1);
    set_point(&points[2], 0, 0);
    set_point(&points[0], 0, 1);
    set_point(&points[3], 1, 0);
    generate_triangulation(points,4, triangulation_data);
}    
```
You can also find this code in the _c_api subproject.
## 3d
Currently work in progress, so nothing here so far.
