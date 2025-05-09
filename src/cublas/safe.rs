//! Safe abstractions around [crate::cublas::result] for doing gemm and gemv.
#![allow(clippy::too_many_arguments)]

use super::{result, result::CublasError, sys};
use crate::driver::{CudaStream, DevicePtr, DevicePtrMut};
use core::ffi::{c_int, c_longlong};
use std::sync::Arc;

/// Wrapper around [sys::cublasHandle_t]
///
/// 1. Create with [CudaBlas::new()]
/// 2. Execute gemm/gemv kernels with [Gemv] and [Gemm]. Both f32 and f64 are supported
///    for both
///
/// Note: This maintains a instance of [`Arc<CudaDevice>`], so will prevent the device
/// from being dropped.
#[derive(Debug)]
pub struct CudaBlas {
    pub(crate) handle: sys::cublasHandle_t,
    pub(crate) stream: Arc<CudaStream>,
}

unsafe impl Send for CudaBlas {}
unsafe impl Sync for CudaBlas {}

impl CudaBlas {
    /// Creates a new cublas handle and sets the stream to the `device`'s stream.
    pub fn new(stream: Arc<CudaStream>) -> Result<Self, CublasError> {
        let ctx = stream.context();
        ctx.record_err(ctx.bind_to_thread());
        let handle = result::create_handle()?;
        unsafe { result::set_stream(handle, stream.cu_stream() as _) }?;
        let blas = Self { handle, stream };
        Ok(blas)
    }

    /// Returns a reference to the underlying cublas handle.
    pub fn handle(&self) -> &sys::cublasHandle_t {
        &self.handle
    }

    /// Sets the handle's current to either the stream specified, or the device's default work
    /// stream.
    ///
    /// # Safety
    /// This is unsafe because you can end up scheduling multiple concurrent kernels that all
    /// write to the same memory address.
    pub unsafe fn set_stream(&mut self, stream: Arc<CudaStream>) -> Result<(), CublasError> {
        self.stream = stream;
        unsafe { result::set_stream(self.handle, self.stream.cu_stream() as _) }
    }

    /// Set the handle's pointer mode.
    /// ref: <https://docs.nvidia.com/cuda/cublas/#cublassetpointermode>
    ///
    /// Some cublas functions require the pointer mode to be set to `cublasPointerMode_t::CUBLAS_POINTER_MODE_DEVICE`
    /// when passing a device memory result buffer into the function, such as `cublas<t>asum()`.
    /// Otherwise the operation will panic with `SIGSEGV: invalid memory reference`,
    /// or one has to use a host memory reference, which has performance implications.
    pub fn set_pointer_mode(
        &self,
        pointer_mode: sys::cublasPointerMode_t,
    ) -> Result<(), CublasError> {
        unsafe {
            sys::cublasSetPointerMode_v2(self.handle, pointer_mode).result()?;
        }
        Ok(())
    }

    /// Get the handle's current pointer mode.
    /// ref: <https://docs.nvidia.com/cuda/cublas/#cublasgetpointermode>
    pub fn get_pointer_mode(&self) -> Result<sys::cublasPointerMode_t, CublasError> {
        unsafe {
            let mut mode = ::core::mem::MaybeUninit::uninit();
            sys::cublasGetPointerMode_v2(self.handle, mode.as_mut_ptr()).result()?;
            Ok(mode.assume_init())
        }
    }
}

impl Drop for CudaBlas {
    fn drop(&mut self) {
        let handle = std::mem::replace(&mut self.handle, std::ptr::null_mut());
        if !handle.is_null() {
            unsafe { result::destroy_handle(handle) }.unwrap();
        }
    }
}

/// Configuration for [Gemv]
#[derive(Debug, Copy, Clone)]
pub struct GemvConfig<T> {
    pub trans: sys::cublasOperation_t,
    pub m: c_int,
    pub n: c_int,
    pub alpha: T,
    pub lda: c_int,
    pub incx: c_int,
    pub beta: T,
    pub incy: c_int,
}

/// Matrix vector multiplication with elements of type `T`
pub trait Gemv<T> {
    /// Matrix vector multiplication.
    ///
    /// # Safety
    /// This is unsafe because improper arguments may lead to invalid
    /// memory accesses.
    unsafe fn gemv<A: DevicePtr<T>, X: DevicePtr<T>, Y: DevicePtrMut<T>>(
        &self,
        cfg: GemvConfig<T>,
        a: &A,
        x: &X,
        y: &mut Y,
    ) -> Result<(), CublasError>;
}

