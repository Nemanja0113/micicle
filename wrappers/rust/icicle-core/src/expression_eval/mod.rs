use icicle_runtime::errors::eIcicleError;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ExpressionEvalConfig {
    pub stream: *const std::ffi::c_void,
    pub is_async: bool,
    pub is_data_on_device: bool,
}

impl Default for ExpressionEvalConfig {
    fn default() -> Self {
        Self {
            stream: std::ptr::null(),
            is_async: false,
            is_data_on_device: false,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ExpressionEvalData<T> {
    pub fixed_columns: *const *const T,
    pub advice_columns: *const *const T,
    pub instance_columns: *const *const T,
    pub constants: *const T,
    pub challenges: *const T,
    pub rotations: *const i32,
    pub calculations: *const u8,
    pub targets: *const u32,
    pub value_types: *const u8,
    pub value_indices: *const u32,
    pub domain_size: u32,
    pub column_len: u32,
    pub num_fixed: u32,
    pub num_advice: u32,
    pub num_instance: u32,
    pub num_constants: u32,
    pub num_challenges: u32,
    pub num_rotations: u32,
    pub num_calculations: u32,
    pub num_intermediates: u32,
    pub rot_scale: i32,
    pub isize: i32,
}

impl<T> ExpressionEvalData<T> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        fixed_columns: *const *const T,
        advice_columns: *const *const T,
        instance_columns: *const *const T,
        constants: *const T,
        challenges: *const T,
        rotations: *const i32,
        calculations: *const u8,
        targets: *const u32,
        value_types: *const u8,
        value_indices: *const u32,
        domain_size: u32,
        column_len: u32,
        num_fixed: u32,
        num_advice: u32,
        num_instance: u32,
        num_constants: u32,
        num_challenges: u32,
        num_rotations: u32,
        num_calculations: u32,
        num_intermediates: u32,
        rot_scale: i32,
        isize: i32,
    ) -> Self {
        Self {
            fixed_columns,
            advice_columns,
            instance_columns,
            constants,
            challenges,
            rotations,
            calculations,
            targets,
            value_types,
            value_indices,
            domain_size,
            column_len,
            num_fixed,
            num_advice,
            num_instance,
            num_constants,
            num_challenges,
            num_rotations,
            num_calculations,
            num_intermediates,
            rot_scale,
            isize,
        }
    }
}

use crate::traits::FieldImpl;

/// Trait for expression evaluation operations
#[doc(hidden)]
pub trait ExpressionEvalOps<F> {
    fn expression_evaluation(
        eval_data: &ExpressionEvalData<F>,
        cfg: &ExpressionEvalConfig,
        results: &mut (impl icicle_runtime::memory::HostOrDeviceSlice<F> + ?Sized),
    ) -> Result<(), eIcicleError>;
}

/// Standalone function that delegates to the trait implementation
pub fn expression_evaluation<F>(
    eval_data: &ExpressionEvalData<F>,
    cfg: &ExpressionEvalConfig,
    results: &mut (impl icicle_runtime::memory::HostOrDeviceSlice<F> + ?Sized),
) -> Result<(), eIcicleError>
where
    F: FieldImpl,
    <F as FieldImpl>::Config: ExpressionEvalOps<F>,
{
    <<F as FieldImpl>::Config as ExpressionEvalOps<F>>::expression_evaluation(
        eval_data, cfg, results,
    )
}

#[macro_export]
macro_rules! impl_expression_eval {
    (
        $field_prefix:literal,
        $field_prefix_ident:ident,
        $field:ident,
        $field_config:ident
    ) => {
        mod $field_prefix_ident {
            use icicle_core::expression_eval::{ExpressionEvalConfig, ExpressionEvalData, ExpressionEvalOps};
            use icicle_runtime::errors::eIcicleError;

            use super::{$field, $field_config};

            extern "C" {
                #[link_name = concat!($field_prefix, "_expression_evaluation")]
                fn expression_evaluation_ffi(
                    eval_data: *const ExpressionEvalData<$field>,
                    cfg: *const ExpressionEvalConfig,
                    results: *mut $field,
                ) -> eIcicleError;
            }

            impl ExpressionEvalOps<$field> for $field_config {
                fn expression_evaluation(
                    eval_data: &ExpressionEvalData<$field>,
                    cfg: &ExpressionEvalConfig,
                    results: &mut (impl icicle_runtime::memory::HostOrDeviceSlice<$field> + ?Sized),
                ) -> Result<(), eIcicleError> {
                    unsafe {
                        expression_evaluation_ffi(
                            eval_data as *const ExpressionEvalData<$field>,
                            cfg as *const ExpressionEvalConfig,
                            results.as_mut_ptr(),
                        )
                        .wrap()
                    }
                }
            }
        }
    };
}

