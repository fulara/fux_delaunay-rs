#ifdef __cplusplus
extern "C" {
#endif

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

struct CApiTriangulation2 {
    CApiElement3* elements;
    size_t element_count;
};
typedef struct CApiTriangulation2 CApiTriangulation2;

void generate_triangulation2(CApiPoint2 *,size_t, CApiTriangulation2*);


struct CApiPoint3 {
    double x;
    double y;
    double z;
};
typedef struct CApiPoint3 CApiPoint3;

struct CApiPoint3Data {
    CApiPoint3 *points;
    size_t point_count;
};
typedef struct CApiPoint3Data CApiPoint3Data;

struct CApiElement4 {
    size_t v[4];
};
typedef struct CApiElement4 CApiElement4;

struct CApiTriangulation3 {
    CApiElement4* elements;
    size_t element_count;
};
typedef struct CApiTriangulation3 CApiTriangulation3;

void generate_triangulation3(CApiPoint3 *,size_t, CApiTriangulation3*);

#ifdef __cplusplus
}
#endif