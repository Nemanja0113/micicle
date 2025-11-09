#pragma once

#include <cstdint>
#include "errors.h"
#include "runtime.h"
#include "icicle/fields/field.h"

namespace icicle {

struct PolyEvalBatchConfig {
  icicleStreamHandle stream{nullptr};
  bool is_async{false};
};

struct PolyEvalBatch {
  const T* coeffs;      // concatenated coefficients
  const uint32_t* offsets; // start index per polynomial
  const uint32_t* lengths; // number of coefficients per polynomial
  const T* eval_points;    // evaluation point per polynomial
  uint32_t num_polys;
  PolyEvalBatchConfig config;
};

template <typename T>
eIcicleError polynomial_eval(
    const PolyEvalBatch<T>& batch,
    T* results);

} // namespace icicle
