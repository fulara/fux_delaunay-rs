#include <stddef.h>

struct CApiPoint2 {
    double x;
    double y;
};
typedef struct CApiPoint2 CApiPoint2;

struct CApiPoint2Data {
    CApiPoint2 *points;
    size_t point_count;
};
typedef struct CApiPoint2Data CApiPoint2Data;

struct CApiElement3 {
    size_t v[3];
};
typedef struct CApiElement3 CApiElement3;

struct CApiTriangulation {
    CApiElement3* elements;
    size_t element_count;
};
typedef struct CApiTriangulation CApiTriangulation;

CApiTriangulation generate_triangulation(CApiPoint2 *,size_t);