impl Gemv<f32> for CudaBlas {
    unsafe fn gemv<A: DevicePtr<f32>, X: DevicePtr<f32>, Y: DevicePtrMut<f32>>(
        &self,
        cfg: GemvConfig<f32>,
        a: &A,
        x: &X,
        y: &mut Y,
    ) -> Result<(), CublasError> {
        let (a, _record_a) = a.device_ptr(&self.stream);
        let (x, _record_x) = x.device_ptr(&self.stream);
        let (y, _record_y) = y.device_ptr_mut(&self.stream);
        result::sgemv(
            self.handle,
            cfg.trans,
            cfg.m,
            cfg.n,
            (&cfg.alpha) as *const _,
            a as *const _,
            cfg.lda,
            x as *const _,
            cfg.incx,
            (&cfg.beta) as *const _,
            y as *mut _,
            cfg.incy,
        )
    }
}

impl Gemv<f64> for CudaBlas {
    unsafe fn gemv<A: DevicePtr<f64>, X: DevicePtr<f64>, Y: DevicePtrMut<f64>>(
        &self,
        cfg: GemvConfig<f64>,
        a: &A,
        x: &X,
        y: &mut Y,
    ) -> Result<(), CublasError> {
        let (a, _record_a) = a.device_ptr(&self.stream);
        let (x, _record_x) = x.device_ptr(&self.stream);
        let (y, _record_y) = y.device_ptr_mut(&self.stream);
        result::dgemv(
            self.handle,
            cfg.trans,
            cfg.m,
            cfg.n,
            (&cfg.alpha) as *const _,
            a as *const _,
            cfg.lda,
            x as *const _,
            cfg.incx,
            (&cfg.beta) as *const _,
            y as *mut _,
            cfg.incy,
        )
    }
}

/// Configuration for [Gemm]
#[derive(Debug, Copy, Clone)]
pub struct GemmConfig<T> {
    pub transa: sys::cublasOperation_t,
    pub transb: sys::cublasOperation_t,
    pub m: c_int,
    pub n: c_int,
    pub k: c_int,
    pub alpha: T,
    pub lda: c_int,
    pub ldb: c_int,
    pub beta: T,
    pub ldc: c_int,
}

/// Configuration for [Gemm] strided batched call
#[derive(Debug, Copy, Clone)]
pub struct StridedBatchedConfig<T> {
    pub gemm: GemmConfig<T>,
    pub batch_size: c_int,
    pub stride_a: c_longlong,
    pub stride_b: c_longlong,
    pub stride_c: c_longlong,
}

/// Matrix matrix multiplication with elements of type `T`.
pub trait Gemm<T> {
    /// Matrix matrix multiplication. See
    /// [nvidia docs](https://docs.nvidia.com/cuda/cublas/index.html#cublas-t-gemm)
    ///
    /// # Safety
    /// This is unsafe because improper arguments may lead to invalid
    /// memory accesses.
    unsafe fn gemm<A: DevicePtr<T>, B: DevicePtr<T>, C: DevicePtrMut<T>>(
        &self,
        cfg: GemmConfig<T>,
        a: &A,
        b: &B,
        c: &mut C,
    ) -> Result<(), CublasError>;

    /// Batched matrix multiplication with stride support on batch dimension. See
    /// [nvidia docs](https://docs.nvidia.com/cuda/cublas/index.html#cublas-t-gemmstridedbatched)
    ///
    /// # Safety
    /// This is unsafe because improper arguments may lead to invalid
    /// memory accesses.
    unsafe fn gemm_strided_batched<A: DevicePtr<T>, B: DevicePtr<T>, C: DevicePtrMut<T>>(
        &self,
        cfg: StridedBatchedConfig<T>,
        a: &A,
        b: &B,
        c: &mut C,
    ) -> Result<(), CublasError>;
}

