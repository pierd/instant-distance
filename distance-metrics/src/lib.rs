const DIMENSIONS: usize = 300;

#[repr(align(32))]
pub struct FloatArray(pub [f32; DIMENSIONS]);

pub fn distance(lhs: &FloatArray, rhs: &FloatArray) -> f32 {
    #[cfg(target_arch = "x86_64")]
    {
        use std::arch::x86_64::{
            _mm256_castps256_ps128, _mm256_extractf128_ps, _mm256_fmadd_ps, _mm256_load_ps,
            _mm256_setzero_ps, _mm256_sub_ps, _mm_add_ps, _mm_add_ss, _mm_cvtss_f32,
            _mm_fmadd_ps, _mm_load_ps, _mm_movehl_ps, _mm_shuffle_ps, _mm_sub_ps,
        };
        debug_assert_eq!(lhs.0.len() % 8, 4);

        unsafe {
            let mut acc_8x = _mm256_setzero_ps();
            for (lh_slice, rh_slice) in lhs.0.chunks_exact(8).zip(rhs.0.chunks_exact(8)) {
                let lh_8x = _mm256_load_ps(lh_slice.as_ptr());
                let rh_8x = _mm256_load_ps(rh_slice.as_ptr());
                let diff = _mm256_sub_ps(lh_8x, rh_8x);
                acc_8x = _mm256_fmadd_ps(diff, diff, acc_8x);
            }

            let mut acc_4x = _mm256_extractf128_ps(acc_8x, 1); // upper half
            let right = _mm256_castps256_ps128(acc_8x); // lower half
            acc_4x = _mm_add_ps(acc_4x, right); // sum halves

            let lh_4x = _mm_load_ps(lhs.0[DIMENSIONS - 4..].as_ptr());
            let rh_4x = _mm_load_ps(rhs.0[DIMENSIONS - 4..].as_ptr());
            let diff = _mm_sub_ps(lh_4x, rh_4x);
            acc_4x = _mm_fmadd_ps(diff, diff, acc_4x);

            let lower = _mm_movehl_ps(acc_4x, acc_4x);
            acc_4x = _mm_add_ps(acc_4x, lower);
            let upper = _mm_shuffle_ps(acc_4x, acc_4x, 0x1);
            acc_4x = _mm_add_ss(acc_4x, upper);
            _mm_cvtss_f32(acc_4x)
        }
    }
    #[cfg(not(target_arch = "x86_64"))]
    lhs.0
        .iter()
        .zip(rhs.0.iter())
        .map(|(&a, &b)| (a - b).powi(2))
        .sum::<f32>()
}

pub fn distance_no_simd(lhs: &FloatArray, rhs: &FloatArray) -> f32 {
    lhs.0
        .iter()
        .zip(rhs.0.iter())
        .map(|(&a, &b)| (a - b).powi(2))
        .sum::<f32>()
}

pub fn distance_slice(lhs: &[f32], rhs: &[f32]) -> f32 {
    lhs
        .iter()
        .zip(rhs.iter())
        .map(|(&a, &b)| (a - b).powi(2))
        .sum::<f32>()
}
