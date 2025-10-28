#pragma once

#include <cstdint>
#include "icicle/errors.h"
#include "icicle/runtime.h"
#include "icicle/fields/field.h"
#include "icicle/config_extension.h"

namespace icicle {

/**
 * @brief Configuration for circuit element-wise operations.
 */
struct CircuitElementwiseConfig {
    icicleStreamHandle stream = nullptr; /** Stream for asynchronous execution. */
    bool is_a_on_device = false;         /** True if `a` is on the device, false if it is not. Default value: false. */
    bool is_b_on_device = false;         /** True if `b` is on the device, false if it is not. Default value: false. */
    bool is_result_on_device = false;    /** If true, the output is preserved on the device, otherwise on the host. Default value: false. */
    bool is_async = false;               /** Whether to run the operations asynchronously. */
    ConfigExtension* ext = nullptr;       /** Backend-specific extension. */
};

/**
 * @brief Returns the default value of CircuitElementwiseConfig.
 */
static CircuitElementwiseConfig default_circuit_elementwise_config() { 
    return CircuitElementwiseConfig{}; 
}

/**
 * @brief Adds two field elements element-wise using GPU acceleration.
 *
 * @tparam T Type of the field elements.
 * @param a First input vector.
 * @param b Second input vector.
 * @param size Size of the vectors.
 * @param config Configuration for the operation.
 * @param result Output vector.
 * @return eIcicleError Error code.
 */
template <typename T>
eIcicleError circuit_elementwise_add(
    const T* a, const T* b, uint64_t size, 
    const CircuitElementwiseConfig& config, T* result);

/**
 * @brief Subtracts two field elements element-wise using GPU acceleration.
 *
 * @tparam T Type of the field elements.
 * @param a First input vector.
 * @param b Second input vector.
 * @param size Size of the vectors.
 * @param config Configuration for the operation.
 * @param result Output vector.
 * @return eIcicleError Error code.
 */
template <typename T>
eIcicleError circuit_elementwise_sub(
    const T* a, const T* b, uint64_t size, 
    const CircuitElementwiseConfig& config, T* result);

/**
 * @brief Multiplies two field elements element-wise using GPU acceleration.
 *
 * @tparam T Type of the field elements.
 * @param a First input vector.
 * @param b Second input vector.
 * @param size Size of the vectors.
 * @param config Configuration for the operation.
 * @param result Output vector.
 * @return eIcicleError Error code.
 */
template <typename T>
eIcicleError circuit_elementwise_mult(
    const T* a, const T* b, uint64_t size, 
    const CircuitElementwiseConfig& config, T* result);

/**
 * @brief Computes multiplicative inverse of field elements element-wise using GPU acceleration.
 *
 * @tparam T Type of the field elements.
 * @param a Input vector.
 * @param size Size of the vector.
 * @param config Configuration for the operation.
 * @param result Output vector.
 * @return eIcicleError Error code.
 */
template <typename T>
eIcicleError circuit_elementwise_inverse(
    const T* a, uint64_t size, 
    const CircuitElementwiseConfig& config, T* result);

} // namespace icicle