#[cfg(feature = "f16")]
impl Gemm<half::f16> for CudaBlas {
    unsafe fn gemm<A: DevicePtr<half::f16>, B: DevicePtr<half::f16>, C: DevicePtrMut<half::f16>>(
        &self,
        cfg: GemmConfig<half::f16>,
        a: &A,
        b: &B,
        c: &mut C,
    ) -> Result<(), CublasError> {
        let alpha: f32 = cfg.alpha.to_f32();
        let beta: f32 = cfg.beta.to_f32();
        let (a, _record_a) = a.device_ptr(&self.stream);
        let (b, _record_b) = b.device_ptr(&self.stream);
        let (c, _record_c) = c.device_ptr_mut(&self.stream);
        result::gemm_ex(
            self.handle,
            cfg.transa,
            cfg.transb,
            cfg.m,
            cfg.n,
            cfg.k,
            (&alpha) as *const f32 as *const _,
            a as *const _,
            sys::cudaDataType_t::CUDA_R_16F,
            cfg.lda,
            b as *const _,
            sys::cudaDataType_t::CUDA_R_16F,
            cfg.ldb,
            (&beta) as *const f32 as *const _,
            c as *mut _,
            sys::cudaDataType_t::CUDA_R_16F,
            cfg.ldc,
            sys::cublasComputeType_t::CUBLAS_COMPUTE_32F,
            sys::cublasGemmAlgo_t::CUBLAS_GEMM_DEFAULT,
        )
    }
    unsafe fn gemm_strided_batched<
        A: DevicePtr<half::f16>,
        B: DevicePtr<half::f16>,
        C: DevicePtrMut<half::f16>,
    >(
        &self,
        cfg: StridedBatchedConfig<half::f16>,
        a: &A,
        b: &B,
        c: &mut C,
    ) -> Result<(), CublasError> {
        let alpha: f32 = cfg.gemm.alpha.to_f32();
        let beta: f32 = cfg.gemm.beta.to_f32();
        let (a, _record_a) = a.device_ptr(&self.stream);
        let (b, _record_b) = b.device_ptr(&self.stream);
        let (c, _record_c) = c.device_ptr_mut(&self.stream);
        result::gemm_strided_batched_ex(
            self.handle,
            cfg.gemm.transa,
            cfg.gemm.transb,
            cfg.gemm.m,
            cfg.gemm.n,
            cfg.gemm.k,
            (&alpha) as *const f32 as *const _,
            a as *const _,
            sys::cudaDataType_t::CUDA_R_16F,
            cfg.gemm.lda,
            cfg.stride_a,
            b as *const _,
            sys::cudaDataType_t::CUDA_R_16F,
            cfg.gemm.ldb,
            cfg.stride_b,
            (&beta) as *const f32 as *const _,
            c as *mut _,
            sys::cudaDataType_t::CUDA_R_16F,
            cfg.gemm.ldc,
            cfg.stride_c,
            cfg.batch_size,
            sys::cublasComputeType_t::CUBLAS_COMPUTE_32F,
            sys::cublasGemmAlgo_t::CUBLAS_GEMM_DEFAULT,
        )
    }
}

#[cfg(feature = "f16")]
impl Gemm<half::bf16> for CudaBlas {
    unsafe fn gemm<
        A: DevicePtr<half::bf16>,
        B: DevicePtr<half::bf16>,
        C: DevicePtrMut<half::bf16>,
    >(
        &self,
        cfg: GemmConfig<half::bf16>,
        a: &A,
        b: &B,
        c: &mut C,
    ) -> Result<(), CublasError> {
        let alpha: f32 = cfg.alpha.to_f32();
        let beta: f32 = cfg.beta.to_f32();
        let (a, _record_a) = a.device_ptr(&self.stream);
        let (b, _record_b) = b.device_ptr(&self.stream);
        let (c, _record_c) = c.device_ptr_mut(&self.stream);
        result::gemm_ex(
            self.handle,
            cfg.transa,
            cfg.transb,
            cfg.m,
            cfg.n,
            cfg.k,
            (&alpha) as *const f32 as *const _,
            a as *const _,
            sys::cudaDataType_t::CUDA_R_16BF,
            cfg.lda,
            b as *const _,
            sys::cudaDataType_t::CUDA_R_16BF,
            cfg.ldb,
            (&beta) as *const f32 as *const _,
            c as *mut _,
            sys::cudaDataType_t::CUDA_R_16BF,
            cfg.ldc,
            sys::cublasComputeType_t::CUBLAS_COMPUTE_32F,
            sys::cublasGemmAlgo_t::CUBLAS_GEMM_DEFAULT,
        )
    }
    unsafe fn gemm_strided_batched<
        A: DevicePtr<half::bf16>,
        B: DevicePtr<half::bf16>,
        C: DevicePtrMut<half::bf16>,
    >(
        &self,
        cfg: StridedBatchedConfig<half::bf16>,
        a: &A,
        b: &B,
        c: &mut C,
    ) -> Result<(), CublasError> {
        let alpha: f32 = cfg.gemm.alpha.to_f32();
        let beta: f32 = cfg.gemm.beta.to_f32();
        let (a, _record_a) = a.device_ptr(&self.stream);
        let (b, _record_b) = b.device_ptr(&self.stream);
        let (c, _record_c) = c.device_ptr_mut(&self.stream);
        result::gemm_strided_batched_ex(
            self.handle,
            cfg.gemm.transa,
            cfg.gemm.transb,
            cfg.gemm.m,
            cfg.gemm.n,
            cfg.gemm.k,
            (&alpha) as *const f32 as *const _,
            a as *const _,
            sys::cudaDataType_t::CUDA_R_16BF,
            cfg.gemm.lda,
            cfg.stride_a,
            b as *const _,
            sys::cudaDataType_t::CUDA_R_16BF,
            cfg.gemm.ldb,
            cfg.stride_b,
            (&beta) as *const f32 as *const _,
            c as *mut _,
            sys::cudaDataType_t::CUDA_R_16BF,
            cfg.gemm.ldc,
            cfg.stride_c,
            cfg.batch_size,
            sys::cublasComputeType_t::CUBLAS_COMPUTE_32F,
            sys::cublasGemmAlgo_t::CUBLAS_GEMM_DEFAULT,
        )
    }
}

