use crate::traits::FieldLike;
use crate::errors::IcicleError;
use std::ffi::c_void;

/// Configuration for circuit element-wise operations
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CircuitElementwiseConfig {
    pub stream: *mut c_void,
    pub is_a_on_device: bool,
    pub is_b_on_device: bool,
    pub is_result_on_device: bool,
    pub is_async: bool,
    pub ext: *mut c_void,
}

impl Default for CircuitElementwiseConfig {
    fn default() -> Self {
        Self {
            stream: std::ptr::null_mut(),
            is_a_on_device: false,
            is_b_on_device: false,
            is_result_on_device: false,
            is_async: false,
            ext: std::ptr::null_mut(),
        }
    }
}

/// External CUDA function declarations
extern "C" {
    fn circuit_elementwise_add(
        a: *const u8,
        b: *const u8,
        size: u64,
        config: *const CircuitElementwiseConfig,
        result: *mut u8,
    ) -> u32;

    fn circuit_elementwise_sub(
        a: *const u8,
        b: *const u8,
        size: u64,
        config: *const CircuitElementwiseConfig,
        result: *mut u8,
    ) -> u32;

    fn circuit_elementwise_mult(
        a: *const u8,
        b: *const u8,
        size: u64,
        config: *const CircuitElementwiseConfig,
        result: *mut u8,
    ) -> u32;

    fn circuit_elementwise_inverse(
        a: *const u8,
        size: u64,
        config: *const CircuitElementwiseConfig,
        result: *mut u8,
    ) -> u32;
}

/// Circuit element-wise operations for BN254 field
pub struct CircuitElementwiseOps;

impl CircuitElementwiseOps {
    /// Adds two field element vectors element-wise using GPU acceleration
    pub fn add<F: FieldLike>(
        a: &[F],
        b: &[F],
        config: CircuitElementwiseConfig,
    ) -> Result<Vec<F>, IcicleError> {
        if a.len() != b.len() {
            return Err(IcicleError::InvalidInput("Vector lengths must match".to_string()));
        }

        let size = a.len() as u64;
        let mut result = vec![F::zero(); a.len()];

        unsafe {
            let cuda_error = circuit_elementwise_add(
                a.as_ptr() as *const u8,
                b.as_ptr() as *const u8,
                size,
                &config as *const CircuitElementwiseConfig,
                result.as_mut_ptr() as *mut u8,
            );

            if cuda_error != 0 {
                return Err(IcicleError::CudaError(cuda_error));
            }
        }

        Ok(result)
    }

    /// Subtracts two field element vectors element-wise using GPU acceleration
    pub fn sub<F: FieldLike>(
        a: &[F],
        b: &[F],
        config: CircuitElementwiseConfig,
    ) -> Result<Vec<F>, IcicleError> {
        if a.len() != b.len() {
            return Err(IcicleError::InvalidInput("Vector lengths must match".to_string()));
        }

        let size = a.len() as u64;
        let mut result = vec![F::zero(); a.len()];

        unsafe {
            let cuda_error = circuit_elementwise_sub(
                a.as_ptr() as *const u8,
                b.as_ptr() as *const u8,
                size,
                &config as *const CircuitElementwiseConfig,
                result.as_mut_ptr() as *mut u8,
            );

            if cuda_error != 0 {
                return Err(IcicleError::CudaError(cuda_error));
            }
        }

        Ok(result)
    }

    /// Multiplies two field element vectors element-wise using GPU acceleration
    pub fn mult<F: FieldLike>(
        a: &[F],
        b: &[F],
        config: CircuitElementwiseConfig,
    ) -> Result<Vec<F>, IcicleError> {
        if a.len() != b.len() {
            return Err(IcicleError::InvalidInput("Vector lengths must match".to_string()));
        }

        let size = a.len() as u64;
        let mut result = vec![F::zero(); a.len()];

        unsafe {
            let cuda_error = circuit_elementwise_mult(
                a.as_ptr() as *const u8,
                b.as_ptr() as *const u8,
                size,
                &config as *const CircuitElementwiseConfig,
                result.as_mut_ptr() as *mut u8,
            );

            if cuda_error != 0 {
                return Err(IcicleError::CudaError(cuda_error));
            }
        }

        Ok(result)
    }

    /// Computes multiplicative inverse of field elements element-wise using GPU acceleration
    pub fn inverse<F: FieldLike>(
        a: &[F],
        config: CircuitElementwiseConfig,
    ) -> Result<Vec<F>, IcicleError> {
        let size = a.len() as u64;
        let mut result = vec![F::zero(); a.len()];

        unsafe {
            let cuda_error = circuit_elementwise_inverse(
                a.as_ptr() as *const u8,
                size,
                &config as *const CircuitElementwiseConfig,
                result.as_mut_ptr() as *mut u8,
            );

            if cuda_error != 0 {
                return Err(IcicleError::CudaError(cuda_error));
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::FieldLike;
    use halo2curves::bn256::Fr as Fp;

    #[test]
    fn test_elementwise_add() {
        let a = vec![Fp::from(1), Fp::from(2), Fp::from(3)];
        let b = vec![Fp::from(4), Fp::from(5), Fp::from(6)];
        let config = CircuitElementwiseConfig::default();

        let result = CircuitElementwiseOps::add(&a, &b, config).unwrap();
        let expected = vec![Fp::from(5), Fp::from(7), Fp::from(9)];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_elementwise_sub() {
        let a = vec![Fp::from(5), Fp::from(7), Fp::from(9)];
        let b = vec![Fp::from(1), Fp::from(2), Fp::from(3)];
        let config = CircuitElementwiseConfig::default();

        let result = CircuitElementwiseOps::sub(&a, &b, config).unwrap();
        let expected = vec![Fp::from(4), Fp::from(5), Fp::from(6)];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_elementwise_mult() {
        let a = vec![Fp::from(2), Fp::from(3), Fp::from(4)];
        let b = vec![Fp::from(5), Fp::from(6), Fp::from(7)];
        let config = CircuitElementwiseConfig::default();

        let result = CircuitElementwiseOps::mult(&a, &b, config).unwrap();
        let expected = vec![Fp::from(10), Fp::from(18), Fp::from(28)];

        assert_eq!(result, expected);
    }
}

