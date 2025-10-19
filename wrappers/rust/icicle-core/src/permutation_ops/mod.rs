use crate::traits::FieldImpl;
use icicle_runtime::{
    errors::eIcicleError, memory::HostOrDeviceSlice, stream::IcicleStreamHandle,
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct PermutationConfig {
    pub stream_handle: IcicleStreamHandle,
    pub is_data_on_device: bool,
    pub is_async: bool,
}

impl PermutationConfig {
    pub fn default() -> Self {
        Self {
            stream_handle: std::ptr::null_mut(),
            is_data_on_device: false,
            is_async: false,
        }
    }
}

impl Default for PermutationConfig {
    fn default() -> Self {
        Self::default()
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct PermutationData<T> {
    pub permutation_products: *const *const T,
    pub fixed_cosets: *const T,
    pub advice_cosets: *const T,
    pub instance_cosets: *const T,
    pub perm_cosets: *const *const T,
    pub l0: *const T,
    pub l_last: *const T,
    pub l_active_row: *const T,
    pub column_types: *const i32,
    pub column_indices: *const i32,
    pub beta: *const T,
    pub gamma: *const T,
    pub y: *const T,
    pub delta_start: *const T,
    pub extended_omega: *const T,
    pub extended_len: u32,
    pub num_sets: u32,
    pub chunk_len: u32,
    pub num_fixed: u32,
    pub num_advice: u32,
    pub num_instance: u32,
    pub rot_scale: u32,
    pub isize: i32,
    pub blinding_factors: i32,
}

impl<T> PermutationData<T> {
    pub fn new(
        permutation_products: *const *const T,
        fixed_cosets: *const T,
        advice_cosets: *const T,
        instance_cosets: *const T,
        perm_cosets: *const *const T,
        l0: *const T,
        l_last: *const T,
        l_active_row: *const T,
        column_types: *const i32,
        column_indices: *const i32,
        beta: *const T,
        gamma: *const T,
        y: *const T,
        delta_start: *const T,
        extended_omega: *const T,
        extended_len: u32,
        num_sets: u32,
        chunk_len: u32,
        num_fixed: u32,
        num_advice: u32,
        num_instance: u32,
        rot_scale: u32,
        isize: i32,
        blinding_factors: i32,
    ) -> Self {
        Self {
            permutation_products,
            fixed_cosets,
            advice_cosets,
            instance_cosets,
            perm_cosets,
            l0,
            l_last,
            l_active_row,
            column_types,
            column_indices,
            beta,
            gamma,
            y,
            delta_start,
            extended_omega,
            extended_len,
            num_sets,
            chunk_len,
            num_fixed,
            num_advice,
            num_instance,
            rot_scale,
            isize,
            blinding_factors,
        }
    }
}

#[doc(hidden)]
pub trait PermutationOps<F: FieldImpl> {
    fn permutation_evaluation(
        perm_data: &PermutationData<F>,
        config: &PermutationConfig,
        results: &mut (impl HostOrDeviceSlice<F> + ?Sized),
    ) -> Result<(), eIcicleError>;
}

// Standalone function that delegates to the trait method
pub fn permutation_evaluation<F>(
    perm_data: &PermutationData<F>,
    config: &PermutationConfig,
    results: &mut (impl HostOrDeviceSlice<F> + ?Sized),
) -> Result<(), eIcicleError>
where
    F: FieldImpl,
    <F as FieldImpl>::Config: PermutationOps<F>,
{
    <<F as FieldImpl>::Config as PermutationOps<F>>::permutation_evaluation(
        perm_data,
        config,
        results,
    )
}

#[macro_export]
macro_rules! impl_permutation_ops {
    (
      $field_prefix:literal,
      $field_prefix_ident:ident,
      $field:ident,
      $field_config:ident
    ) => {
        mod $field_prefix_ident {
            use icicle_core::permutation_ops::{PermutationData, PermutationConfig};
            use icicle_runtime::{errors::eIcicleError, memory::HostOrDeviceSlice};

            extern "C" {
                #[link_name = concat!($field_prefix, "_permutation_evaluation")]
                pub(crate) fn permutation_evaluation_ffi(
                    perm_data: *const PermutationData<$field>,
                    config: *const PermutationConfig,
                    results: *mut $field,
                ) -> eIcicleError;
            }
        }

        impl icicle_core::permutation_ops::PermutationOps<$field> for $field_config {
            fn permutation_evaluation(
                perm_data: &icicle_core::permutation_ops::PermutationData<$field>,
                config: &icicle_core::permutation_ops::PermutationConfig,
                results: &mut (impl icicle_runtime::memory::HostOrDeviceSlice<$field> + ?Sized),
            ) -> Result<(), icicle_runtime::errors::eIcicleError> {
                unsafe {
                    $field_prefix_ident::permutation_evaluation_ffi(
                        perm_data as *const icicle_core::permutation_ops::PermutationData<$field>,
                        config as *const icicle_core::permutation_ops::PermutationConfig,
                        results.as_mut_ptr(),
                    )
                    .wrap()
                }
            }
        }
    };
}
