#pragma once

#include <functional>
#include "icicle/permutation_ops.h"
#include "icicle/device.h"
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

  using permutationDenominatorImpl = std::function<eIcicleError(
    const Device& device,
    scalar_t* modified,
    const scalar_t* column,
    const scalar_t* permuted,
    uint32_t len,
    scalar_t beta,
    scalar_t gamma,
    const PermutationConfig& config)>;

  void register_permutation_denominator(const std::string& deviceType, permutationDenominatorImpl impl);

#define REGISTER_PERMUTATION_DENOMINATOR_BACKEND(DEVICE_TYPE, FUNC)                                    \
  namespace {                                                                                          \
    static bool UNIQUE(_reg_permutation_denominator) = []() -> bool {                                  \
      register_permutation_denominator(DEVICE_TYPE, FUNC);                                             \
      return true;                                                                                     \
    }();                                                                                               \
  }

} // namespace icicle