impl Gemm<f32> for CudaBlas {
    unsafe fn gemm<A: DevicePtr<f32>, B: DevicePtr<f32>, C: DevicePtrMut<f32>>(
        &self,
        cfg: GemmConfig<f32>,
        a: &A,
        b: &B,
        c: &mut C,
    ) -> Result<(), CublasError> {
        let (a, _record_a) = a.device_ptr(&self.stream);
        let (b, _record_b) = b.device_ptr(&self.stream);
        let (c, _record_c) = c.device_ptr_mut(&self.stream);
        result::sgemm(
            self.handle,
            cfg.transa,
            cfg.transb,
            cfg.m,
            cfg.n,
            cfg.k,
            (&cfg.alpha) as *const _,
            a as *const _,
            cfg.lda,
            b as *const _,
            cfg.ldb,
            (&cfg.beta) as *const _,
            c as *mut _,
            cfg.ldc,
        )
    }

    unsafe fn gemm_strided_batched<A: DevicePtr<f32>, B: DevicePtr<f32>, C: DevicePtrMut<f32>>(
        &self,
        cfg: StridedBatchedConfig<f32>,
        a: &A,
        b: &B,
        c: &mut C,
    ) -> Result<(), CublasError> {
        let (a, _record_a) = a.device_ptr(&self.stream);
        let (b, _record_b) = b.device_ptr(&self.stream);
        let (c, _record_c) = c.device_ptr_mut(&self.stream);
        result::sgemm_strided_batched(
            self.handle,
            cfg.gemm.transa,
            cfg.gemm.transb,
            cfg.gemm.m,
            cfg.gemm.n,
            cfg.gemm.k,
            (&cfg.gemm.alpha) as *const _,
            a as *const _,
            cfg.gemm.lda,
            cfg.stride_a,
            b as *const _,
            cfg.gemm.ldb,
            cfg.stride_b,
            (&cfg.gemm.beta) as *const _,
            c as *mut _,
            cfg.gemm.ldc,
            cfg.stride_c,
            cfg.batch_size,
        )
    }
}

impl Gemm<f64> for CudaBlas {
    unsafe fn gemm<A: DevicePtr<f64>, B: DevicePtr<f64>, C: DevicePtrMut<f64>>(
        &self,
        cfg: GemmConfig<f64>,
        a: &A,
        b: &B,
        c: &mut C,
    ) -> Result<(), CublasError> {
        let (a, _record_a) = a.device_ptr(&self.stream);
        let (b, _record_b) = b.device_ptr(&self.stream);
        let (c, _record_c) = c.device_ptr_mut(&self.stream);
        result::dgemm(
            self.handle,
            cfg.transa,
            cfg.transb,
            cfg.m,
            cfg.n,
            cfg.k,
            (&cfg.alpha) as *const _,
            a as *const _,
            cfg.lda,
            b as *const _,
            cfg.ldb,
            (&cfg.beta) as *const _,
            c as *mut _,
            cfg.ldc,
        )
    }

