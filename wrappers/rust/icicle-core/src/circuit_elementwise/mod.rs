use crate::traits::FieldImpl;
use icicle_runtime::{errors::eIcicleError, memory::HostOrDeviceSlice, stream::IcicleStreamHandle};

/// Configuration for circuit element-wise operations
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CircuitElementwiseConfig {
    pub stream_handle: IcicleStreamHandle,
    pub is_a_on_device: bool,
    pub is_b_on_device: bool,
    pub is_result_on_device: bool,
    pub is_async: bool,
}

impl CircuitElementwiseConfig {
    pub fn default() -> Self {
        Self {
            stream_handle: std::ptr::null_mut(),
            is_a_on_device: false,
            is_b_on_device: false,
            is_result_on_device: false,
            is_async: false,
        }
    }
}

impl Default for CircuitElementwiseConfig {
    fn default() -> Self {
        Self::default()
    }
}

#[doc(hidden)]
pub trait CircuitElementwiseOps<F: FieldImpl> {
    fn add(
        a: &(impl HostOrDeviceSlice<F> + ?Sized),
        b: &(impl HostOrDeviceSlice<F> + ?Sized),
        config: &CircuitElementwiseConfig,
        result: &mut (impl HostOrDeviceSlice<F> + ?Sized),
    ) -> Result<(), eIcicleError>;

    fn sub(
        a: &(impl HostOrDeviceSlice<F> + ?Sized),
        b: &(impl HostOrDeviceSlice<F> + ?Sized),
        config: &CircuitElementwiseConfig,
        result: &mut (impl HostOrDeviceSlice<F> + ?Sized),
    ) -> Result<(), eIcicleError>;

    fn mult(
        a: &(impl HostOrDeviceSlice<F> + ?Sized),
        b: &(impl HostOrDeviceSlice<F> + ?Sized),
        config: &CircuitElementwiseConfig,
        result: &mut (impl HostOrDeviceSlice<F> + ?Sized),
    ) -> Result<(), eIcicleError>;

    fn inverse(
        a: &(impl HostOrDeviceSlice<F> + ?Sized),
        config: &CircuitElementwiseConfig,
        result: &mut (impl HostOrDeviceSlice<F> + ?Sized),
    ) -> Result<(), eIcicleError>;
}

// Standalone functions that delegate to the trait methods
pub fn add<F>(
    a: &(impl HostOrDeviceSlice<F> + ?Sized),
    b: &(impl HostOrDeviceSlice<F> + ?Sized),
    config: &CircuitElementwiseConfig,
    result: &mut (impl HostOrDeviceSlice<F> + ?Sized),
) -> Result<(), eIcicleError>
where
    F: FieldImpl,
    <F as FieldImpl>::Config: CircuitElementwiseOps<F>,
{
    <<F as FieldImpl>::Config as CircuitElementwiseOps<F>>::add(a, b, config, result)
}

pub fn sub<F>(
    a: &(impl HostOrDeviceSlice<F> + ?Sized),
    b: &(impl HostOrDeviceSlice<F> + ?Sized),
    config: &CircuitElementwiseConfig,
    result: &mut (impl HostOrDeviceSlice<F> + ?Sized),
) -> Result<(), eIcicleError>
where
    F: FieldImpl,
    <F as FieldImpl>::Config: CircuitElementwiseOps<F>,
{
    <<F as FieldImpl>::Config as CircuitElementwiseOps<F>>::sub(a, b, config, result)
}

pub fn mult<F>(
    a: &(impl HostOrDeviceSlice<F> + ?Sized),
    b: &(impl HostOrDeviceSlice<F> + ?Sized),
    config: &CircuitElementwiseConfig,
    result: &mut (impl HostOrDeviceSlice<F> + ?Sized),
) -> Result<(), eIcicleError>
where
    F: FieldImpl,
    <F as FieldImpl>::Config: CircuitElementwiseOps<F>,
{
    <<F as FieldImpl>::Config as CircuitElementwiseOps<F>>::mult(a, b, config, result)
}

