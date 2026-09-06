#include "spfresh_c.h"
#include "inc/Core/VectorIndex.h"
#include "inc/Core/SearchQuery.h"
#include "inc/Helper/StringConvert.h"
#include <cstring>

struct spfresh_index {
    std::shared_ptr<SPTAG::VectorIndex> index;
    SPTAG::DimensionType dimension;
};

extern "C" {

spfresh_index_t* spfresh_create(const char* algo, const char* value_type, int dim) {
    SPTAG::IndexAlgoType algo_type = SPTAG::IndexAlgoType::Undefined;
    SPTAG::VectorValueType vt = SPTAG::VectorValueType::Undefined;
    SPTAG::Helper::Convert::ConvertStringTo<SPTAG::IndexAlgoType>(algo, algo_type);
    SPTAG::Helper::Convert::ConvertStringTo<SPTAG::VectorValueType>(value_type, vt);

    auto index = SPTAG::VectorIndex::CreateInstance(algo_type, vt);
    if (!index) return nullptr;

    auto* w = new spfresh_index_t;
    w->index = index;
    w->dimension = dim;
    return w;
}

void spfresh_free(spfresh_index_t* idx) {
    delete idx;
}

void spfresh_set_build_param(spfresh_index_t* idx, const char* name, const char* value, const char* section) {
    idx->index->SetParameter(name, value, section);
}

int spfresh_build(spfresh_index_t* idx, const uint8_t* data, int num_vectors, int normalized) {
    size_t sz = SPTAG::GetValueTypeSize(idx->index->GetVectorValueType());
    SPTAG::ByteArray arr((uint8_t*)data, num_vectors * idx->dimension * sz, false);
    auto ec = idx->index->BuildIndex(arr.Data(), num_vectors, idx->dimension, normalized != 0);
    return (ec == SPTAG::ErrorCode::Success) ? 0 : -1;
}

int spfresh_search(spfresh_index_t* idx, const uint8_t* query, int result_num, int* out_ids, float* out_dists) {
    SPTAG::QueryResult qr(query, result_num, false);
    auto ec = idx->index->SearchIndex(qr);
    if (ec != SPTAG::ErrorCode::Success) return -1;

    for (int i = 0; i < result_num; ++i) {
        auto* r = qr.GetResult(i);
        out_ids[i] = r->VID;
        out_dists[i] = r->Dist;
    }
    return 0;
}

}
