use crate::traits::FieldImpl;
use icicle_runtime::{errors::eIcicleError, memory::HostOrDeviceSlice, stream::IcicleStreamHandle};

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PolyEvalBatchConfig {
    pub stream_handle: IcicleStreamHandle,
    pub is_async: bool,
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
    ($field_prefix:literal, $field_prefix_ident:ident, $field:ident, $field_config:ident) => {
        mod $field_prefix_ident {
            use super::{$field, $field_config};
            use icicle_core::polynomial_eval::{PolyEvalBatch, PolyEvalBatchConfig};
            use icicle_runtime::{errors::eIcicleError, memory::HostOrDeviceSlice};

            extern "C" {
                #[link_name = concat!($field_prefix, "_polynomial_eval")]
                pub(crate) fn polynomial_eval_ffi(
                    batch: *const PolyEvalBatch<$field>,
                    results: *mut $field,
                ) -> eIcicleError;
            }
        }

        impl icicle_core::polynomial_eval::PolynomialEvalOps<$field> for $field_config {
            fn polynomial_eval(
                batch: &icicle_core::polynomial_eval::PolyEvalBatch<$field>,
                results: &mut (impl icicle_runtime::memory::HostOrDeviceSlice<$field> + ?Sized),
            ) -> Result<(), icicle_runtime::errors::eIcicleError> {
                unsafe {
                    $field_prefix_ident::polynomial_eval_ffi(
                        batch as *const icicle_core::polynomial_eval::PolyEvalBatch<$field>,
                        results.as_mut_ptr(),
                    )
                    .wrap()
                }
            }
        }
    };
}
