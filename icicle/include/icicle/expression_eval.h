#pragma once

#include <cstdint>
#include "icicle/runtime.h"
#include "icicle/fields/field_config.h"

using namespace field_config;

namespace icicle {

  struct ExpressionEvalConfig {
    void* stream;
    bool is_async;
    bool is_data_on_device;
  };

  template <typename T>
  struct ExpressionEvalData {
    const T* const* fixed_columns;
    const T* const* advice_columns;
    const T* const* instance_columns;
    const T* constants;
    const T* challenges;
    const int* rotations;
    const uint8_t* calculations;
    const uint32_t* targets;
    const uint8_t* value_types;
    const uint32_t* value_indices;
    uint32_t domain_size;
    uint32_t num_fixed;
    uint32_t num_advice;
    uint32_t num_instance;
    uint32_t num_constants;
    uint32_t num_challenges;
    uint32_t num_rotations;
    uint32_t num_calculations;
    uint32_t num_intermediates;
    int32_t rot_scale;
    int32_t isize;
  };

  static ExpressionEvalConfig default_expression_eval_config()
  {
    return ExpressionEvalConfig{
      nullptr,  // stream
      false,    // is_async
      false,    // is_data_on_device
    };
  }

  /**
   * Evaluate an expression on regular domain
   */
  eIcicleError expression_evaluation(
    const ExpressionEvalData<scalar_t>& eval_data,
    const ExpressionEvalConfig& config,
    scalar_t* results);

} // namespace icicle

