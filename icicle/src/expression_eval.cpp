#include "icicle/backend/expression_eval_backend.h"
#include "icicle/dispatcher.h"

namespace icicle {

  ICICLE_DISPATCHER_INST(ExpressionEvaluationDispatcher, expression_evaluation, expressionEvaluationImpl);

  extern "C" eIcicleError CONCAT_EXPAND(ICICLE_FFI_PREFIX, expression_evaluation)(
    const ExpressionEvalData<scalar_t>* eval_data, 
    const ExpressionEvalConfig* config,
    scalar_t* results)
  {
      return ExpressionEvaluationDispatcher::execute(
        *eval_data, 
        *config,
        results
      );
  }

  eIcicleError expression_evaluation(
    const ExpressionEvalData<scalar_t>& eval_data, 
    const ExpressionEvalConfig& config,
    scalar_t* results)
  {
    return CONCAT_EXPAND(ICICLE_FFI_PREFIX, expression_evaluation)(
      &eval_data, 
      &config,
      results
    );
  }

} // namespace icicle

