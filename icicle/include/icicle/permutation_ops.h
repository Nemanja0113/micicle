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
    const T* extended_omega;
    uint32_t extended_len;
    uint32_t num_sets;
    uint32_t chunk_len;
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

} // namespace icicle

