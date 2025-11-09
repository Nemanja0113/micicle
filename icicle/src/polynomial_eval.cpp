#include "icicle/backend/polynomial_eval_backend.h"
#include "icicle/dispatcher.h"
#include "icicle/fields/field_config.h"

namespace icicle {

using namespace field_config;

ICICLE_DISPATCHER_INST(PolynomialEvalDispatcher, polynomial_eval, polynomialEvalImpl);

extern "C" eIcicleError CONCAT_EXPAND(ICICLE_FFI_PREFIX, polynomial_eval)(
    const PolyEvalBatch<scalar_t>* batch,
    scalar_t* results)
{
  return PolynomialEvalDispatcher::execute(*batch, results);
}

eIcicleError polynomial_eval(
    const PolyEvalBatch<scalar_t>& batch,
    scalar_t* results)
{
  return CONCAT_EXPAND(ICICLE_FFI_PREFIX, polynomial_eval)(&batch, results);
}

} // namespace icicle
