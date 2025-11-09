'use'
use crate::traits::FieldImpl;
use icicle_runtime::{errors::eIcicleError, memory::HostOrDeviceSlice, stream::IcicleStreamHandle};
use std::ptr;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PolyEvalBatchConfig {
    pub stream_handle: IcicleStreamHandle,
    pub is_async: bool,
}

impl Default for PolyEvalBatchConfig {
    fn default() -> Self {
        Self {
            stream_handle: ptr::null_mut(),
            is_async: false,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PolyEvalBatch<T> {
    pub coeffs: *const T,
    pub offsets: *const u32,
    pub lengths: *const u32,
    pub eval_points: *const T,
    pub num_polys: u32,
    pub config: PolyEvalBatchConfig,
}

impl<T> PolyEvalBatch<T> {
    pub fn new(
        coeffs: *const T,
        offsets: *const u32,
        lengths: *const u32,
        eval_points: *const T,
        num_polys: u32,
        config: PolyEvalBatchConfig,
    ) -> Self {
        Self {
            coeffs,
            offsets,
            lengths,
            eval_points,
            num_polys,
            config,
        }
    }
}

#[doc(hidden)]
pub trait PolynomialEvalOps<F: FieldImpl> {
    fn polynomial_eval(
        batch: &PolyEvalBatch<F>,
        results: &mut (impl HostOrDeviceSlice<F> + ?Sized),
    ) -> Result<(), eIcicleError>;
}

pub fn polynomial_eval<F>(
    batch: &PolyEvalBatch<F>,
    results: &mut (impl HostOrDeviceSlice<F> + ?Sized),
) -> Result<(), eIcicleError>
where
    F: FieldImpl,
    <F as FieldImpl>::Config: PolynomialEvalOps<F>,
{
    <<F as FieldImpl>::Config as PolynomialEvalOps<F>>::polynomial_eval(batch, results)
}

#[macro_export]
macro_rules! impl_polynomial_eval_ops {
    ($field_prefix:literal, $field:ident, $field_config:ident) => {
        const _: () = {
            use icicle_core::polynomial_eval::PolyEvalBatch;
            use icicle_runtime::errors::eIcicleError;
            use icicle_runtime::memory::HostOrDeviceSlice;

            extern "C" {
                #[link_name = concat!($field_prefix, "_polynomial_eval")]
                fn polynomial_eval_ffi(
                    batch: *const PolyEvalBatch<$field>,
                    results: *mut $field,
                ) -> eIcicleError;
            }

            impl icicle_core::polynomial_eval::PolynomialEvalOps<$field> for $field_config {
                fn polynomial_eval(
                    batch: &PolyEvalBatch<$field>,
                    results: &mut (impl HostOrDeviceSlice<$field> + ?Sized),
                ) -> Result<(), eIcicleError> {
                    unsafe {
                        polynomial_eval_ffi(batch as *const _, results.as_mut_ptr()).wrap()
                    }
                }
            }
        };
    };
}
