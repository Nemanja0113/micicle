#pragma once

#include <functional>
#include "icicle/permutation_ops.h"
#include "icicle/fields/field_config.h"
using namespace field_config;

namespace icicle {

  using permutationEvaluationImpl = std::function<eIcicleError(
    const Device& device,
    const PermutationData<scalar_t>& perm_data,
    const PermutationConfig& config,
    scalar_t* results)>;

  void register_permutation_evaluation(const std::string& deviceType, permutationEvaluationImpl impl);

#define REGISTER_PERMUTATION_EVALUATION_BACKEND(DEVICE_TYPE, FUNC)                                     \
  namespace {                                                                                          \
    static bool UNIQUE(_reg_permutation_evaluation) = []() -> bool {                                  \
      register_permutation_evaluation(DEVICE_TYPE, FUNC);                                             \
      return true;                                                                                     \
    }();                                                                                               \
  }

} // namespace icicle

