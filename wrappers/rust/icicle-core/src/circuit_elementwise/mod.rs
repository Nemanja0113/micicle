use crate::traits::FieldImpl;
use icicle_runtime::errors::eIcicleError;
use std::ffi::c_void;

/// Configuration for circuit element-wise operations
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CircuitElementwiseConfig {
    pub stream_handle: *mut c_void,
    pub is_data_on_device: bool,
    pub is_async: bool,
}

impl Default for CircuitElementwiseConfig {
    fn default() -> Self {
        Self {
            stream_handle: std::ptr::null_mut(),
            is_data_on_device: false,
            is_async: false,
        }
    }
}

/// Circuit element-wise operations trait
pub trait CircuitElementwiseOps<F: FieldImpl> {
    fn add(
        a: &[F],
        b: &[F],
        config: CircuitElementwiseConfig,
    ) -> Result<Vec<F>, eIcicleError>;

    fn sub(
        a: &[F],
        b: &[F],
        config: CircuitElementwiseConfig,
    ) -> Result<Vec<F>, eIcicleError>;

    fn mult(
        a: &[F],
        b: &[F],
        config: CircuitElementwiseConfig,
    ) -> Result<Vec<F>, eIcicleError>;

    fn inverse(
        a: &[F],
        config: CircuitElementwiseConfig,
    ) -> Result<Vec<F>, eIcicleError>;
}

/// Macro to implement circuit element-wise operations for specific fields
#[macro_export]
macro_rules! impl_circuit_elementwise_ops {
    (
        $field_prefix:literal,
        $field_prefix_ident:ident,
        $field:ident,
        $field_config:ident
    ) => {
        mod $field_prefix_ident {
            use super::{$field, $field_config};
            use icicle_core::circuit_elementwise::{CircuitElementwiseConfig, CircuitElementwiseOps};
            use icicle_runtime::{errors::eIcicleError, memory::HostOrDeviceSlice};

            extern "C" {
                #[link_name = concat!($field_prefix, "_circuit_elementwise_add")]
                pub(crate) fn circuit_elementwise_add_ffi(
                    a: *const u8,
                    b: *const u8,
                    size: u64,
                    config: *const CircuitElementwiseConfig,
                    result: *mut u8,
                ) -> eIcicleError;

                #[link_name = concat!($field_prefix, "_circuit_elementwise_sub")]
                pub(crate) fn circuit_elementwise_sub_ffi(
                    a: *const u8,
                    b: *const u8,
                    size: u64,
                    config: *const CircuitElementwiseConfig,
                    result: *mut u8,
                ) -> eIcicleError;

                #[link_name = concat!($field_prefix, "_circuit_elementwise_mult")]
                pub(crate) fn circuit_elementwise_mult_ffi(
                    a: *const u8,
                    b: *const u8,
                    size: u64,
                    config: *const CircuitElementwiseConfig,
                    result: *mut u8,
                ) -> eIcicleError;

                #[link_name = concat!($field_prefix, "_circuit_elementwise_inverse")]
                pub(crate) fn circuit_elementwise_inverse_ffi(
                    a: *const u8,
                    size: u64,
                    config: *const CircuitElementwiseConfig,
                    result: *mut u8,
                ) -> eIcicleError;
            }
        }

        impl icicle_core::circuit_elementwise::CircuitElementwiseOps<$field> for $field_config {
            fn add(
                a: &[$field],
                b: &[$field],
                config: icicle_core::circuit_elementwise::CircuitElementwiseConfig,
            ) -> Result<Vec<$field>, icicle_runtime::errors::eIcicleError> {
                if a.len() != b.len() {
                    return Err(eIcicleError::InvalidArgument);
                }

                let size = a.len() as u64;
                let mut result = vec![$field::zero(); a.len()];

                unsafe {
                    $field_prefix_ident::circuit_elementwise_add_ffi(
                        a.as_ptr() as *const u8,
                        b.as_ptr() as *const u8,
                        size,
                        &config as *const CircuitElementwiseConfig,
                        result.as_mut_ptr() as *mut u8,
                    )
                    .wrap()
                }?;

                Ok(result)
            }

            fn sub(
                a: &[$field],
                b: &[$field],
                config: icicle_core::circuit_elementwise::CircuitElementwiseConfig,
            ) -> Result<Vec<$field>, icicle_runtime::errors::eIcicleError> {
                if a.len() != b.len() {
                    return Err(eIcicleError::InvalidArgument);
                }

                let size = a.len() as u64;
                let mut result = vec![$field::zero(); a.len()];

                unsafe {
                    $field_prefix_ident::circuit_elementwise_sub_ffi(
                        a.as_ptr() as *const u8,
                        b.as_ptr() as *const u8,
                        size,
                        &config as *const CircuitElementwiseConfig,
                        result.as_mut_ptr() as *mut u8,
                    )
                    .wrap()
                }?;

                Ok(result)
            }

            fn mult(
                a: &[$field],
                b: &[$field],
                config: icicle_core::circuit_elementwise::CircuitElementwiseConfig,
            ) -> Result<Vec<$field>, icicle_runtime::errors::eIcicleError> {
                if a.len() != b.len() {
                    return Err(eIcicleError::InvalidArgument);
                }

                let size = a.len() as u64;
                let mut result = vec![$field::zero(); a.len()];

                unsafe {
                    $field_prefix_ident::circuit_elementwise_mult_ffi(
                        a.as_ptr() as *const u8,
                        b.as_ptr() as *const u8,
                        size,
                        &config as *const CircuitElementwiseConfig,
                        result.as_mut_ptr() as *mut u8,
                    )
                    .wrap()
                }?;

                Ok(result)
            }

            fn inverse(
                a: &[$field],
                config: icicle_core::circuit_elementwise::CircuitElementwiseConfig,
            ) -> Result<Vec<$field>, icicle_runtime::errors::eIcicleError> {
                let size = a.len() as u64;
                let mut result = vec![$field::zero(); a.len()];

                unsafe {
                    $field_prefix_ident::circuit_elementwise_inverse_ffi(
                        a.as_ptr() as *const u8,
                        size,
                        &config as *const CircuitElementwiseConfig,
                        result.as_mut_ptr() as *mut u8,
                    )
                    .wrap()
                }?;

                Ok(result)
            }
        }
    };
}
