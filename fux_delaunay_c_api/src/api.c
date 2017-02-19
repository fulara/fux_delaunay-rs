#include "fux_delaunay_c_api.h"

#include <stdlib.h>

void set_point(CApiPoint2* p, double x, double y) {
    p->x = x;
    p->y = y;
}

int test_c_api(CApiTriangulation *triangulation_data) {
    CApiTriangulation *triangulation_data
    CApiPoint2* points = malloc(sizeof(CApiPoint2) * 4);
    set_point(&points[1], 1, 1);
    set_point(&points[2], 0, 0);
    set_point(&points[0], 0, 1);
    set_point(&points[3], 1, 0);
    generate_triangulation(points,4, triangulation_data);
    return 3;
}