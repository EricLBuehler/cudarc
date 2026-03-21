#![cfg_attr(feature = "no-std", no_std)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#[cfg(feature = "no-std")]
extern crate alloc;
#[cfg(feature = "no-std")]
extern crate no_std_compat as std;
pub use self::cudaDataType_t as cutensorDataType_t;
pub type FILE = __sFILE;
pub type __darwin_off_t = __int64_t;
pub type __int64_t = ::core::ffi::c_longlong;
pub type cudaStream_t = *mut CUstream_st;
pub type cutensorBlockSparseTensorDescriptor_t = *mut cutensorBlockSparseTensorDescriptor;
pub type cutensorComputeDescriptor_t = *mut cutensorComputeDescriptor;
pub type cutensorHandle_t = *mut cutensorHandle;
pub type cutensorLoggerCallback_t = ::core::option::Option<
    unsafe extern "C" fn(
        logLevel: i32,
        functionName: *const ::core::ffi::c_char,
        message: *const ::core::ffi::c_char,
    ),
>;
pub type cutensorOperationDescriptor_t = *mut cutensorOperationDescriptor;
pub type cutensorPlanPreference_t = *mut cutensorPlanPreference;
pub type cutensorPlan_t = *mut cutensorPlan;
pub type cutensorTensorDescriptor_t = *mut cutensorTensorDescriptor;
pub type fpos_t = __darwin_off_t;
#[cfg(any(
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaDataType_t {
    CUDA_R_16F = 2,
    CUDA_C_16F = 6,
    CUDA_R_16BF = 14,
    CUDA_C_16BF = 15,
    CUDA_R_32F = 0,
    CUDA_C_32F = 4,
    CUDA_R_64F = 1,
    CUDA_C_64F = 5,
    CUDA_R_4I = 16,
    CUDA_C_4I = 17,
    CUDA_R_4U = 18,
    CUDA_C_4U = 19,
    CUDA_R_8I = 3,
    CUDA_C_8I = 7,
    CUDA_R_8U = 8,
    CUDA_C_8U = 9,
    CUDA_R_16I = 20,
    CUDA_C_16I = 21,
    CUDA_R_16U = 22,
    CUDA_C_16U = 23,
    CUDA_R_32I = 10,
    CUDA_C_32I = 11,
    CUDA_R_32U = 12,
    CUDA_C_32U = 13,
    CUDA_R_64I = 24,
    CUDA_C_64I = 25,
    CUDA_R_64U = 26,
    CUDA_C_64U = 27,
    CUDA_R_8F_E4M3 = 28,
    CUDA_R_8F_E5M2 = 29,
}
#[cfg(any(
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000",
    feature = "cuda-13010",
    feature = "cuda-13020"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaDataType_t {
    CUDA_R_16F = 2,
    CUDA_C_16F = 6,
    CUDA_R_16BF = 14,
    CUDA_C_16BF = 15,
    CUDA_R_32F = 0,
    CUDA_C_32F = 4,
    CUDA_R_64F = 1,
    CUDA_C_64F = 5,
    CUDA_R_4I = 16,
    CUDA_C_4I = 17,
    CUDA_R_4U = 18,
    CUDA_C_4U = 19,
    CUDA_R_8I = 3,
    CUDA_C_8I = 7,
    CUDA_R_8U = 8,
    CUDA_C_8U = 9,
    CUDA_R_16I = 20,
    CUDA_C_16I = 21,
    CUDA_R_16U = 22,
    CUDA_C_16U = 23,
    CUDA_R_32I = 10,
    CUDA_C_32I = 11,
    CUDA_R_32U = 12,
    CUDA_C_32U = 13,
    CUDA_R_64I = 24,
    CUDA_C_64I = 25,
    CUDA_R_64U = 26,
    CUDA_C_64U = 27,
    CUDA_R_8F_E4M3 = 28,
    CUDA_R_8F_E5M2 = 29,
    CUDA_R_8F_UE8M0 = 30,
    CUDA_R_6F_E2M3 = 31,
    CUDA_R_6F_E3M2 = 32,
    CUDA_R_4F_E2M1 = 33,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cutensorAlgo_t {
    CUTENSOR_ALGO_DEFAULT_PATIENT = -6,
    CUTENSOR_ALGO_GETT = -4,
    CUTENSOR_ALGO_TGETT = -3,
    CUTENSOR_ALGO_TTGT = -2,
    CUTENSOR_ALGO_DEFAULT = -1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cutensorAutotuneMode_t {
    CUTENSOR_AUTOTUNE_MODE_NONE = 0,
    CUTENSOR_AUTOTUNE_MODE_INCREMENTAL = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cutensorCacheMode_t {
    CUTENSOR_CACHE_MODE_NONE = 0,
    CUTENSOR_CACHE_MODE_PEDANTIC = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cutensorJitMode_t {
    CUTENSOR_JIT_MODE_NONE = 0,
    CUTENSOR_JIT_MODE_DEFAULT = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cutensorOperationDescriptorAttribute_t {
    CUTENSOR_OPERATION_DESCRIPTOR_TAG = 0,
    CUTENSOR_OPERATION_DESCRIPTOR_SCALAR_TYPE = 1,
    CUTENSOR_OPERATION_DESCRIPTOR_FLOPS = 2,
    CUTENSOR_OPERATION_DESCRIPTOR_MOVED_BYTES = 3,
    CUTENSOR_OPERATION_DESCRIPTOR_PADDING_LEFT = 4,
    CUTENSOR_OPERATION_DESCRIPTOR_PADDING_RIGHT = 5,
    CUTENSOR_OPERATION_DESCRIPTOR_PADDING_VALUE = 6,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cutensorOperator_t {
    CUTENSOR_OP_IDENTITY = 1,
    CUTENSOR_OP_SQRT = 2,
    CUTENSOR_OP_RELU = 8,
    CUTENSOR_OP_CONJ = 9,
    CUTENSOR_OP_RCP = 10,
    CUTENSOR_OP_SIGMOID = 11,
    CUTENSOR_OP_TANH = 12,
    CUTENSOR_OP_EXP = 22,
    CUTENSOR_OP_LOG = 23,
    CUTENSOR_OP_ABS = 24,
    CUTENSOR_OP_NEG = 25,
    CUTENSOR_OP_SIN = 26,
    CUTENSOR_OP_COS = 27,
    CUTENSOR_OP_TAN = 28,
    CUTENSOR_OP_SINH = 29,
    CUTENSOR_OP_COSH = 30,
    CUTENSOR_OP_ASIN = 31,
    CUTENSOR_OP_ACOS = 32,
    CUTENSOR_OP_ATAN = 33,
    CUTENSOR_OP_ASINH = 34,
    CUTENSOR_OP_ACOSH = 35,
    CUTENSOR_OP_ATANH = 36,
    CUTENSOR_OP_CEIL = 37,
    CUTENSOR_OP_FLOOR = 38,
    CUTENSOR_OP_MISH = 39,
    CUTENSOR_OP_SWISH = 40,
    CUTENSOR_OP_SOFT_PLUS = 41,
    CUTENSOR_OP_SOFT_SIGN = 42,
    CUTENSOR_OP_ADD = 3,
    CUTENSOR_OP_MUL = 5,
    CUTENSOR_OP_MAX = 6,
    CUTENSOR_OP_MIN = 7,
    CUTENSOR_OP_UNKNOWN = 126,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cutensorPlanAttribute_t {
    CUTENSOR_PLAN_REQUIRED_WORKSPACE = 0,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cutensorPlanPreferenceAttribute_t {
    CUTENSOR_PLAN_PREFERENCE_AUTOTUNE_MODE = 0,
    CUTENSOR_PLAN_PREFERENCE_CACHE_MODE = 1,
    CUTENSOR_PLAN_PREFERENCE_INCREMENTAL_COUNT = 2,
    CUTENSOR_PLAN_PREFERENCE_ALGO = 3,
    CUTENSOR_PLAN_PREFERENCE_KERNEL_RANK = 4,
    CUTENSOR_PLAN_PREFERENCE_JIT = 5,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cutensorStatus_t {
    CUTENSOR_STATUS_SUCCESS = 0,
    CUTENSOR_STATUS_NOT_INITIALIZED = 1,
    CUTENSOR_STATUS_ALLOC_FAILED = 3,
    CUTENSOR_STATUS_INVALID_VALUE = 7,
    CUTENSOR_STATUS_ARCH_MISMATCH = 8,
    CUTENSOR_STATUS_MAPPING_ERROR = 11,
    CUTENSOR_STATUS_EXECUTION_FAILED = 13,
    CUTENSOR_STATUS_INTERNAL_ERROR = 14,
    CUTENSOR_STATUS_NOT_SUPPORTED = 15,
    CUTENSOR_STATUS_LICENSE_ERROR = 16,
    CUTENSOR_STATUS_CUBLAS_ERROR = 17,
    CUTENSOR_STATUS_CUDA_ERROR = 18,
    CUTENSOR_STATUS_INSUFFICIENT_WORKSPACE = 19,
    CUTENSOR_STATUS_INSUFFICIENT_DRIVER = 20,
    CUTENSOR_STATUS_IO_ERROR = 21,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cutensorWorksizePreference_t {
    CUTENSOR_WORKSPACE_MIN = 1,
    CUTENSOR_WORKSPACE_DEFAULT = 2,
    CUTENSOR_WORKSPACE_MAX = 3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUstream_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct __sFILE {
    pub _p: *mut ::core::ffi::c_uchar,
    pub _r: ::core::ffi::c_int,
    pub _w: ::core::ffi::c_int,
    pub _flags: ::core::ffi::c_short,
    pub _file: ::core::ffi::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: ::core::ffi::c_int,
    pub _cookie: *mut ::core::ffi::c_void,
    pub _close: ::core::option::Option<
        unsafe extern "C" fn(arg1: *mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    >,
    pub _read: ::core::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::core::ffi::c_void,
            arg2: *mut ::core::ffi::c_char,
            __n: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub _seek: ::core::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::core::ffi::c_void,
            arg2: fpos_t,
            arg3: ::core::ffi::c_int,
        ) -> fpos_t,
    >,
    pub _write: ::core::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::core::ffi::c_void,
            arg2: *const ::core::ffi::c_char,
            __n: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: ::core::ffi::c_int,
    pub _ubuf: [::core::ffi::c_uchar; 3usize],
    pub _nbuf: [::core::ffi::c_uchar; 1usize],
    pub _lb: __sbuf,
    pub _blksize: ::core::ffi::c_int,
    pub _offset: fpos_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sFILEX {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct __sbuf {
    pub _base: *mut ::core::ffi::c_uchar,
    pub _size: ::core::ffi::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cutensorBlockSparseTensorDescriptor {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cutensorComputeDescriptor {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cutensorHandle {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cutensorOperationDescriptor {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cutensorPlan {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cutensorPlanPreference {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cutensorTensorDescriptor {
    _unused: [u8; 0],
}
#[cfg(any(
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000",
    feature = "cuda-13010",
    feature = "cuda-13020"
))]
impl cudaDataType_t {
    pub const CUDA_R_8F_UE4M3: cudaDataType_t = cudaDataType_t::CUDA_R_8F_E4M3;
}
#[cfg(not(feature = "dynamic-loading"))]
extern "C" {
    pub fn cutensorBlockSparseContract(
        handle: cutensorHandle_t,
        plan: cutensorPlan_t,
        alpha: *const ::core::ffi::c_void,
        A: *const *const ::core::ffi::c_void,
        B: *const *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        C: *const *const ::core::ffi::c_void,
        D: *const *mut ::core::ffi::c_void,
        workspace: *mut ::core::ffi::c_void,
        workspaceSize: u64,
        stream: cudaStream_t,
    ) -> cutensorStatus_t;
    pub fn cutensorContract(
        handle: cutensorHandle_t,
        plan: cutensorPlan_t,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        B: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        C: *const ::core::ffi::c_void,
        D: *mut ::core::ffi::c_void,
        workspace: *mut ::core::ffi::c_void,
        workspaceSize: u64,
        stream: cudaStream_t,
    ) -> cutensorStatus_t;
    pub fn cutensorContractTrinary(
        handle: cutensorHandle_t,
        plan: cutensorPlan_t,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        B: *const ::core::ffi::c_void,
        C: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        D: *const ::core::ffi::c_void,
        E: *mut ::core::ffi::c_void,
        workspace: *mut ::core::ffi::c_void,
        workspaceSize: u64,
        stream: cudaStream_t,
    ) -> cutensorStatus_t;
    pub fn cutensorCreate(handle: *mut cutensorHandle_t) -> cutensorStatus_t;
    pub fn cutensorCreateBlockSparseContraction(
        handle: cutensorHandle_t,
        desc: *mut cutensorOperationDescriptor_t,
        descA: cutensorBlockSparseTensorDescriptor_t,
        modeA: *const i32,
        opA: cutensorOperator_t,
        descB: cutensorBlockSparseTensorDescriptor_t,
        modeB: *const i32,
        opB: cutensorOperator_t,
        descC: cutensorBlockSparseTensorDescriptor_t,
        modeC: *const i32,
        opC: cutensorOperator_t,
        descD: cutensorBlockSparseTensorDescriptor_t,
        modeD: *const i32,
        descCompute: cutensorComputeDescriptor_t,
    ) -> cutensorStatus_t;
    pub fn cutensorCreateBlockSparseTensorDescriptor(
        handle: cutensorHandle_t,
        desc: *mut cutensorBlockSparseTensorDescriptor_t,
        numModes: u32,
        numNonZeroBlocks: u64,
        numSectionsPerMode: *const u32,
        extent: *const i64,
        nonZeroCoordinates: *const i32,
        stride: *const i64,
        dataType: cudaDataType_t,
    ) -> cutensorStatus_t;
    pub fn cutensorCreateContraction(
        handle: cutensorHandle_t,
        desc: *mut cutensorOperationDescriptor_t,
        descA: cutensorTensorDescriptor_t,
        modeA: *const i32,
        opA: cutensorOperator_t,
        descB: cutensorTensorDescriptor_t,
        modeB: *const i32,
        opB: cutensorOperator_t,
        descC: cutensorTensorDescriptor_t,
        modeC: *const i32,
        opC: cutensorOperator_t,
        descD: cutensorTensorDescriptor_t,
        modeD: *const i32,
        descCompute: cutensorComputeDescriptor_t,
    ) -> cutensorStatus_t;
    pub fn cutensorCreateContractionTrinary(
        handle: cutensorHandle_t,
        desc: *mut cutensorOperationDescriptor_t,
        descA: cutensorTensorDescriptor_t,
        modeA: *const i32,
        opA: cutensorOperator_t,
        descB: cutensorTensorDescriptor_t,
        modeB: *const i32,
        opB: cutensorOperator_t,
        descC: cutensorTensorDescriptor_t,
        modeC: *const i32,
        opC: cutensorOperator_t,
        descD: cutensorTensorDescriptor_t,
        modeD: *const i32,
        opD: cutensorOperator_t,
        descE: cutensorTensorDescriptor_t,
        modeE: *const i32,
        descCompute: cutensorComputeDescriptor_t,
    ) -> cutensorStatus_t;
    pub fn cutensorCreateElementwiseBinary(
        handle: cutensorHandle_t,
        desc: *mut cutensorOperationDescriptor_t,
        descA: cutensorTensorDescriptor_t,
        modeA: *const i32,
        opA: cutensorOperator_t,
        descC: cutensorTensorDescriptor_t,
        modeC: *const i32,
        opC: cutensorOperator_t,
        descD: cutensorTensorDescriptor_t,
        modeD: *const i32,
        opAC: cutensorOperator_t,
        descCompute: cutensorComputeDescriptor_t,
    ) -> cutensorStatus_t;
    pub fn cutensorCreateElementwiseTrinary(
        handle: cutensorHandle_t,
        desc: *mut cutensorOperationDescriptor_t,
        descA: cutensorTensorDescriptor_t,
        modeA: *const i32,
        opA: cutensorOperator_t,
        descB: cutensorTensorDescriptor_t,
        modeB: *const i32,
        opB: cutensorOperator_t,
        descC: cutensorTensorDescriptor_t,
        modeC: *const i32,
        opC: cutensorOperator_t,
        descD: cutensorTensorDescriptor_t,
        modeD: *const i32,
        opAB: cutensorOperator_t,
        opABC: cutensorOperator_t,
        descCompute: cutensorComputeDescriptor_t,
    ) -> cutensorStatus_t;
    pub fn cutensorCreatePermutation(
        handle: cutensorHandle_t,
        desc: *mut cutensorOperationDescriptor_t,
        descA: cutensorTensorDescriptor_t,
        modeA: *const i32,
        opA: cutensorOperator_t,
        descB: cutensorTensorDescriptor_t,
        modeB: *const i32,
        descCompute: cutensorComputeDescriptor_t,
    ) -> cutensorStatus_t;
    pub fn cutensorCreatePlan(
        handle: cutensorHandle_t,
        plan: *mut cutensorPlan_t,
        desc: cutensorOperationDescriptor_t,
        pref: cutensorPlanPreference_t,
        workspaceSizeLimit: u64,
    ) -> cutensorStatus_t;
    pub fn cutensorCreatePlanPreference(
        handle: cutensorHandle_t,
        pref: *mut cutensorPlanPreference_t,
        algo: cutensorAlgo_t,
        jitMode: cutensorJitMode_t,
    ) -> cutensorStatus_t;
    pub fn cutensorCreateReduction(
        handle: cutensorHandle_t,
        desc: *mut cutensorOperationDescriptor_t,
        descA: cutensorTensorDescriptor_t,
        modeA: *const i32,
        opA: cutensorOperator_t,
        descC: cutensorTensorDescriptor_t,
        modeC: *const i32,
        opC: cutensorOperator_t,
        descD: cutensorTensorDescriptor_t,
        modeD: *const i32,
        opReduce: cutensorOperator_t,
        descCompute: cutensorComputeDescriptor_t,
    ) -> cutensorStatus_t;
    pub fn cutensorCreateTensorDescriptor(
        handle: cutensorHandle_t,
        desc: *mut cutensorTensorDescriptor_t,
        numModes: u32,
        extent: *const i64,
        stride: *const i64,
        dataType: cudaDataType_t,
        alignmentRequirement: u32,
    ) -> cutensorStatus_t;
    pub fn cutensorDestroy(handle: cutensorHandle_t) -> cutensorStatus_t;
    pub fn cutensorDestroyBlockSparseTensorDescriptor(
        desc: cutensorBlockSparseTensorDescriptor_t,
    ) -> cutensorStatus_t;
    pub fn cutensorDestroyOperationDescriptor(
        desc: cutensorOperationDescriptor_t,
    ) -> cutensorStatus_t;
    pub fn cutensorDestroyPlan(plan: cutensorPlan_t) -> cutensorStatus_t;
    pub fn cutensorDestroyPlanPreference(pref: cutensorPlanPreference_t) -> cutensorStatus_t;
    pub fn cutensorDestroyTensorDescriptor(desc: cutensorTensorDescriptor_t) -> cutensorStatus_t;
    pub fn cutensorElementwiseBinaryExecute(
        handle: cutensorHandle_t,
        plan: cutensorPlan_t,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        gamma: *const ::core::ffi::c_void,
        C: *const ::core::ffi::c_void,
        D: *mut ::core::ffi::c_void,
        stream: cudaStream_t,
    ) -> cutensorStatus_t;
    pub fn cutensorElementwiseTrinaryExecute(
        handle: cutensorHandle_t,
        plan: cutensorPlan_t,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        B: *const ::core::ffi::c_void,
        gamma: *const ::core::ffi::c_void,
        C: *const ::core::ffi::c_void,
        D: *mut ::core::ffi::c_void,
        stream: cudaStream_t,
    ) -> cutensorStatus_t;
    pub fn cutensorEstimateWorkspaceSize(
        handle: cutensorHandle_t,
        desc: cutensorOperationDescriptor_t,
        planPref: cutensorPlanPreference_t,
        workspacePref: cutensorWorksizePreference_t,
        workspaceSizeEstimate: *mut u64,
    ) -> cutensorStatus_t;
    pub fn cutensorGetCudartVersion() -> usize;
    pub fn cutensorGetErrorString(error: cutensorStatus_t) -> *const ::core::ffi::c_char;
    pub fn cutensorGetVersion() -> usize;
    pub fn cutensorHandleReadPlanCacheFromFile(
        handle: cutensorHandle_t,
        filename: *const ::core::ffi::c_char,
        numCachelinesRead: *mut u32,
    ) -> cutensorStatus_t;
    pub fn cutensorHandleResizePlanCache(
        handle: cutensorHandle_t,
        numEntries: u32,
    ) -> cutensorStatus_t;
    pub fn cutensorHandleWritePlanCacheToFile(
        handle: cutensorHandle_t,
        filename: *const ::core::ffi::c_char,
    ) -> cutensorStatus_t;
    pub fn cutensorLoggerForceDisable() -> cutensorStatus_t;
    pub fn cutensorLoggerOpenFile(logFile: *const ::core::ffi::c_char) -> cutensorStatus_t;
    pub fn cutensorLoggerSetCallback(callback: cutensorLoggerCallback_t) -> cutensorStatus_t;
    pub fn cutensorLoggerSetFile(file: *mut FILE) -> cutensorStatus_t;
    pub fn cutensorLoggerSetLevel(level: i32) -> cutensorStatus_t;
    pub fn cutensorLoggerSetMask(mask: i32) -> cutensorStatus_t;
    pub fn cutensorOperationDescriptorGetAttribute(
        handle: cutensorHandle_t,
        desc: cutensorOperationDescriptor_t,
        attr: cutensorOperationDescriptorAttribute_t,
        buf: *mut ::core::ffi::c_void,
        sizeInBytes: usize,
    ) -> cutensorStatus_t;
    pub fn cutensorOperationDescriptorSetAttribute(
        handle: cutensorHandle_t,
        desc: cutensorOperationDescriptor_t,
        attr: cutensorOperationDescriptorAttribute_t,
        buf: *const ::core::ffi::c_void,
        sizeInBytes: usize,
    ) -> cutensorStatus_t;
    pub fn cutensorPermute(
        handle: cutensorHandle_t,
        plan: cutensorPlan_t,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        B: *mut ::core::ffi::c_void,
        stream: cudaStream_t,
    ) -> cutensorStatus_t;
    pub fn cutensorPlanGetAttribute(
        handle: cutensorHandle_t,
        plan: cutensorPlan_t,
        attr: cutensorPlanAttribute_t,
        buf: *mut ::core::ffi::c_void,
        sizeInBytes: usize,
    ) -> cutensorStatus_t;
    pub fn cutensorPlanPreferenceSetAttribute(
        handle: cutensorHandle_t,
        pref: cutensorPlanPreference_t,
        attr: cutensorPlanPreferenceAttribute_t,
        buf: *const ::core::ffi::c_void,
        sizeInBytes: usize,
    ) -> cutensorStatus_t;
    pub fn cutensorReadKernelCacheFromFile(
        handle: cutensorHandle_t,
        filename: *const ::core::ffi::c_char,
    ) -> cutensorStatus_t;
    pub fn cutensorReduce(
        handle: cutensorHandle_t,
        plan: cutensorPlan_t,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        C: *const ::core::ffi::c_void,
        D: *mut ::core::ffi::c_void,
        workspace: *mut ::core::ffi::c_void,
        workspaceSize: u64,
        stream: cudaStream_t,
    ) -> cutensorStatus_t;
    pub fn cutensorWriteKernelCacheToFile(
        handle: cutensorHandle_t,
        filename: *const ::core::ffi::c_char,
    ) -> cutensorStatus_t;
}
#[cfg(feature = "dynamic-loading")]
mod loaded {
    use super::*;
    pub unsafe fn cutensorBlockSparseContract(
        handle: cutensorHandle_t,
        plan: cutensorPlan_t,
        alpha: *const ::core::ffi::c_void,
        A: *const *const ::core::ffi::c_void,
        B: *const *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        C: *const *const ::core::ffi::c_void,
        D: *const *mut ::core::ffi::c_void,
        workspace: *mut ::core::ffi::c_void,
        workspaceSize: u64,
        stream: cudaStream_t,
    ) -> cutensorStatus_t {
        (culib().cutensorBlockSparseContract)(
            handle,
            plan,
            alpha,
            A,
            B,
            beta,
            C,
            D,
            workspace,
            workspaceSize,
            stream,
        )
    }
    pub unsafe fn cutensorContract(
        handle: cutensorHandle_t,
        plan: cutensorPlan_t,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        B: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        C: *const ::core::ffi::c_void,
        D: *mut ::core::ffi::c_void,
        workspace: *mut ::core::ffi::c_void,
        workspaceSize: u64,
        stream: cudaStream_t,
    ) -> cutensorStatus_t {
        (culib().cutensorContract)(
            handle,
            plan,
            alpha,
            A,
            B,
            beta,
            C,
            D,
            workspace,
            workspaceSize,
            stream,
        )
    }
    pub unsafe fn cutensorContractTrinary(
        handle: cutensorHandle_t,
        plan: cutensorPlan_t,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        B: *const ::core::ffi::c_void,
        C: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        D: *const ::core::ffi::c_void,
        E: *mut ::core::ffi::c_void,
        workspace: *mut ::core::ffi::c_void,
        workspaceSize: u64,
        stream: cudaStream_t,
    ) -> cutensorStatus_t {
        (culib().cutensorContractTrinary)(
            handle,
            plan,
            alpha,
            A,
            B,
            C,
            beta,
            D,
            E,
            workspace,
            workspaceSize,
            stream,
        )
    }
    pub unsafe fn cutensorCreate(handle: *mut cutensorHandle_t) -> cutensorStatus_t {
        (culib().cutensorCreate)(handle)
    }
    pub unsafe fn cutensorCreateBlockSparseContraction(
        handle: cutensorHandle_t,
        desc: *mut cutensorOperationDescriptor_t,
        descA: cutensorBlockSparseTensorDescriptor_t,
        modeA: *const i32,
        opA: cutensorOperator_t,
        descB: cutensorBlockSparseTensorDescriptor_t,
        modeB: *const i32,
        opB: cutensorOperator_t,
        descC: cutensorBlockSparseTensorDescriptor_t,
        modeC: *const i32,
        opC: cutensorOperator_t,
        descD: cutensorBlockSparseTensorDescriptor_t,
        modeD: *const i32,
        descCompute: cutensorComputeDescriptor_t,
    ) -> cutensorStatus_t {
        (culib().cutensorCreateBlockSparseContraction)(
            handle,
            desc,
            descA,
            modeA,
            opA,
            descB,
            modeB,
            opB,
            descC,
            modeC,
            opC,
            descD,
            modeD,
            descCompute,
        )
    }
    pub unsafe fn cutensorCreateBlockSparseTensorDescriptor(
        handle: cutensorHandle_t,
        desc: *mut cutensorBlockSparseTensorDescriptor_t,
        numModes: u32,
        numNonZeroBlocks: u64,
        numSectionsPerMode: *const u32,
        extent: *const i64,
        nonZeroCoordinates: *const i32,
        stride: *const i64,
        dataType: cudaDataType_t,
    ) -> cutensorStatus_t {
        (culib().cutensorCreateBlockSparseTensorDescriptor)(
            handle,
            desc,
            numModes,
            numNonZeroBlocks,
            numSectionsPerMode,
            extent,
            nonZeroCoordinates,
            stride,
            dataType,
        )
    }
    pub unsafe fn cutensorCreateContraction(
        handle: cutensorHandle_t,
        desc: *mut cutensorOperationDescriptor_t,
        descA: cutensorTensorDescriptor_t,
        modeA: *const i32,
        opA: cutensorOperator_t,
        descB: cutensorTensorDescriptor_t,
        modeB: *const i32,
        opB: cutensorOperator_t,
        descC: cutensorTensorDescriptor_t,
        modeC: *const i32,
        opC: cutensorOperator_t,
        descD: cutensorTensorDescriptor_t,
        modeD: *const i32,
        descCompute: cutensorComputeDescriptor_t,
    ) -> cutensorStatus_t {
        (culib().cutensorCreateContraction)(
            handle,
            desc,
            descA,
            modeA,
            opA,
            descB,
            modeB,
            opB,
            descC,
            modeC,
            opC,
            descD,
            modeD,
            descCompute,
        )
    }
    pub unsafe fn cutensorCreateContractionTrinary(
        handle: cutensorHandle_t,
        desc: *mut cutensorOperationDescriptor_t,
        descA: cutensorTensorDescriptor_t,
        modeA: *const i32,
        opA: cutensorOperator_t,
        descB: cutensorTensorDescriptor_t,
        modeB: *const i32,
        opB: cutensorOperator_t,
        descC: cutensorTensorDescriptor_t,
        modeC: *const i32,
        opC: cutensorOperator_t,
        descD: cutensorTensorDescriptor_t,
        modeD: *const i32,
        opD: cutensorOperator_t,
        descE: cutensorTensorDescriptor_t,
        modeE: *const i32,
        descCompute: cutensorComputeDescriptor_t,
    ) -> cutensorStatus_t {
        (culib().cutensorCreateContractionTrinary)(
            handle,
            desc,
            descA,
            modeA,
            opA,
            descB,
            modeB,
            opB,
            descC,
            modeC,
            opC,
            descD,
            modeD,
            opD,
            descE,
            modeE,
            descCompute,
        )
    }
    pub unsafe fn cutensorCreateElementwiseBinary(
        handle: cutensorHandle_t,
        desc: *mut cutensorOperationDescriptor_t,
        descA: cutensorTensorDescriptor_t,
        modeA: *const i32,
        opA: cutensorOperator_t,
        descC: cutensorTensorDescriptor_t,
        modeC: *const i32,
        opC: cutensorOperator_t,
        descD: cutensorTensorDescriptor_t,
        modeD: *const i32,
        opAC: cutensorOperator_t,
        descCompute: cutensorComputeDescriptor_t,
    ) -> cutensorStatus_t {
        (culib().cutensorCreateElementwiseBinary)(
            handle,
            desc,
            descA,
            modeA,
            opA,
            descC,
            modeC,
            opC,
            descD,
            modeD,
            opAC,
            descCompute,
        )
    }
    pub unsafe fn cutensorCreateElementwiseTrinary(
        handle: cutensorHandle_t,
        desc: *mut cutensorOperationDescriptor_t,
        descA: cutensorTensorDescriptor_t,
        modeA: *const i32,
        opA: cutensorOperator_t,
        descB: cutensorTensorDescriptor_t,
        modeB: *const i32,
        opB: cutensorOperator_t,
        descC: cutensorTensorDescriptor_t,
        modeC: *const i32,
        opC: cutensorOperator_t,
        descD: cutensorTensorDescriptor_t,
        modeD: *const i32,
        opAB: cutensorOperator_t,
        opABC: cutensorOperator_t,
        descCompute: cutensorComputeDescriptor_t,
    ) -> cutensorStatus_t {
        (culib().cutensorCreateElementwiseTrinary)(
            handle,
            desc,
            descA,
            modeA,
            opA,
            descB,
            modeB,
            opB,
            descC,
            modeC,
            opC,
            descD,
            modeD,
            opAB,
            opABC,
            descCompute,
        )
    }
    pub unsafe fn cutensorCreatePermutation(
        handle: cutensorHandle_t,
        desc: *mut cutensorOperationDescriptor_t,
        descA: cutensorTensorDescriptor_t,
        modeA: *const i32,
        opA: cutensorOperator_t,
        descB: cutensorTensorDescriptor_t,
        modeB: *const i32,
        descCompute: cutensorComputeDescriptor_t,
    ) -> cutensorStatus_t {
        (culib().cutensorCreatePermutation)(
            handle,
            desc,
            descA,
            modeA,
            opA,
            descB,
            modeB,
            descCompute,
        )
    }
    pub unsafe fn cutensorCreatePlan(
        handle: cutensorHandle_t,
        plan: *mut cutensorPlan_t,
        desc: cutensorOperationDescriptor_t,
        pref: cutensorPlanPreference_t,
        workspaceSizeLimit: u64,
    ) -> cutensorStatus_t {
        (culib().cutensorCreatePlan)(handle, plan, desc, pref, workspaceSizeLimit)
    }
    pub unsafe fn cutensorCreatePlanPreference(
        handle: cutensorHandle_t,
        pref: *mut cutensorPlanPreference_t,
        algo: cutensorAlgo_t,
        jitMode: cutensorJitMode_t,
    ) -> cutensorStatus_t {
        (culib().cutensorCreatePlanPreference)(handle, pref, algo, jitMode)
    }
    pub unsafe fn cutensorCreateReduction(
        handle: cutensorHandle_t,
        desc: *mut cutensorOperationDescriptor_t,
        descA: cutensorTensorDescriptor_t,
        modeA: *const i32,
        opA: cutensorOperator_t,
        descC: cutensorTensorDescriptor_t,
        modeC: *const i32,
        opC: cutensorOperator_t,
        descD: cutensorTensorDescriptor_t,
        modeD: *const i32,
        opReduce: cutensorOperator_t,
        descCompute: cutensorComputeDescriptor_t,
    ) -> cutensorStatus_t {
        (culib().cutensorCreateReduction)(
            handle,
            desc,
            descA,
            modeA,
            opA,
            descC,
            modeC,
            opC,
            descD,
            modeD,
            opReduce,
            descCompute,
        )
    }
    pub unsafe fn cutensorCreateTensorDescriptor(
        handle: cutensorHandle_t,
        desc: *mut cutensorTensorDescriptor_t,
        numModes: u32,
        extent: *const i64,
        stride: *const i64,
        dataType: cudaDataType_t,
        alignmentRequirement: u32,
    ) -> cutensorStatus_t {
        (culib().cutensorCreateTensorDescriptor)(
            handle,
            desc,
            numModes,
            extent,
            stride,
            dataType,
            alignmentRequirement,
        )
    }
    pub unsafe fn cutensorDestroy(handle: cutensorHandle_t) -> cutensorStatus_t {
        (culib().cutensorDestroy)(handle)
    }
    pub unsafe fn cutensorDestroyBlockSparseTensorDescriptor(
        desc: cutensorBlockSparseTensorDescriptor_t,
    ) -> cutensorStatus_t {
        (culib().cutensorDestroyBlockSparseTensorDescriptor)(desc)
    }
    pub unsafe fn cutensorDestroyOperationDescriptor(
        desc: cutensorOperationDescriptor_t,
    ) -> cutensorStatus_t {
        (culib().cutensorDestroyOperationDescriptor)(desc)
    }
    pub unsafe fn cutensorDestroyPlan(plan: cutensorPlan_t) -> cutensorStatus_t {
        (culib().cutensorDestroyPlan)(plan)
    }
    pub unsafe fn cutensorDestroyPlanPreference(
        pref: cutensorPlanPreference_t,
    ) -> cutensorStatus_t {
        (culib().cutensorDestroyPlanPreference)(pref)
    }
    pub unsafe fn cutensorDestroyTensorDescriptor(
        desc: cutensorTensorDescriptor_t,
    ) -> cutensorStatus_t {
        (culib().cutensorDestroyTensorDescriptor)(desc)
    }
    pub unsafe fn cutensorElementwiseBinaryExecute(
        handle: cutensorHandle_t,
        plan: cutensorPlan_t,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        gamma: *const ::core::ffi::c_void,
        C: *const ::core::ffi::c_void,
        D: *mut ::core::ffi::c_void,
        stream: cudaStream_t,
    ) -> cutensorStatus_t {
        (culib().cutensorElementwiseBinaryExecute)(handle, plan, alpha, A, gamma, C, D, stream)
    }
    pub unsafe fn cutensorElementwiseTrinaryExecute(
        handle: cutensorHandle_t,
        plan: cutensorPlan_t,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        B: *const ::core::ffi::c_void,
        gamma: *const ::core::ffi::c_void,
        C: *const ::core::ffi::c_void,
        D: *mut ::core::ffi::c_void,
        stream: cudaStream_t,
    ) -> cutensorStatus_t {
        (culib().cutensorElementwiseTrinaryExecute)(
            handle, plan, alpha, A, beta, B, gamma, C, D, stream,
        )
    }
    pub unsafe fn cutensorEstimateWorkspaceSize(
        handle: cutensorHandle_t,
        desc: cutensorOperationDescriptor_t,
        planPref: cutensorPlanPreference_t,
        workspacePref: cutensorWorksizePreference_t,
        workspaceSizeEstimate: *mut u64,
    ) -> cutensorStatus_t {
        (culib().cutensorEstimateWorkspaceSize)(
            handle,
            desc,
            planPref,
            workspacePref,
            workspaceSizeEstimate,
        )
    }
    pub unsafe fn cutensorGetCudartVersion() -> usize {
        (culib().cutensorGetCudartVersion)()
    }
    pub unsafe fn cutensorGetErrorString(error: cutensorStatus_t) -> *const ::core::ffi::c_char {
        (culib().cutensorGetErrorString)(error)
    }
    pub unsafe fn cutensorGetVersion() -> usize {
        (culib().cutensorGetVersion)()
    }
    pub unsafe fn cutensorHandleReadPlanCacheFromFile(
        handle: cutensorHandle_t,
        filename: *const ::core::ffi::c_char,
        numCachelinesRead: *mut u32,
    ) -> cutensorStatus_t {
        (culib().cutensorHandleReadPlanCacheFromFile)(handle, filename, numCachelinesRead)
    }
    pub unsafe fn cutensorHandleResizePlanCache(
        handle: cutensorHandle_t,
        numEntries: u32,
    ) -> cutensorStatus_t {
        (culib().cutensorHandleResizePlanCache)(handle, numEntries)
    }
    pub unsafe fn cutensorHandleWritePlanCacheToFile(
        handle: cutensorHandle_t,
        filename: *const ::core::ffi::c_char,
    ) -> cutensorStatus_t {
        (culib().cutensorHandleWritePlanCacheToFile)(handle, filename)
    }
    pub unsafe fn cutensorLoggerForceDisable() -> cutensorStatus_t {
        (culib().cutensorLoggerForceDisable)()
    }
    pub unsafe fn cutensorLoggerOpenFile(logFile: *const ::core::ffi::c_char) -> cutensorStatus_t {
        (culib().cutensorLoggerOpenFile)(logFile)
    }
    pub unsafe fn cutensorLoggerSetCallback(
        callback: cutensorLoggerCallback_t,
    ) -> cutensorStatus_t {
        (culib().cutensorLoggerSetCallback)(callback)
    }
    pub unsafe fn cutensorLoggerSetFile(file: *mut FILE) -> cutensorStatus_t {
        (culib().cutensorLoggerSetFile)(file)
    }
    pub unsafe fn cutensorLoggerSetLevel(level: i32) -> cutensorStatus_t {
        (culib().cutensorLoggerSetLevel)(level)
    }
    pub unsafe fn cutensorLoggerSetMask(mask: i32) -> cutensorStatus_t {
        (culib().cutensorLoggerSetMask)(mask)
    }
    pub unsafe fn cutensorOperationDescriptorGetAttribute(
        handle: cutensorHandle_t,
        desc: cutensorOperationDescriptor_t,
        attr: cutensorOperationDescriptorAttribute_t,
        buf: *mut ::core::ffi::c_void,
        sizeInBytes: usize,
    ) -> cutensorStatus_t {
        (culib().cutensorOperationDescriptorGetAttribute)(handle, desc, attr, buf, sizeInBytes)
    }
    pub unsafe fn cutensorOperationDescriptorSetAttribute(
        handle: cutensorHandle_t,
        desc: cutensorOperationDescriptor_t,
        attr: cutensorOperationDescriptorAttribute_t,
        buf: *const ::core::ffi::c_void,
        sizeInBytes: usize,
    ) -> cutensorStatus_t {
        (culib().cutensorOperationDescriptorSetAttribute)(handle, desc, attr, buf, sizeInBytes)
    }
    pub unsafe fn cutensorPermute(
        handle: cutensorHandle_t,
        plan: cutensorPlan_t,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        B: *mut ::core::ffi::c_void,
        stream: cudaStream_t,
    ) -> cutensorStatus_t {
        (culib().cutensorPermute)(handle, plan, alpha, A, B, stream)
    }
    pub unsafe fn cutensorPlanGetAttribute(
        handle: cutensorHandle_t,
        plan: cutensorPlan_t,
        attr: cutensorPlanAttribute_t,
        buf: *mut ::core::ffi::c_void,
        sizeInBytes: usize,
    ) -> cutensorStatus_t {
        (culib().cutensorPlanGetAttribute)(handle, plan, attr, buf, sizeInBytes)
    }
    pub unsafe fn cutensorPlanPreferenceSetAttribute(
        handle: cutensorHandle_t,
        pref: cutensorPlanPreference_t,
        attr: cutensorPlanPreferenceAttribute_t,
        buf: *const ::core::ffi::c_void,
        sizeInBytes: usize,
    ) -> cutensorStatus_t {
        (culib().cutensorPlanPreferenceSetAttribute)(handle, pref, attr, buf, sizeInBytes)
    }
    pub unsafe fn cutensorReadKernelCacheFromFile(
        handle: cutensorHandle_t,
        filename: *const ::core::ffi::c_char,
    ) -> cutensorStatus_t {
        (culib().cutensorReadKernelCacheFromFile)(handle, filename)
    }
    pub unsafe fn cutensorReduce(
        handle: cutensorHandle_t,
        plan: cutensorPlan_t,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        C: *const ::core::ffi::c_void,
        D: *mut ::core::ffi::c_void,
        workspace: *mut ::core::ffi::c_void,
        workspaceSize: u64,
        stream: cudaStream_t,
    ) -> cutensorStatus_t {
        (culib().cutensorReduce)(
            handle,
            plan,
            alpha,
            A,
            beta,
            C,
            D,
            workspace,
            workspaceSize,
            stream,
        )
    }
    pub unsafe fn cutensorWriteKernelCacheToFile(
        handle: cutensorHandle_t,
        filename: *const ::core::ffi::c_char,
    ) -> cutensorStatus_t {
        (culib().cutensorWriteKernelCacheToFile)(handle, filename)
    }
    pub struct Lib {
        __library: ::libloading::Library,
        pub cutensorBlockSparseContract: unsafe extern "C" fn(
            handle: cutensorHandle_t,
            plan: cutensorPlan_t,
            alpha: *const ::core::ffi::c_void,
            A: *const *const ::core::ffi::c_void,
            B: *const *const ::core::ffi::c_void,
            beta: *const ::core::ffi::c_void,
            C: *const *const ::core::ffi::c_void,
            D: *const *mut ::core::ffi::c_void,
            workspace: *mut ::core::ffi::c_void,
            workspaceSize: u64,
            stream: cudaStream_t,
        ) -> cutensorStatus_t,
        pub cutensorContract: unsafe extern "C" fn(
            handle: cutensorHandle_t,
            plan: cutensorPlan_t,
            alpha: *const ::core::ffi::c_void,
            A: *const ::core::ffi::c_void,
            B: *const ::core::ffi::c_void,
            beta: *const ::core::ffi::c_void,
            C: *const ::core::ffi::c_void,
            D: *mut ::core::ffi::c_void,
            workspace: *mut ::core::ffi::c_void,
            workspaceSize: u64,
            stream: cudaStream_t,
        ) -> cutensorStatus_t,
        pub cutensorContractTrinary: unsafe extern "C" fn(
            handle: cutensorHandle_t,
            plan: cutensorPlan_t,
            alpha: *const ::core::ffi::c_void,
            A: *const ::core::ffi::c_void,
            B: *const ::core::ffi::c_void,
            C: *const ::core::ffi::c_void,
            beta: *const ::core::ffi::c_void,
            D: *const ::core::ffi::c_void,
            E: *mut ::core::ffi::c_void,
            workspace: *mut ::core::ffi::c_void,
            workspaceSize: u64,
            stream: cudaStream_t,
        ) -> cutensorStatus_t,
        pub cutensorCreate: unsafe extern "C" fn(handle: *mut cutensorHandle_t) -> cutensorStatus_t,
        pub cutensorCreateBlockSparseContraction: unsafe extern "C" fn(
            handle: cutensorHandle_t,
            desc: *mut cutensorOperationDescriptor_t,
            descA: cutensorBlockSparseTensorDescriptor_t,
            modeA: *const i32,
            opA: cutensorOperator_t,
            descB: cutensorBlockSparseTensorDescriptor_t,
            modeB: *const i32,
            opB: cutensorOperator_t,
            descC: cutensorBlockSparseTensorDescriptor_t,
            modeC: *const i32,
            opC: cutensorOperator_t,
            descD: cutensorBlockSparseTensorDescriptor_t,
            modeD: *const i32,
            descCompute: cutensorComputeDescriptor_t,
        ) -> cutensorStatus_t,
        pub cutensorCreateBlockSparseTensorDescriptor: unsafe extern "C" fn(
            handle: cutensorHandle_t,
            desc: *mut cutensorBlockSparseTensorDescriptor_t,
            numModes: u32,
            numNonZeroBlocks: u64,
            numSectionsPerMode: *const u32,
            extent: *const i64,
            nonZeroCoordinates: *const i32,
            stride: *const i64,
            dataType: cudaDataType_t,
        )
            -> cutensorStatus_t,
        pub cutensorCreateContraction: unsafe extern "C" fn(
            handle: cutensorHandle_t,
            desc: *mut cutensorOperationDescriptor_t,
            descA: cutensorTensorDescriptor_t,
            modeA: *const i32,
            opA: cutensorOperator_t,
            descB: cutensorTensorDescriptor_t,
            modeB: *const i32,
            opB: cutensorOperator_t,
            descC: cutensorTensorDescriptor_t,
            modeC: *const i32,
            opC: cutensorOperator_t,
            descD: cutensorTensorDescriptor_t,
            modeD: *const i32,
            descCompute: cutensorComputeDescriptor_t,
        ) -> cutensorStatus_t,
        pub cutensorCreateContractionTrinary: unsafe extern "C" fn(
            handle: cutensorHandle_t,
            desc: *mut cutensorOperationDescriptor_t,
            descA: cutensorTensorDescriptor_t,
            modeA: *const i32,
            opA: cutensorOperator_t,
            descB: cutensorTensorDescriptor_t,
            modeB: *const i32,
            opB: cutensorOperator_t,
            descC: cutensorTensorDescriptor_t,
            modeC: *const i32,
            opC: cutensorOperator_t,
            descD: cutensorTensorDescriptor_t,
            modeD: *const i32,
            opD: cutensorOperator_t,
            descE: cutensorTensorDescriptor_t,
            modeE: *const i32,
            descCompute: cutensorComputeDescriptor_t,
        ) -> cutensorStatus_t,
        pub cutensorCreateElementwiseBinary: unsafe extern "C" fn(
            handle: cutensorHandle_t,
            desc: *mut cutensorOperationDescriptor_t,
            descA: cutensorTensorDescriptor_t,
            modeA: *const i32,
            opA: cutensorOperator_t,
            descC: cutensorTensorDescriptor_t,
            modeC: *const i32,
            opC: cutensorOperator_t,
            descD: cutensorTensorDescriptor_t,
            modeD: *const i32,
            opAC: cutensorOperator_t,
            descCompute: cutensorComputeDescriptor_t,
        ) -> cutensorStatus_t,
        pub cutensorCreateElementwiseTrinary: unsafe extern "C" fn(
            handle: cutensorHandle_t,
            desc: *mut cutensorOperationDescriptor_t,
            descA: cutensorTensorDescriptor_t,
            modeA: *const i32,
            opA: cutensorOperator_t,
            descB: cutensorTensorDescriptor_t,
            modeB: *const i32,
            opB: cutensorOperator_t,
            descC: cutensorTensorDescriptor_t,
            modeC: *const i32,
            opC: cutensorOperator_t,
            descD: cutensorTensorDescriptor_t,
            modeD: *const i32,
            opAB: cutensorOperator_t,
            opABC: cutensorOperator_t,
            descCompute: cutensorComputeDescriptor_t,
        ) -> cutensorStatus_t,
        pub cutensorCreatePermutation: unsafe extern "C" fn(
            handle: cutensorHandle_t,
            desc: *mut cutensorOperationDescriptor_t,
            descA: cutensorTensorDescriptor_t,
            modeA: *const i32,
            opA: cutensorOperator_t,
            descB: cutensorTensorDescriptor_t,
            modeB: *const i32,
            descCompute: cutensorComputeDescriptor_t,
        ) -> cutensorStatus_t,
        pub cutensorCreatePlan: unsafe extern "C" fn(
            handle: cutensorHandle_t,
            plan: *mut cutensorPlan_t,
            desc: cutensorOperationDescriptor_t,
            pref: cutensorPlanPreference_t,
            workspaceSizeLimit: u64,
        ) -> cutensorStatus_t,
        pub cutensorCreatePlanPreference: unsafe extern "C" fn(
            handle: cutensorHandle_t,
            pref: *mut cutensorPlanPreference_t,
            algo: cutensorAlgo_t,
            jitMode: cutensorJitMode_t,
        ) -> cutensorStatus_t,
        pub cutensorCreateReduction: unsafe extern "C" fn(
            handle: cutensorHandle_t,
            desc: *mut cutensorOperationDescriptor_t,
            descA: cutensorTensorDescriptor_t,
            modeA: *const i32,
            opA: cutensorOperator_t,
            descC: cutensorTensorDescriptor_t,
            modeC: *const i32,
            opC: cutensorOperator_t,
            descD: cutensorTensorDescriptor_t,
            modeD: *const i32,
            opReduce: cutensorOperator_t,
            descCompute: cutensorComputeDescriptor_t,
        ) -> cutensorStatus_t,
        pub cutensorCreateTensorDescriptor: unsafe extern "C" fn(
            handle: cutensorHandle_t,
            desc: *mut cutensorTensorDescriptor_t,
            numModes: u32,
            extent: *const i64,
            stride: *const i64,
            dataType: cudaDataType_t,
            alignmentRequirement: u32,
        ) -> cutensorStatus_t,
        pub cutensorDestroy: unsafe extern "C" fn(handle: cutensorHandle_t) -> cutensorStatus_t,
        pub cutensorDestroyBlockSparseTensorDescriptor:
            unsafe extern "C" fn(desc: cutensorBlockSparseTensorDescriptor_t) -> cutensorStatus_t,
        pub cutensorDestroyOperationDescriptor:
            unsafe extern "C" fn(desc: cutensorOperationDescriptor_t) -> cutensorStatus_t,
        pub cutensorDestroyPlan: unsafe extern "C" fn(plan: cutensorPlan_t) -> cutensorStatus_t,
        pub cutensorDestroyPlanPreference:
            unsafe extern "C" fn(pref: cutensorPlanPreference_t) -> cutensorStatus_t,
        pub cutensorDestroyTensorDescriptor:
            unsafe extern "C" fn(desc: cutensorTensorDescriptor_t) -> cutensorStatus_t,
        pub cutensorElementwiseBinaryExecute: unsafe extern "C" fn(
            handle: cutensorHandle_t,
            plan: cutensorPlan_t,
            alpha: *const ::core::ffi::c_void,
            A: *const ::core::ffi::c_void,
            gamma: *const ::core::ffi::c_void,
            C: *const ::core::ffi::c_void,
            D: *mut ::core::ffi::c_void,
            stream: cudaStream_t,
        ) -> cutensorStatus_t,
        pub cutensorElementwiseTrinaryExecute: unsafe extern "C" fn(
            handle: cutensorHandle_t,
            plan: cutensorPlan_t,
            alpha: *const ::core::ffi::c_void,
            A: *const ::core::ffi::c_void,
            beta: *const ::core::ffi::c_void,
            B: *const ::core::ffi::c_void,
            gamma: *const ::core::ffi::c_void,
            C: *const ::core::ffi::c_void,
            D: *mut ::core::ffi::c_void,
            stream: cudaStream_t,
        ) -> cutensorStatus_t,
        pub cutensorEstimateWorkspaceSize: unsafe extern "C" fn(
            handle: cutensorHandle_t,
            desc: cutensorOperationDescriptor_t,
            planPref: cutensorPlanPreference_t,
            workspacePref: cutensorWorksizePreference_t,
            workspaceSizeEstimate: *mut u64,
        ) -> cutensorStatus_t,
        pub cutensorGetCudartVersion: unsafe extern "C" fn() -> usize,
        pub cutensorGetErrorString:
            unsafe extern "C" fn(error: cutensorStatus_t) -> *const ::core::ffi::c_char,
        pub cutensorGetVersion: unsafe extern "C" fn() -> usize,
        pub cutensorHandleReadPlanCacheFromFile: unsafe extern "C" fn(
            handle: cutensorHandle_t,
            filename: *const ::core::ffi::c_char,
            numCachelinesRead: *mut u32,
        ) -> cutensorStatus_t,
        pub cutensorHandleResizePlanCache:
            unsafe extern "C" fn(handle: cutensorHandle_t, numEntries: u32) -> cutensorStatus_t,
        pub cutensorHandleWritePlanCacheToFile: unsafe extern "C" fn(
            handle: cutensorHandle_t,
            filename: *const ::core::ffi::c_char,
        ) -> cutensorStatus_t,
        pub cutensorLoggerForceDisable: unsafe extern "C" fn() -> cutensorStatus_t,
        pub cutensorLoggerOpenFile:
            unsafe extern "C" fn(logFile: *const ::core::ffi::c_char) -> cutensorStatus_t,
        pub cutensorLoggerSetCallback:
            unsafe extern "C" fn(callback: cutensorLoggerCallback_t) -> cutensorStatus_t,
        pub cutensorLoggerSetFile: unsafe extern "C" fn(file: *mut FILE) -> cutensorStatus_t,
        pub cutensorLoggerSetLevel: unsafe extern "C" fn(level: i32) -> cutensorStatus_t,
        pub cutensorLoggerSetMask: unsafe extern "C" fn(mask: i32) -> cutensorStatus_t,
        pub cutensorOperationDescriptorGetAttribute: unsafe extern "C" fn(
            handle: cutensorHandle_t,
            desc: cutensorOperationDescriptor_t,
            attr: cutensorOperationDescriptorAttribute_t,
            buf: *mut ::core::ffi::c_void,
            sizeInBytes: usize,
        )
            -> cutensorStatus_t,
        pub cutensorOperationDescriptorSetAttribute: unsafe extern "C" fn(
            handle: cutensorHandle_t,
            desc: cutensorOperationDescriptor_t,
            attr: cutensorOperationDescriptorAttribute_t,
            buf: *const ::core::ffi::c_void,
            sizeInBytes: usize,
        )
            -> cutensorStatus_t,
        pub cutensorPermute: unsafe extern "C" fn(
            handle: cutensorHandle_t,
            plan: cutensorPlan_t,
            alpha: *const ::core::ffi::c_void,
            A: *const ::core::ffi::c_void,
            B: *mut ::core::ffi::c_void,
            stream: cudaStream_t,
        ) -> cutensorStatus_t,
        pub cutensorPlanGetAttribute: unsafe extern "C" fn(
            handle: cutensorHandle_t,
            plan: cutensorPlan_t,
            attr: cutensorPlanAttribute_t,
            buf: *mut ::core::ffi::c_void,
            sizeInBytes: usize,
        ) -> cutensorStatus_t,
        pub cutensorPlanPreferenceSetAttribute: unsafe extern "C" fn(
            handle: cutensorHandle_t,
            pref: cutensorPlanPreference_t,
            attr: cutensorPlanPreferenceAttribute_t,
            buf: *const ::core::ffi::c_void,
            sizeInBytes: usize,
        ) -> cutensorStatus_t,
        pub cutensorReadKernelCacheFromFile: unsafe extern "C" fn(
            handle: cutensorHandle_t,
            filename: *const ::core::ffi::c_char,
        ) -> cutensorStatus_t,
        pub cutensorReduce: unsafe extern "C" fn(
            handle: cutensorHandle_t,
            plan: cutensorPlan_t,
            alpha: *const ::core::ffi::c_void,
            A: *const ::core::ffi::c_void,
            beta: *const ::core::ffi::c_void,
            C: *const ::core::ffi::c_void,
            D: *mut ::core::ffi::c_void,
            workspace: *mut ::core::ffi::c_void,
            workspaceSize: u64,
            stream: cudaStream_t,
        ) -> cutensorStatus_t,
        pub cutensorWriteKernelCacheToFile: unsafe extern "C" fn(
            handle: cutensorHandle_t,
            filename: *const ::core::ffi::c_char,
        ) -> cutensorStatus_t,
    }
    impl Lib {
        pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
        where
            P: AsRef<::std::ffi::OsStr>,
        {
            let library = ::libloading::Library::new(path.as_ref())?;
            Self::from_library(library)
        }
        pub unsafe fn from_library<L>(library: L) -> Result<Self, ::libloading::Error>
        where
            L: Into<::libloading::Library>,
        {
            let __library = library.into();
            let cutensorBlockSparseContract = __library
                .get(b"cutensorBlockSparseContract\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorContract = __library
                .get(b"cutensorContract\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorContractTrinary = __library
                .get(b"cutensorContractTrinary\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorCreate = __library
                .get(b"cutensorCreate\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorCreateBlockSparseContraction = __library
                .get(b"cutensorCreateBlockSparseContraction\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorCreateBlockSparseTensorDescriptor = __library
                .get(b"cutensorCreateBlockSparseTensorDescriptor\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorCreateContraction = __library
                .get(b"cutensorCreateContraction\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorCreateContractionTrinary = __library
                .get(b"cutensorCreateContractionTrinary\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorCreateElementwiseBinary = __library
                .get(b"cutensorCreateElementwiseBinary\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorCreateElementwiseTrinary = __library
                .get(b"cutensorCreateElementwiseTrinary\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorCreatePermutation = __library
                .get(b"cutensorCreatePermutation\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorCreatePlan = __library
                .get(b"cutensorCreatePlan\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorCreatePlanPreference = __library
                .get(b"cutensorCreatePlanPreference\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorCreateReduction = __library
                .get(b"cutensorCreateReduction\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorCreateTensorDescriptor = __library
                .get(b"cutensorCreateTensorDescriptor\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorDestroy = __library
                .get(b"cutensorDestroy\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorDestroyBlockSparseTensorDescriptor = __library
                .get(b"cutensorDestroyBlockSparseTensorDescriptor\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorDestroyOperationDescriptor = __library
                .get(b"cutensorDestroyOperationDescriptor\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorDestroyPlan = __library
                .get(b"cutensorDestroyPlan\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorDestroyPlanPreference = __library
                .get(b"cutensorDestroyPlanPreference\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorDestroyTensorDescriptor = __library
                .get(b"cutensorDestroyTensorDescriptor\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorElementwiseBinaryExecute = __library
                .get(b"cutensorElementwiseBinaryExecute\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorElementwiseTrinaryExecute = __library
                .get(b"cutensorElementwiseTrinaryExecute\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorEstimateWorkspaceSize = __library
                .get(b"cutensorEstimateWorkspaceSize\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorGetCudartVersion = __library
                .get(b"cutensorGetCudartVersion\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorGetErrorString = __library
                .get(b"cutensorGetErrorString\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorGetVersion = __library
                .get(b"cutensorGetVersion\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorHandleReadPlanCacheFromFile = __library
                .get(b"cutensorHandleReadPlanCacheFromFile\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorHandleResizePlanCache = __library
                .get(b"cutensorHandleResizePlanCache\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorHandleWritePlanCacheToFile = __library
                .get(b"cutensorHandleWritePlanCacheToFile\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorLoggerForceDisable = __library
                .get(b"cutensorLoggerForceDisable\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorLoggerOpenFile = __library
                .get(b"cutensorLoggerOpenFile\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorLoggerSetCallback = __library
                .get(b"cutensorLoggerSetCallback\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorLoggerSetFile = __library
                .get(b"cutensorLoggerSetFile\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorLoggerSetLevel = __library
                .get(b"cutensorLoggerSetLevel\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorLoggerSetMask = __library
                .get(b"cutensorLoggerSetMask\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorOperationDescriptorGetAttribute = __library
                .get(b"cutensorOperationDescriptorGetAttribute\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorOperationDescriptorSetAttribute = __library
                .get(b"cutensorOperationDescriptorSetAttribute\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorPermute = __library
                .get(b"cutensorPermute\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorPlanGetAttribute = __library
                .get(b"cutensorPlanGetAttribute\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorPlanPreferenceSetAttribute = __library
                .get(b"cutensorPlanPreferenceSetAttribute\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorReadKernelCacheFromFile = __library
                .get(b"cutensorReadKernelCacheFromFile\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorReduce = __library
                .get(b"cutensorReduce\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            let cutensorWriteKernelCacheToFile = __library
                .get(b"cutensorWriteKernelCacheToFile\0")
                .map(|sym| *sym)
                .expect("Expected symbol in library");
            Ok(Self {
                __library,
                cutensorBlockSparseContract,
                cutensorContract,
                cutensorContractTrinary,
                cutensorCreate,
                cutensorCreateBlockSparseContraction,
                cutensorCreateBlockSparseTensorDescriptor,
                cutensorCreateContraction,
                cutensorCreateContractionTrinary,
                cutensorCreateElementwiseBinary,
                cutensorCreateElementwiseTrinary,
                cutensorCreatePermutation,
                cutensorCreatePlan,
                cutensorCreatePlanPreference,
                cutensorCreateReduction,
                cutensorCreateTensorDescriptor,
                cutensorDestroy,
                cutensorDestroyBlockSparseTensorDescriptor,
                cutensorDestroyOperationDescriptor,
                cutensorDestroyPlan,
                cutensorDestroyPlanPreference,
                cutensorDestroyTensorDescriptor,
                cutensorElementwiseBinaryExecute,
                cutensorElementwiseTrinaryExecute,
                cutensorEstimateWorkspaceSize,
                cutensorGetCudartVersion,
                cutensorGetErrorString,
                cutensorGetVersion,
                cutensorHandleReadPlanCacheFromFile,
                cutensorHandleResizePlanCache,
                cutensorHandleWritePlanCacheToFile,
                cutensorLoggerForceDisable,
                cutensorLoggerOpenFile,
                cutensorLoggerSetCallback,
                cutensorLoggerSetFile,
                cutensorLoggerSetLevel,
                cutensorLoggerSetMask,
                cutensorOperationDescriptorGetAttribute,
                cutensorOperationDescriptorSetAttribute,
                cutensorPermute,
                cutensorPlanGetAttribute,
                cutensorPlanPreferenceSetAttribute,
                cutensorReadKernelCacheFromFile,
                cutensorReduce,
                cutensorWriteKernelCacheToFile,
            })
        }
    }
    pub unsafe fn is_culib_present() -> bool {
        let lib_names = ["cutensor"];
        let choices = lib_names
            .iter()
            .map(|l| crate::get_lib_name_candidates(l))
            .flatten();
        for choice in choices {
            if Lib::new(choice).is_ok() {
                return true;
            }
        }
        false
    }
    pub unsafe fn culib() -> &'static Lib {
        static LIB: std::sync::OnceLock<Lib> = std::sync::OnceLock::new();
        LIB.get_or_init(|| {
            let lib_names = std::vec!["cutensor"];
            let choices: std::vec::Vec<_> = lib_names
                .iter()
                .map(|l| crate::get_lib_name_candidates(l))
                .flatten()
                .collect();
            for choice in choices.iter() {
                if let Ok(lib) = Lib::new(choice) {
                    return lib;
                }
            }
            crate::panic_no_lib_found(lib_names[0], &choices);
        })
    }
}
#[cfg(feature = "dynamic-loading")]
pub use loaded::*;
