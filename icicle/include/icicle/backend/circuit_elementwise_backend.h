#pragma once

#include <functional>
#include "icicle/circuit_elementwise.h"
#include "icicle/fields/field_config.h"
using namespace field_config;

namespace icicle {

  using circuitElementwiseAddImpl = std::function<eIcicleError(
    const Device& device,
    const scalar_t* a,
    const scalar_t* b,
    uint64_t size,
    const CircuitElementwiseConfig& config,
    scalar_t* result)>;

  using circuitElementwiseSubImpl = std::function<eIcicleError(
    const Device& device,
    const scalar_t* a,
    const scalar_t* b,
    uint64_t size,
    const CircuitElementwiseConfig& config,
    scalar_t* result)>;

  using circuitElementwiseMultImpl = std::function<eIcicleError(
    const Device& device,
    const scalar_t* a,
    const scalar_t* b,
    uint64_t size,
    const CircuitElementwiseConfig& config,
    scalar_t* result)>;

  using circuitElementwiseInverseImpl = std::function<eIcicleError(
    const Device& device,
    const scalar_t* a,
    uint64_t size,
    const CircuitElementwiseConfig& config,
    scalar_t* result)>;

  void register_circuit_elementwise_add(const std::string& deviceType, circuitElementwiseAddImpl impl);
  void register_circuit_elementwise_sub(const std::string& deviceType, circuitElementwiseSubImpl impl);
  void register_circuit_elementwise_mult(const std::string& deviceType, circuitElementwiseMultImpl impl);
  void register_circuit_elementwise_inverse(const std::string& deviceType, circuitElementwiseInverseImpl impl);

#define REGISTER_CIRCUIT_ELEMENTWISE_ADD_BACKEND(DEVICE_TYPE, FUNC)                                     \
  namespace {                                                                                          \
    static bool UNIQUE(_reg_circuit_elementwise_add) = []() -> bool {                                  \
      register_circuit_elementwise_add(DEVICE_TYPE, FUNC);                                             \
      return true;                                                                                     \
    }();                                                                                               \
  }

#define REGISTER_CIRCUIT_ELEMENTWISE_SUB_BACKEND(DEVICE_TYPE, FUNC)                                     \
  namespace {                                                                                          \
    static bool UNIQUE(_reg_circuit_elementwise_sub) = []() -> bool {                                  \
      register_circuit_elementwise_sub(DEVICE_TYPE, FUNC);                                             \
      return true;                                                                                     \
    }();                                                                                               \
  }

#define REGISTER_CIRCUIT_ELEMENTWISE_MULT_BACKEND(DEVICE_TYPE, FUNC)                                     \
  namespace {                                                                                          \
    static bool UNIQUE(_reg_circuit_elementwise_mult) = []() -> bool {                                  \
      register_circuit_elementwise_mult(DEVICE_TYPE, FUNC);                                             \
      return true;                                                                                     \
    }();                                                                                               \
  }

#define REGISTER_CIRCUIT_ELEMENTWISE_INVERSE_BACKEND(DEVICE_TYPE, FUNC)                                     \
  namespace {                                                                                          \
    static bool UNIQUE(_reg_circuit_elementwise_inverse) = []() -> bool {                                  \
      register_circuit_elementwise_inverse(DEVICE_TYPE, FUNC);                                             \
      return true;                                                                                     \
    }();                                                                                               \
  }

} // namespace icicle
