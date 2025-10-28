#include "icicle/backend/circuit_elementwise_backend.h"
#include "icicle/dispatcher.h"

namespace icicle {

  /*********************************** CIRCUIT ELEMENTWISE OPERATIONS ************************/
  ICICLE_DISPATCHER_INST(CircuitElementwiseDispatcher, circuit_elementwise_add, circuitElementwiseAddImpl);
  ICICLE_DISPATCHER_INST(CircuitElementwiseDispatcher, circuit_elementwise_sub, circuitElementwiseSubImpl);
  ICICLE_DISPATCHER_INST(CircuitElementwiseDispatcher, circuit_elementwise_mult, circuitElementwiseMultImpl);
  ICICLE_DISPATCHER_INST(CircuitElementwiseDispatcher, circuit_elementwise_inverse, circuitElementwiseInverseImpl);

  extern "C" eIcicleError CONCAT_EXPAND(ICICLE_FFI_PREFIX, circuit_elementwise_add)(
    const scalar_t* a,
    const scalar_t* b,
    uint64_t size,
    const CircuitElementwiseConfig* config,
    scalar_t* result)
  {
      return CircuitElementwiseDispatcher::execute(
        a,
        b,
        size,
        *config,
        result
      );
  }

  extern "C" eIcicleError CONCAT_EXPAND(ICICLE_FFI_PREFIX, circuit_elementwise_sub)(
    const scalar_t* a,
    const scalar_t* b,
    uint64_t size,
    const CircuitElementwiseConfig* config,
    scalar_t* result)
  {
      return CircuitElementwiseDispatcher::execute(
        a,
        b,
        size,
        *config,
        result
      );
  }

  extern "C" eIcicleError CONCAT_EXPAND(ICICLE_FFI_PREFIX, circuit_elementwise_mult)(
    const scalar_t* a,
    const scalar_t* b,
    uint64_t size,
    const CircuitElementwiseConfig* config,
    scalar_t* result)
  {
      return CircuitElementwiseDispatcher::execute(
        a,
        b,
        size,
        *config,
        result
      );
  }

  extern "C" eIcicleError CONCAT_EXPAND(ICICLE_FFI_PREFIX, circuit_elementwise_inverse)(
    const scalar_t* a,
    uint64_t size,
    const CircuitElementwiseConfig* config,
    scalar_t* result)
  {
      return CircuitElementwiseDispatcher::execute(
        a,
        size,
        *config,
        result
      );
  }

  eIcicleError circuit_elementwise_add(
    const scalar_t* a,
    const scalar_t* b,
    uint64_t size,
    const CircuitElementwiseConfig& config,
    scalar_t* result)
  {
    return CONCAT_EXPAND(ICICLE_FFI_PREFIX, circuit_elementwise_add)(
      a,
      b,
      size,
      &config,
      result
    );
  }

  eIcicleError circuit_elementwise_sub(
    const scalar_t* a,
    const scalar_t* b,
    uint64_t size,
    const CircuitElementwiseConfig& config,
    scalar_t* result)
  {
    return CONCAT_EXPAND(ICICLE_FFI_PREFIX, circuit_elementwise_sub)(
      a,
      b,
      size,
      &config,
      result
    );
  }

  eIcicleError circuit_elementwise_mult(
    const scalar_t* a,
    const scalar_t* b,
    uint64_t size,
    const CircuitElementwiseConfig& config,
    scalar_t* result)
  {
    return CONCAT_EXPAND(ICICLE_FFI_PREFIX, circuit_elementwise_mult)(
      a,
      b,
      size,
      &config,
      result
    );
  }

  eIcicleError circuit_elementwise_inverse(
    const scalar_t* a,
    uint64_t size,
    const CircuitElementwiseConfig& config,
    scalar_t* result)
  {
    return CONCAT_EXPAND(ICICLE_FFI_PREFIX, circuit_elementwise_inverse)(
      a,
      size,
      &config,
      result
    );
  }

} // namespace icicle