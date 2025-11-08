#include "icicle/backend/permutation_ops_backend.h"
#include "icicle/dispatcher.h"

namespace icicle {

  /*********************************** PERMUTATION EVALUATION ************************/
  ICICLE_DISPATCHER_INST(PermutationEvaluationDispatcher, permutation_evaluation, permutationEvaluationImpl);

  extern "C" eIcicleError CONCAT_EXPAND(ICICLE_FFI_PREFIX, permutation_evaluation)(
    const PermutationData<scalar_t>* perm_data,
    const PermutationConfig* config,
    scalar_t* results)
  {
      return PermutationEvaluationDispatcher::execute(
        *perm_data,
        *config,
        results
      );
  }

  eIcicleError permutation_evaluation(
    const PermutationData<scalar_t>& perm_data,
    const PermutationConfig& config,
    scalar_t* results)
  {
    return CONCAT_EXPAND(ICICLE_FFI_PREFIX, permutation_evaluation)(
      &perm_data,
      &config,
      results
    );
  }

  ICICLE_DISPATCHER_INST(PermutationDenominatorDispatcher, permutation_denominator, permutationDenominatorImpl);

  extern "C" eIcicleError CONCAT_EXPAND(ICICLE_FFI_PREFIX, permutation_denominator)(
    scalar_t* modified,
    const scalar_t* column,
    const scalar_t* permuted,
    uint32_t len,
    scalar_t beta,
    scalar_t gamma,
    const PermutationConfig* config)
  {
    return PermutationDenominatorDispatcher::execute(
      modified,
      column,
      permuted,
      len,
      beta,
      gamma,
      *config
    );
  }

  eIcicleError permutation_denominator_accumulate(
    scalar_t* modified,
    const scalar_t* column,
    const scalar_t* permuted,
    uint32_t len,
    scalar_t beta,
    scalar_t gamma,
    const PermutationConfig& config)
  {
    return CONCAT_EXPAND(ICICLE_FFI_PREFIX, permutation_denominator)(
      modified,
      column,
      permuted,
      len,
      beta,
      gamma,
      &config
    );
  }

  ICICLE_DISPATCHER_INST(PermutationNumeratorDispatcher, permutation_numerator, permutationNumeratorImpl);

  extern "C" eIcicleError CONCAT_EXPAND(ICICLE_FFI_PREFIX, permutation_numerator)(
    scalar_t* modified,
    const scalar_t* column,
    uint32_t len,
    scalar_t beta,
    scalar_t gamma,
    scalar_t delta_base,
    scalar_t omega,
    const PermutationConfig* config)
  {
    return PermutationNumeratorDispatcher::execute(
      modified,
      column,
      len,
      beta,
      gamma,
      delta_base,
      omega,
      *config
    );
  }

  eIcicleError permutation_numerator_accumulate(
    scalar_t* modified,
    const scalar_t* column,
    uint32_t len,
    scalar_t beta,
    scalar_t gamma,
    scalar_t delta_base,
    scalar_t omega,
    const PermutationConfig& config)
  {
    return CONCAT_EXPAND(ICICLE_FFI_PREFIX, permutation_numerator)(
      modified,
      column,
      len,
      beta,
      gamma,
      delta_base,
      omega,
      &config
    );
  }

  ICICLE_DISPATCHER_INST(PermutationPrefixDispatcher, permutation_prefix, permutationPrefixImpl);

  extern "C" eIcicleError CONCAT_EXPAND(ICICLE_FFI_PREFIX, permutation_prefix)(
    const scalar_t* fractions,
    scalar_t* output,
    uint32_t len,
    scalar_t last_z,
    const PermutationConfig* config)
  {
    return PermutationPrefixDispatcher::execute(
      fractions,
      output,
      len,
      last_z,
      *config
    );
  }

  eIcicleError permutation_prefix_product(
    const scalar_t* fractions,
    scalar_t* output,
    uint32_t len,
    scalar_t last_z,
    const PermutationConfig& config)
  {
    return CONCAT_EXPAND(ICICLE_FFI_PREFIX, permutation_prefix)(
      fractions,
      output,
      len,
      last_z,
      &config
    );
  }

} // namespace icicle

