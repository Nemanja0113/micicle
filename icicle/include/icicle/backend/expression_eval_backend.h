#pragma once

#include <functional>
#include "icicle/expression_eval.h"
#include "icicle/fields/field_config.h"
using namespace field_config;

namespace icicle {

  using expressionEvaluationImpl = std::function<eIcicleError(
    const Device& device,
    const ExpressionEvalData<scalar_t>& eval_data,
    const ExpressionEvalConfig& config,
    scalar_t* results)>;

  void register_expression_evaluation(const std::string& deviceType, expressionEvaluationImpl impl);

#define REGISTER_EXPRESSION_EVALUATION_BACKEND(DEVICE_TYPE, FUNC)                                     \
  namespace {                                                                                          \
    static bool UNIQUE(_reg_expression_evaluation) = []() -> bool {                                   \
      register_expression_evaluation(DEVICE_TYPE, FUNC);                                              \
      return true;                                                                                     \
    }();                                                                                               \
  }

} // namespace icicle

