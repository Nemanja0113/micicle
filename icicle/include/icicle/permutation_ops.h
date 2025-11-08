#pragma once

#include <cstdint>
#include "errors.h"
#include "runtime.h"
#include "icicle/fields/field.h"

namespace icicle {

  struct PermutationConfig {
    icicleStreamHandle stream;
    bool is_data_on_device;
    bool is_async;
  };

  template <typename T>
  struct PermutationData {
    const T* const* permutation_products;
    const T* fixed_cosets;
    const T* advice_cosets;
    const T* instance_cosets;
    const T* const* perm_cosets;
    const T* l0;
    const T* l_last;
    const T* l_active_row;
    const int* column_types;
    const int* column_indices;
    const T* beta;
    const T* gamma;
    const T* y;
    const T* delta_start;
    const T* delta_const;
    const T* extended_omega;
    uint32_t extended_len;
    uint32_t num_sets;
    uint32_t chunk_len;
    uint32_t num_perm_cosets;  // Total number of permutation cosets
    uint32_t num_fixed;
    uint32_t num_advice;
    uint32_t num_instance;
    uint32_t rot_scale;
    int32_t isize;
    int32_t blinding_factors;
  };

  static PermutationConfig default_permutation_config()
  {
    return PermutationConfig{
      nullptr,  // stream
      false,    // is_data_on_device
      false     // is_async
    };
  }

  template <typename T>
  eIcicleError permutation_evaluation(
    const PermutationData<T>& perm_data,
    const PermutationConfig& config,
    T* results
  );

  template <typename T>
  eIcicleError permutation_denominator_accumulate(
    T* modified,
    const T* column,
    const T* permuted,
    uint32_t len,
    T beta,
    T gamma,
    const PermutationConfig& config
  );

  template <typename T>
  eIcicleError permutation_numerator_accumulate(
    T* modified,
    const T* column,
    uint32_t len,
    T beta,
    T gamma,
    T delta_base,
    T omega,
    const PermutationConfig& config
  );

  template <typename T>
  eIcicleError permutation_prefix_product(
    const T* fractions,
    T* output,
    uint32_t len,
    T last_z,
    const PermutationConfig& config
  );

} // namespace icicle