    unsafe fn gemm_strided_batched<A: DevicePtr<f64>, B: DevicePtr<f64>, C: DevicePtrMut<f64>>(
        &self,
        cfg: StridedBatchedConfig<f64>,
        a: &A,
        b: &B,
        c: &mut C,
    ) -> Result<(), CublasError> {
        let (a, _record_a) = a.device_ptr(&self.stream);
        let (b, _record_b) = b.device_ptr(&self.stream);
        let (c, _record_c) = c.device_ptr_mut(&self.stream);
        result::dgemm_strided_batched(
            self.handle,
            cfg.gemm.transa,
            cfg.gemm.transb,
            cfg.gemm.m,
            cfg.gemm.n,
            cfg.gemm.k,
            (&cfg.gemm.alpha) as *const _,
            a as *const _,
            cfg.gemm.lda,
            cfg.stride_a,
            b as *const _,
            cfg.gemm.ldb,
            cfg.stride_b,
            (&cfg.gemm.beta) as *const _,
            c as *mut _,
            cfg.gemm.ldc,
            cfg.stride_c,
            cfg.batch_size,
        )
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::needless_range_loop)]

    use crate::driver::CudaContext;

    use super::*;

    fn gemv_truth<T, const M: usize, const N: usize>(
        alpha: T,
        a: &[[T; N]; M],
        x: &[T; N],
        beta: T,
        y: &mut [T; M],
    ) where
        T: Copy + Clone + std::ops::AddAssign + std::ops::MulAssign + std::ops::Mul<T, Output = T>,
    {
        for m in 0..M {
            y[m] *= beta;
        }
        for m in 0..M {
            for n in 0..N {
                y[m] += alpha * a[m][n] * x[n];
            }
        }
    }

    fn gemm_truth<T, const M: usize, const N: usize, const K: usize>(
        alpha: T,
        a: &[[T; K]; M],
        b: &[[T; N]; K],
        beta: T,
        c: &mut [[T; N]; M],
    ) where
        T: Copy + Clone + std::ops::AddAssign + std::ops::MulAssign + std::ops::Mul<T, Output = T>,
    {
        for m in 0..M {
            for n in 0..N {
                c[m][n] *= beta;
            }
        }
        for m in 0..M {
            for n in 0..N {
                for k in 0..K {
                    c[m][n] += alpha * a[m][k] * b[k][n];
                }
            }
        }
    }

    #[test]
    fn test_sgemv() {
        let ctx = CudaContext::new(0).unwrap();
        let stream = ctx.default_stream();
        let blas = CudaBlas::new(stream.clone()).unwrap();
        const M: usize = 2;
        const N: usize = 5;
        let a: [[f32; N]; M] = [
            [0.9314776, 0.10300648, -0.620774, 1.5270752, 0.0259804],
            [0.16820757, -0.94463515, -1.3850101, 1.0600523, 1.5124008],
        ];
        #[rustfmt::skip]
        let b: [f32; N] = [-1.3441996, 1.3965541, -0.89106345, 0.21196432, -0.95535654];
        let mut c: [f32; M] = [1.0; M];
        gemv_truth(1.0, &a, &b, 0.0, &mut c);

        #[rustfmt::skip]
        let a_dev = stream.memcpy_stod(&[
            0.9314776, 0.10300648, -0.620774, 1.527075, 0.0259804,
            0.16820757, -0.94463515, -1.3850101, 1.0600523, 1.5124008,
        ]).unwrap();
        let b_dev = stream.memcpy_stod(&b).unwrap();
        let mut c_dev = stream.alloc_zeros(M).unwrap();
        unsafe {
            blas.gemv(
                GemvConfig {
                    trans: sys::cublasOperation_t::CUBLAS_OP_T,
                    m: N as i32,
                    n: M as i32,
                    alpha: 1.0,
                    lda: N as i32,
                    incx: 1,
                    beta: 0.0,
                    incy: 1,
                },
                &a_dev,
                &b_dev,
                &mut c_dev,
            )
        }
        .unwrap();

        let c_host = stream.memcpy_dtov(&c_dev).unwrap();
        for i in 0..M {
            assert!((c_host[i] - c[i]).abs() <= 1e-6);
        }
    }

    #[test]
    fn test_dgemv() {
        let ctx = CudaContext::new(0).unwrap();
        let stream = ctx.default_stream();
        let blas = CudaBlas::new(stream.clone()).unwrap();
        const M: usize = 8;
        const N: usize = 3;
        let a: [[f64; N]; M] = [
            [0.96151888, -0.36771390, 0.94069099],
            [2.20621538, -0.16479775, -1.78425562],
            [0.41080803, -0.56567699, -0.72781092],
            [-0.65718418, -0.14466463, 0.63984287],
            [0.20309605, 0.40480086, -1.57559848],
            [0.85628128, -0.51614553, -1.15904427],
            [-1.84258616, 0.24096519, -0.04563522],
            [-0.53364468, -1.07902217, 0.46823528],
        ];
        #[rustfmt::skip]
        let b: [f64; N] = [ 0.39745075, -1.06677043, -1.18272650];
        let mut c: [f64; M] = [1.0; M];
        gemv_truth(1.0, &a, &b, 0.0, &mut c);

        #[rustfmt::skip]
        let a_dev = stream.memcpy_stod(&[
            0.96151888, -0.36771390, 0.94069099,
            2.20621538, -0.16479775, -1.78425562,
            0.41080803, -0.56567699, -0.72781092,
            -0.65718418, -0.14466463, 0.63984287,
            0.20309605, 0.40480086, -1.57559848,
            0.85628128, -0.51614553, -1.15904427,
            -1.84258616, 0.24096519, -0.04563522,
            -0.53364468, -1.07902217, 0.46823528,
        ]).unwrap();
        let b_dev = stream.memcpy_stod(&b).unwrap();
        let mut c_dev = stream.alloc_zeros(M).unwrap();
        unsafe {
            blas.gemv(
                GemvConfig {
                    trans: sys::cublasOperation_t::CUBLAS_OP_T,
                    m: N as i32,
                    n: M as i32,
                    alpha: 1.0,
                    lda: N as i32,
                    incx: 1,
                    beta: 0.0,
                    incy: 1,
                },
                &a_dev,
                &b_dev,
                &mut c_dev,
            )
        }
        .unwrap();

        let c_host = stream.memcpy_dtov(&c_dev).unwrap();
        for i in 0..M {
            assert!((c_host[i] - c[i]).abs() <= 1e-8);
        }
    }

    #[cfg(feature = "f16")]
    #[test]
    fn test_hgemm() {
        let ctx = CudaContext::new(0).unwrap();
        let stream = ctx.default_stream();
        let blas = CudaBlas::new(stream.clone()).unwrap();
        const M: usize = 3;
        const K: usize = 4;
        const N: usize = 5;
        let a: [[half::f16; K]; M] = [
            [-0.5944882, 1.8055636, 0.52204555, -0.00397902],
            [-0.38346434, -0.38013917, 0.4198623, -0.22479166],
            [-1.6661372, -0.4568837, -0.9043474, 0.39125723],
        ]
        .map(|r| r.map(half::f16::from_f32));
        let b: [[half::f16; N]; K] = [
            [1.1292169, -0.13450263, 0.62789696, -0.5685516, 0.21946938],
            [1.0585804, -0.39789402, 0.90205914, 0.989318, -0.3443096],
            [1.3412506, 0.3059701, -0.9714474, -0.36113533, -1.6809629],
            [3.4746711, -1.0930681, 0.16502666, -0.59988785, 0.41375792],
        ]
        .map(|r| r.map(half::f16::from_f32));
        let mut c: [[half::f16; N]; M] = [[0.0; N]; M].map(|r| r.map(half::f16::from_f32));
        gemm_truth(
            half::f16::from_f32(1.0),
            &a,
            &b,
            half::f16::from_f32(0.0),
            &mut c,
        );

        #[rustfmt::skip]
        let a_dev = stream.memcpy_stod(&[
            -0.5944882, 1.8055636, 0.52204555, -0.00397902,
            -0.38346434, -0.38013917, 0.4198623, -0.22479166,
            -1.6661372, -0.4568837, -0.9043474, 0.39125723,
        ].map(half::f16::from_f32)).unwrap();
        #[rustfmt::skip]
        let b_dev = stream.memcpy_stod(&[
            1.1292169, -0.13450263, 0.62789696, -0.5685516, 0.21946938,
            1.0585804, -0.39789402, 0.90205914, 0.989318, -0.3443096,
            1.3412506, 0.3059701, -0.9714474, -0.36113533, -1.6809629,
            3.4746711, -1.0930681, 0.16502666, -0.59988785, 0.41375792,
        ].map(half::f16::from_f32)).unwrap();
        let mut c_dev = stream.alloc_zeros::<half::f16>(M * N).unwrap();
        unsafe {
            blas.gemm(
                GemmConfig {
                    transa: sys::cublasOperation_t::CUBLAS_OP_N,
                    transb: sys::cublasOperation_t::CUBLAS_OP_N,
                    m: N as i32,
                    n: M as i32,
                    k: K as i32,
                    alpha: half::f16::from_f32(1.0),
                    lda: N as i32,
                    ldb: K as i32,
                    beta: half::f16::from_f32(0.0),
                    ldc: N as i32,
                },
                &b_dev,
                &a_dev,
                &mut c_dev,
            )
        }
        .unwrap();

        let c_host = stream.memcpy_dtov(&c_dev).unwrap();
        for m in 0..M {
            for n in 0..N {
                let found = c_host[m * N + n];
                let expected = c[m][n];
                assert!(
                    (found - expected) <= half::f16::from_f32(1e-2),
                    "found={found:?}, expected={expected:?}"
                );
            }
        }

        #[rustfmt::skip]
        let a_dev = stream.memcpy_stod(&[
            -0.5944882, 1.8055636, 0.52204555, -0.00397902,
            -0.38346434, -0.38013917, 0.4198623, -0.22479166,
            -1.6661372, -0.4568837, -0.9043474, 0.39125723,
        ].map(half::bf16::from_f32)).unwrap();
        #[rustfmt::skip]
        let b_dev = stream.memcpy_stod(&[
            1.1292169, -0.13450263, 0.62789696, -0.5685516, 0.21946938,
            1.0585804, -0.39789402, 0.90205914, 0.989318, -0.3443096,
            1.3412506, 0.3059701, -0.9714474, -0.36113533, -1.6809629,
            3.4746711, -1.0930681, 0.16502666, -0.59988785, 0.41375792,
        ].map(half::bf16::from_f32)).unwrap();
        let mut c_dev = stream.alloc_zeros::<half::bf16>(M * N).unwrap();
        unsafe {
            blas.gemm(
                GemmConfig {
                    transa: sys::cublasOperation_t::CUBLAS_OP_N,
                    transb: sys::cublasOperation_t::CUBLAS_OP_N,
                    m: N as i32,
                    n: M as i32,
                    k: K as i32,
                    alpha: half::bf16::from_f32(1.0),
                    lda: N as i32,
                    ldb: K as i32,
                    beta: half::bf16::from_f32(0.0),
                    ldc: N as i32,
                },
                &b_dev,
                &a_dev,
                &mut c_dev,
            )
        }
        .unwrap();
        let c_host = stream.memcpy_dtov(&c_dev).unwrap();
        for m in 0..M {
            for n in 0..N {
                let found = c_host[m * N + n];
                let expected = c[m][n];
                assert!(
                    (half::bf16::to_f32(found) - half::f16::to_f32(expected)) <= 1e-2,
                    "found={found:?}, expected={expected:?}"
                );
            }
        }
    }

    #[test]
    fn test_sgemm() {
        let ctx = CudaContext::new(0).unwrap();
        let stream = ctx.default_stream();
        let blas = CudaBlas::new(stream.clone()).unwrap();
        const M: usize = 3;
        const K: usize = 4;
        const N: usize = 5;
        let a: [[f32; K]; M] = [
            [-0.5944882, 1.8055636, 0.52204555, -0.00397902],
            [-0.38346434, -0.38013917, 0.4198623, -0.22479166],
            [-1.6661372, -0.4568837, -0.9043474, 0.39125723],
        ];
        let b: [[f32; N]; K] = [
            [1.1292169, -0.13450263, 0.62789696, -0.5685516, 0.21946938],
            [1.0585804, -0.39789402, 0.90205914, 0.989318, -0.3443096],
            [1.3412506, 0.3059701, -0.9714474, -0.36113533, -1.6809629],
            [3.4746711, -1.0930681, 0.16502666, -0.59988785, 0.41375792],
        ];
        let mut c: [[f32; N]; M] = [[0.0; N]; M];
        gemm_truth(1.0, &a, &b, 0.0, &mut c);

        #[rustfmt::skip]
        let a_dev = stream.memcpy_stod(&[
            -0.5944882, 1.8055636, 0.52204555, -0.00397902,
            -0.38346434, -0.38013917, 0.4198623, -0.22479166,
            -1.6661372, -0.4568837, -0.9043474, 0.39125723,
        ]).unwrap();
        #[rustfmt::skip]
        let b_dev = stream.memcpy_stod(&[
            1.1292169, -0.13450263, 0.62789696, -0.5685516, 0.21946938,
            1.0585804, -0.39789402, 0.90205914, 0.989318, -0.3443096,
            1.3412506, 0.3059701, -0.9714474, -0.36113533, -1.6809629,
            3.4746711, -1.0930681, 0.16502666, -0.59988785, 0.41375792,
        ]).unwrap();
        let mut c_dev = stream.alloc_zeros::<f32>(M * N).unwrap();
        unsafe {
            blas.gemm(
                GemmConfig {
                    transa: sys::cublasOperation_t::CUBLAS_OP_N,
                    transb: sys::cublasOperation_t::CUBLAS_OP_N,
                    m: N as i32,
                    n: M as i32,
                    k: K as i32,
                    alpha: 1.0,
                    lda: N as i32,
                    ldb: K as i32,
                    beta: 0.0,
                    ldc: N as i32,
                },
                &b_dev,
                &a_dev,
                &mut c_dev,
            )
        }
        .unwrap();

        let c_host = stream.memcpy_dtov(&c_dev).unwrap();
        for m in 0..M {
            for n in 0..N {
                assert!((c_host[m * N + n] - c[m][n]) <= 1e-6);
            }
        }
    }

    #[test]
    fn test_dgemm() {
        let ctx = CudaContext::new(0).unwrap();
        let stream = ctx.default_stream();
        let blas = CudaBlas::new(stream.clone()).unwrap();
        const M: usize = 4;
        const K: usize = 3;
        const N: usize = 2;
        let a: [[f64; K]; M] = [
            [-0.70925030, -1.01357541, -0.64827034],
            [2.18493467, -0.61584842, -1.43844327],
            [-1.34792593, 0.68840750, -0.48057214],
            [1.22180992, 1.16245157, 0.01253436],
        ];
        let b: [[f64; N]; K] = [
            [-0.72735474, 1.35931170],
            [1.71798307, -0.13296247],
            [0.26855612, -1.95189980],
        ];
        let mut c: [[f64; N]; M] = [[0.0; N]; M];
        gemm_truth(1.0, &a, &b, 0.0, &mut c);

        #[rustfmt::skip]
        let a_dev = stream.memcpy_stod(&[
            -0.70925030, -1.01357541, -0.64827034,
            2.18493467, -0.61584842, -1.43844327,
            -1.34792593, 0.68840750, -0.48057214,
            1.22180992, 1.16245157, 0.01253436,
        ]).unwrap();
        #[rustfmt::skip]
        let b_dev = stream.memcpy_stod(&[
            -0.72735474, 1.35931170,
            1.71798307, -0.13296247,
            0.26855612, -1.95189980,
        ]).unwrap();
        let mut c_dev = stream.alloc_zeros::<f64>(M * N).unwrap();
        unsafe {
            blas.gemm(
                GemmConfig {
                    transa: sys::cublasOperation_t::CUBLAS_OP_N,
                    transb: sys::cublasOperation_t::CUBLAS_OP_N,
                    m: N as i32,
                    n: M as i32,
                    k: K as i32,
                    alpha: 1.0,
                    lda: N as i32,
                    ldb: K as i32,
                    beta: 0.0,
                    ldc: N as i32,
                },
                &b_dev,
                &a_dev,
                &mut c_dev,
            )
        }
        .unwrap();

        let c_host = stream.memcpy_dtov(&c_dev).unwrap();
        for m in 0..M {
            for n in 0..N {
                assert!((c_host[m * N + n] - c[m][n]) <= 1e-10);
            }
        }
    }

    #[test]
    fn cublas_pointer_mode() {
        let ctx = CudaContext::new(0).unwrap();
        let stream = ctx.default_stream();
        let blas = CudaBlas::new(stream.clone()).unwrap();
        assert_eq!(
            blas.get_pointer_mode().unwrap(),
            sys::cublasPointerMode_t::CUBLAS_POINTER_MODE_HOST,
            "The default pointer mode uses host pointers"
        );

        blas.set_pointer_mode(sys::cublasPointerMode_t::CUBLAS_POINTER_MODE_DEVICE)
            .unwrap();
        assert_eq!(
            blas.get_pointer_mode().unwrap(),
            sys::cublasPointerMode_t::CUBLAS_POINTER_MODE_DEVICE,
            "We have set the mode to use device pointers"
        );
    }
}
