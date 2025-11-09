#pragma once

#include <cstdint>
#include "errors.h"
#include "runtime.h"
#include "icicle/fields/field_config.h"

namespace icicle {

struct PolyEvalBatchConfig {
  icicleStreamHandle stream{nullptr};
  bool is_async{false};
};

template <typename T>
struct PolyEvalBatch {
  const T* coeffs;
  const uint32_t* offsets;
  const uint32_t* lengths;
  const T* eval_points;
  uint32_t num_polys;
  PolyEvalBatchConfig config;
};

template <typename T>
eIcicleError polynomial_eval(
    const PolyEvalBatch<T>& batch,
    T* results);

} // namespace icicle
