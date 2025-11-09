#pragma once

#include <functional>
#include <string>
#include "icicle/polynomial_eval.h"
#include "icicle/device.h"
#include "icicle/fields/field_config.h"

namespace icicle {

using field_config::scalar_t;

using polynomialEvalImpl = std::function<eIcicleError(
    const Device& device,
    const PolyEvalBatch<scalar_t>& batch,
    scalar_t* results)>;

void register_polynomial_eval(const std::string& deviceType, polynomialEvalImpl impl);

#define REGISTER_POLYNOMIAL_EVAL_BACKEND(DEVICE_TYPE, FUNC) \
  namespace {                                                \
    static bool UNIQUE(_reg_polynomial_eval) = []() -> bool { \
      register_polynomial_eval(DEVICE_TYPE, FUNC);            \
      return true;                                            \
    }();                                                      \
  }

} // namespace icicle
