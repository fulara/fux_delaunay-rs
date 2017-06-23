#include "fux_delaunay_c_api.h"

#include <stdlib.h>

void set_point2(CApiPoint2* p, double x, double y) {
    p->x = x;
    p->y = y;
}

int test_c_api(CApiTriangulation2 *triangulation_data) {
    CApiPoint2 points[4];
    set_point2(&points[1], 1, 1);
    set_point2(&points[2], 0, 0);
    set_point2(&points[0], 0, 1);
    set_point2(&points[3], 1, 0);
    generate_triangulation2(points,4, triangulation_data);
    return 3;
}

void set_point3(CApiPoint3* p, double x, double y, double z) {
    p->x = x;
    p->y = y;
    p->z = z;
}

int test_c_api3(CApiTriangulation3 *triangulation_data) {
    CApiPoint3 points[8];
    set_point3(&points[0], 0., 0., 0.);
    set_point3(&points[1], 0., 1., 0.);
    set_point3(&points[2], 1., 1., 0.);
    set_point3(&points[3], 1., 0., 0.);
    set_point3(&points[4], 0., 0., 1.);
    set_point3(&points[5], 0., 1., 1.);
    set_point3(&points[6], 1., 1., 1.);
    set_point3(&points[7], 1., 0., 1.);
    generate_triangulation3(points,8, triangulation_data);
    return 3;
}