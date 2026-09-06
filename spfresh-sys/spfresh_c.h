#ifndef SPFRESH_C_H
#define SPFRESH_C_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct spfresh_index spfresh_index_t;

/* Lifecycle */
spfresh_index_t* spfresh_create(const char* algo, const char* value_type, int dim);
void spfresh_free(spfresh_index_t* idx);

/* Parameters */
void spfresh_set_build_param(spfresh_index_t* idx, const char* name, const char* value, const char* section);

/* Build */
int spfresh_build(spfresh_index_t* idx, const uint8_t* data, int num_vectors, int normalized);

/* Search */
int spfresh_search(spfresh_index_t* idx, const uint8_t* query, int result_num, int* out_ids, float* out_dists);

#ifdef __cplusplus
}
#endif

#endif