pub fn inverse<F>(
    a: &(impl HostOrDeviceSlice<F> + ?Sized),
    config: &CircuitElementwiseConfig,
    result: &mut (impl HostOrDeviceSlice<F> + ?Sized),
) -> Result<(), eIcicleError>
where
    F: FieldImpl,
    <F as FieldImpl>::Config: CircuitElementwiseOps<F>,
{
    <<F as FieldImpl>::Config as CircuitElementwiseOps<F>>::inverse(a, config, result)
}

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
            use icicle_core::circuit_elementwise::{CircuitElementwiseConfig};
            use icicle_runtime::{errors::eIcicleError, memory::HostOrDeviceSlice};

            extern "C" {
                #[link_name = concat!($field_prefix, "_circuit_elementwise_add")]
                pub(crate) fn circuit_elementwise_add_ffi(
                    a: *const $field,
                    b: *const $field,
                    size: u64,
                    config: *const CircuitElementwiseConfig,
                    result: *mut $field,
                ) -> eIcicleError;

                #[link_name = concat!($field_prefix, "_circuit_elementwise_sub")]
                pub(crate) fn circuit_elementwise_sub_ffi(
                    a: *const $field,
                    b: *const $field,
                    size: u64,
                    config: *const CircuitElementwiseConfig,
                    result: *mut $field,
                ) -> eIcicleError;

                #[link_name = concat!($field_prefix, "_circuit_elementwise_mult")]
                pub(crate) fn circuit_elementwise_mult_ffi(
                    a: *const $field,
                    b: *const $field,
                    size: u64,
                    config: *const CircuitElementwiseConfig,
                    result: *mut $field,
                ) -> eIcicleError;

                #[link_name = concat!($field_prefix, "_circuit_elementwise_inverse")]
                pub(crate) fn circuit_elementwise_inverse_ffi(
                    a: *const $field,
                    size: u64,
                    config: *const CircuitElementwiseConfig,
                    result: *mut $field,
                ) -> eIcicleError;
            }

            impl icicle_core::circuit_elementwise::CircuitElementwiseOps<$field> for $field_config {
                fn add(
                    a: &(impl HostOrDeviceSlice<$field> + ?Sized),
                    b: &(impl HostOrDeviceSlice<$field> + ?Sized),
                    config: &CircuitElementwiseConfig,
                    result: &mut (impl HostOrDeviceSlice<$field> + ?Sized),
                ) -> Result<(), eIcicleError> {
                    unsafe {
                        let error = circuit_elementwise_add_ffi(
                            a.as_ptr(),
                            b.as_ptr(),
                            a.len() as u64,
                            config as *const CircuitElementwiseConfig,
                            result.as_mut_ptr(),
                        );
                        if error == eIcicleError::IcicleSuccess {
                            Ok(())
                        } else {
                            Err(error)
                        }
                    }
                }

                fn sub(
                    a: &(impl HostOrDeviceSlice<$field> + ?Sized),
                    b: &(impl HostOrDeviceSlice<$field> + ?Sized),
                    config: &CircuitElementwiseConfig,
                    result: &mut (impl HostOrDeviceSlice<$field> + ?Sized),
                ) -> Result<(), eIcicleError> {
                    unsafe {
                        let error = circuit_elementwise_sub_ffi(
                            a.as_ptr(),
                            b.as_ptr(),
                            a.len() as u64,
                            config as *const CircuitElementwiseConfig,
                            result.as_mut_ptr(),
                        );
                        if error == eIcicleError::IcicleSuccess {
                            Ok(())
                        } else {
                            Err(error)
                        }
                    }
                }

                fn mult(
                    a: &(impl HostOrDeviceSlice<$field> + ?Sized),
                    b: &(impl HostOrDeviceSlice<$field> + ?Sized),
                    config: &CircuitElementwiseConfig,
                    result: &mut (impl HostOrDeviceSlice<$field> + ?Sized),
                ) -> Result<(), eIcicleError> {
                    unsafe {
                        let error = circuit_elementwise_mult_ffi(
                            a.as_ptr(),
                            b.as_ptr(),
                            a.len() as u64,
                            config as *const CircuitElementwiseConfig,
                            result.as_mut_ptr(),
                        );
                        if error == eIcicleError::IcicleSuccess {
                            Ok(())
                        } else {
                            Err(error)
                        }
                    }
                }

                fn inverse(
                    a: &(impl HostOrDeviceSlice<$field> + ?Sized),
                    config: &CircuitElementwiseConfig,
                    result: &mut (impl HostOrDeviceSlice<$field> + ?Sized),
                ) -> Result<(), eIcicleError> {
                    unsafe {
                        let error = circuit_elementwise_inverse_ffi(
                            a.as_ptr(),
                            a.len() as u64,
                            config as *const CircuitElementwiseConfig,
                            result.as_mut_ptr(),
                        );
                        if error == eIcicleError::IcicleSuccess {
                            Ok(())
                        } else {
                            Err(error)
                        }
                    }
                }
            }
        }
    };
}
