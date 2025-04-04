/* automatically generated by rust-bindgen 0.71.1 */

pub const CUDA_VERSION: u32 = 12030;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUstream_st {
    _unused: [u8; 0],
}
pub type cudaStream_t = *mut CUstream_st;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum libraryPropertyType_t {
    MAJOR_VERSION = 0,
    MINOR_VERSION = 1,
    PATCH_LEVEL = 2,
}
pub use self::libraryPropertyType_t as libraryPropertyType;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum curandStatus {
    CURAND_STATUS_SUCCESS = 0,
    CURAND_STATUS_VERSION_MISMATCH = 100,
    CURAND_STATUS_NOT_INITIALIZED = 101,
    CURAND_STATUS_ALLOCATION_FAILED = 102,
    CURAND_STATUS_TYPE_ERROR = 103,
    CURAND_STATUS_OUT_OF_RANGE = 104,
    CURAND_STATUS_LENGTH_NOT_MULTIPLE = 105,
    CURAND_STATUS_DOUBLE_PRECISION_REQUIRED = 106,
    CURAND_STATUS_LAUNCH_FAILURE = 201,
    CURAND_STATUS_PREEXISTING_FAILURE = 202,
    CURAND_STATUS_INITIALIZATION_FAILED = 203,
    CURAND_STATUS_ARCH_MISMATCH = 204,
    CURAND_STATUS_INTERNAL_ERROR = 999,
}
pub use self::curandStatus as curandStatus_t;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum curandRngType {
    CURAND_RNG_TEST = 0,
    CURAND_RNG_PSEUDO_DEFAULT = 100,
    CURAND_RNG_PSEUDO_XORWOW = 101,
    CURAND_RNG_PSEUDO_MRG32K3A = 121,
    CURAND_RNG_PSEUDO_MTGP32 = 141,
    CURAND_RNG_PSEUDO_MT19937 = 142,
    CURAND_RNG_PSEUDO_PHILOX4_32_10 = 161,
    CURAND_RNG_QUASI_DEFAULT = 200,
    CURAND_RNG_QUASI_SOBOL32 = 201,
    CURAND_RNG_QUASI_SCRAMBLED_SOBOL32 = 202,
    CURAND_RNG_QUASI_SOBOL64 = 203,
    CURAND_RNG_QUASI_SCRAMBLED_SOBOL64 = 204,
}
pub use self::curandRngType as curandRngType_t;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum curandOrdering {
    CURAND_ORDERING_PSEUDO_BEST = 100,
    CURAND_ORDERING_PSEUDO_DEFAULT = 101,
    CURAND_ORDERING_PSEUDO_SEEDED = 102,
    CURAND_ORDERING_PSEUDO_LEGACY = 103,
    CURAND_ORDERING_PSEUDO_DYNAMIC = 104,
    CURAND_ORDERING_QUASI_DEFAULT = 201,
}
pub use self::curandOrdering as curandOrdering_t;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum curandDirectionVectorSet {
    CURAND_DIRECTION_VECTORS_32_JOEKUO6 = 101,
    CURAND_SCRAMBLED_DIRECTION_VECTORS_32_JOEKUO6 = 102,
    CURAND_DIRECTION_VECTORS_64_JOEKUO6 = 103,
    CURAND_SCRAMBLED_DIRECTION_VECTORS_64_JOEKUO6 = 104,
}
pub use self::curandDirectionVectorSet as curandDirectionVectorSet_t;
pub type curandDirectionVectors32_t = [::core::ffi::c_uint; 32usize];
pub type curandDirectionVectors64_t = [::core::ffi::c_ulonglong; 64usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct curandGenerator_st {
    _unused: [u8; 0],
}
pub type curandGenerator_t = *mut curandGenerator_st;
pub type curandDistribution_st = f64;
pub type curandDistribution_t = *mut curandDistribution_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct curandDistributionShift_st {
    _unused: [u8; 0],
}
pub type curandDistributionShift_t = *mut curandDistributionShift_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct curandDistributionM2Shift_st {
    _unused: [u8; 0],
}
pub type curandDistributionM2Shift_t = *mut curandDistributionM2Shift_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct curandHistogramM2_st {
    _unused: [u8; 0],
}
pub type curandHistogramM2_t = *mut curandHistogramM2_st;
pub type curandHistogramM2K_st = ::core::ffi::c_uint;
pub type curandHistogramM2K_t = *mut curandHistogramM2K_st;
pub type curandHistogramM2V_st = curandDistribution_st;
pub type curandHistogramM2V_t = *mut curandHistogramM2V_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct curandDiscreteDistribution_st {
    _unused: [u8; 0],
}
pub type curandDiscreteDistribution_t = *mut curandDiscreteDistribution_st;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum curandMethod {
    CURAND_CHOOSE_BEST = 0,
    CURAND_ITR = 1,
    CURAND_KNUTH = 2,
    CURAND_HITR = 3,
    CURAND_M1 = 4,
    CURAND_M2 = 5,
    CURAND_BINARY_SEARCH = 6,
    CURAND_DISCRETE_GAUSS = 7,
    CURAND_REJECTION = 8,
    CURAND_DEVICE_API = 9,
    CURAND_FAST_REJECTION = 10,
    CURAND_3RD = 11,
    CURAND_DEFINITION = 12,
    CURAND_POISSON = 13,
}
pub use self::curandMethod as curandMethod_t;
pub struct Lib {
    __library: ::libloading::Library,
    pub curandCreateGenerator: Result<
        unsafe extern "C" fn(
            generator: *mut curandGenerator_t,
            rng_type: curandRngType_t,
        ) -> curandStatus_t,
        ::libloading::Error,
    >,
    pub curandCreateGeneratorHost: Result<
        unsafe extern "C" fn(
            generator: *mut curandGenerator_t,
            rng_type: curandRngType_t,
        ) -> curandStatus_t,
        ::libloading::Error,
    >,
    pub curandDestroyGenerator: Result<
        unsafe extern "C" fn(generator: curandGenerator_t) -> curandStatus_t,
        ::libloading::Error,
    >,
    pub curandGetVersion: Result<
        unsafe extern "C" fn(version: *mut ::core::ffi::c_int) -> curandStatus_t,
        ::libloading::Error,
    >,
    pub curandGetProperty: Result<
        unsafe extern "C" fn(
            type_: libraryPropertyType,
            value: *mut ::core::ffi::c_int,
        ) -> curandStatus_t,
        ::libloading::Error,
    >,
    pub curandSetStream: Result<
        unsafe extern "C" fn(generator: curandGenerator_t, stream: cudaStream_t) -> curandStatus_t,
        ::libloading::Error,
    >,
    pub curandSetPseudoRandomGeneratorSeed: Result<
        unsafe extern "C" fn(
            generator: curandGenerator_t,
            seed: ::core::ffi::c_ulonglong,
        ) -> curandStatus_t,
        ::libloading::Error,
    >,
    pub curandSetGeneratorOffset: Result<
        unsafe extern "C" fn(
            generator: curandGenerator_t,
            offset: ::core::ffi::c_ulonglong,
        ) -> curandStatus_t,
        ::libloading::Error,
    >,
    pub curandSetGeneratorOrdering: Result<
        unsafe extern "C" fn(
            generator: curandGenerator_t,
            order: curandOrdering_t,
        ) -> curandStatus_t,
        ::libloading::Error,
    >,
    pub curandSetQuasiRandomGeneratorDimensions: Result<
        unsafe extern "C" fn(
            generator: curandGenerator_t,
            num_dimensions: ::core::ffi::c_uint,
        ) -> curandStatus_t,
        ::libloading::Error,
    >,
    pub curandGenerate: Result<
        unsafe extern "C" fn(
            generator: curandGenerator_t,
            outputPtr: *mut ::core::ffi::c_uint,
            num: usize,
        ) -> curandStatus_t,
        ::libloading::Error,
    >,
    pub curandGenerateLongLong: Result<
        unsafe extern "C" fn(
            generator: curandGenerator_t,
            outputPtr: *mut ::core::ffi::c_ulonglong,
            num: usize,
        ) -> curandStatus_t,
        ::libloading::Error,
    >,
    pub curandGenerateUniform: Result<
        unsafe extern "C" fn(
            generator: curandGenerator_t,
            outputPtr: *mut f32,
            num: usize,
        ) -> curandStatus_t,
        ::libloading::Error,
    >,
    pub curandGenerateUniformDouble: Result<
        unsafe extern "C" fn(
            generator: curandGenerator_t,
            outputPtr: *mut f64,
            num: usize,
        ) -> curandStatus_t,
        ::libloading::Error,
    >,
    pub curandGenerateNormal: Result<
        unsafe extern "C" fn(
            generator: curandGenerator_t,
            outputPtr: *mut f32,
            n: usize,
            mean: f32,
            stddev: f32,
        ) -> curandStatus_t,
        ::libloading::Error,
    >,
    pub curandGenerateNormalDouble: Result<
        unsafe extern "C" fn(
            generator: curandGenerator_t,
            outputPtr: *mut f64,
            n: usize,
            mean: f64,
            stddev: f64,
        ) -> curandStatus_t,
        ::libloading::Error,
    >,
    pub curandGenerateLogNormal: Result<
        unsafe extern "C" fn(
            generator: curandGenerator_t,
            outputPtr: *mut f32,
            n: usize,
            mean: f32,
            stddev: f32,
        ) -> curandStatus_t,
        ::libloading::Error,
    >,
    pub curandGenerateLogNormalDouble: Result<
        unsafe extern "C" fn(
            generator: curandGenerator_t,
            outputPtr: *mut f64,
            n: usize,
            mean: f64,
            stddev: f64,
        ) -> curandStatus_t,
        ::libloading::Error,
    >,
    pub curandCreatePoissonDistribution: Result<
        unsafe extern "C" fn(
            lambda: f64,
            discrete_distribution: *mut curandDiscreteDistribution_t,
        ) -> curandStatus_t,
        ::libloading::Error,
    >,
    pub curandDestroyDistribution: Result<
        unsafe extern "C" fn(discrete_distribution: curandDiscreteDistribution_t) -> curandStatus_t,
        ::libloading::Error,
    >,
    pub curandGeneratePoisson: Result<
        unsafe extern "C" fn(
            generator: curandGenerator_t,
            outputPtr: *mut ::core::ffi::c_uint,
            n: usize,
            lambda: f64,
        ) -> curandStatus_t,
        ::libloading::Error,
    >,
    pub curandGeneratePoissonMethod: Result<
        unsafe extern "C" fn(
            generator: curandGenerator_t,
            outputPtr: *mut ::core::ffi::c_uint,
            n: usize,
            lambda: f64,
            method: curandMethod_t,
        ) -> curandStatus_t,
        ::libloading::Error,
    >,
    pub curandGenerateBinomial: Result<
        unsafe extern "C" fn(
            generator: curandGenerator_t,
            outputPtr: *mut ::core::ffi::c_uint,
            num: usize,
            n: ::core::ffi::c_uint,
            p: f64,
        ) -> curandStatus_t,
        ::libloading::Error,
    >,
    pub curandGenerateBinomialMethod: Result<
        unsafe extern "C" fn(
            generator: curandGenerator_t,
            outputPtr: *mut ::core::ffi::c_uint,
            num: usize,
            n: ::core::ffi::c_uint,
            p: f64,
            method: curandMethod_t,
        ) -> curandStatus_t,
        ::libloading::Error,
    >,
    pub curandGenerateSeeds: Result<
        unsafe extern "C" fn(generator: curandGenerator_t) -> curandStatus_t,
        ::libloading::Error,
    >,
    pub curandGetDirectionVectors32: Result<
        unsafe extern "C" fn(
            vectors: *mut *mut curandDirectionVectors32_t,
            set: curandDirectionVectorSet_t,
        ) -> curandStatus_t,
        ::libloading::Error,
    >,
    pub curandGetScrambleConstants32: Result<
        unsafe extern "C" fn(constants: *mut *mut ::core::ffi::c_uint) -> curandStatus_t,
        ::libloading::Error,
    >,
    pub curandGetDirectionVectors64: Result<
        unsafe extern "C" fn(
            vectors: *mut *mut curandDirectionVectors64_t,
            set: curandDirectionVectorSet_t,
        ) -> curandStatus_t,
        ::libloading::Error,
    >,
    pub curandGetScrambleConstants64: Result<
        unsafe extern "C" fn(constants: *mut *mut ::core::ffi::c_ulonglong) -> curandStatus_t,
        ::libloading::Error,
    >,
}
impl Lib {
    pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let library = ::libloading::Library::new(path)?;
        Self::from_library(library)
    }
    pub unsafe fn from_library<L>(library: L) -> Result<Self, ::libloading::Error>
    where
        L: Into<::libloading::Library>,
    {
        let __library = library.into();
        let curandCreateGenerator = __library.get(b"curandCreateGenerator\0").map(|sym| *sym);
        let curandCreateGeneratorHost = __library
            .get(b"curandCreateGeneratorHost\0")
            .map(|sym| *sym);
        let curandDestroyGenerator = __library.get(b"curandDestroyGenerator\0").map(|sym| *sym);
        let curandGetVersion = __library.get(b"curandGetVersion\0").map(|sym| *sym);
        let curandGetProperty = __library.get(b"curandGetProperty\0").map(|sym| *sym);
        let curandSetStream = __library.get(b"curandSetStream\0").map(|sym| *sym);
        let curandSetPseudoRandomGeneratorSeed = __library
            .get(b"curandSetPseudoRandomGeneratorSeed\0")
            .map(|sym| *sym);
        let curandSetGeneratorOffset = __library.get(b"curandSetGeneratorOffset\0").map(|sym| *sym);
        let curandSetGeneratorOrdering = __library
            .get(b"curandSetGeneratorOrdering\0")
            .map(|sym| *sym);
        let curandSetQuasiRandomGeneratorDimensions = __library
            .get(b"curandSetQuasiRandomGeneratorDimensions\0")
            .map(|sym| *sym);
        let curandGenerate = __library.get(b"curandGenerate\0").map(|sym| *sym);
        let curandGenerateLongLong = __library.get(b"curandGenerateLongLong\0").map(|sym| *sym);
        let curandGenerateUniform = __library.get(b"curandGenerateUniform\0").map(|sym| *sym);
        let curandGenerateUniformDouble = __library
            .get(b"curandGenerateUniformDouble\0")
            .map(|sym| *sym);
        let curandGenerateNormal = __library.get(b"curandGenerateNormal\0").map(|sym| *sym);
        let curandGenerateNormalDouble = __library
            .get(b"curandGenerateNormalDouble\0")
            .map(|sym| *sym);
        let curandGenerateLogNormal = __library.get(b"curandGenerateLogNormal\0").map(|sym| *sym);
        let curandGenerateLogNormalDouble = __library
            .get(b"curandGenerateLogNormalDouble\0")
            .map(|sym| *sym);
        let curandCreatePoissonDistribution = __library
            .get(b"curandCreatePoissonDistribution\0")
            .map(|sym| *sym);
        let curandDestroyDistribution = __library
            .get(b"curandDestroyDistribution\0")
            .map(|sym| *sym);
        let curandGeneratePoisson = __library.get(b"curandGeneratePoisson\0").map(|sym| *sym);
        let curandGeneratePoissonMethod = __library
            .get(b"curandGeneratePoissonMethod\0")
            .map(|sym| *sym);
        let curandGenerateBinomial = __library.get(b"curandGenerateBinomial\0").map(|sym| *sym);
        let curandGenerateBinomialMethod = __library
            .get(b"curandGenerateBinomialMethod\0")
            .map(|sym| *sym);
        let curandGenerateSeeds = __library.get(b"curandGenerateSeeds\0").map(|sym| *sym);
        let curandGetDirectionVectors32 = __library
            .get(b"curandGetDirectionVectors32\0")
            .map(|sym| *sym);
        let curandGetScrambleConstants32 = __library
            .get(b"curandGetScrambleConstants32\0")
            .map(|sym| *sym);
        let curandGetDirectionVectors64 = __library
            .get(b"curandGetDirectionVectors64\0")
            .map(|sym| *sym);
        let curandGetScrambleConstants64 = __library
            .get(b"curandGetScrambleConstants64\0")
            .map(|sym| *sym);
        Ok(Lib {
            __library,
            curandCreateGenerator,
            curandCreateGeneratorHost,
            curandDestroyGenerator,
            curandGetVersion,
            curandGetProperty,
            curandSetStream,
            curandSetPseudoRandomGeneratorSeed,
            curandSetGeneratorOffset,
            curandSetGeneratorOrdering,
            curandSetQuasiRandomGeneratorDimensions,
            curandGenerate,
            curandGenerateLongLong,
            curandGenerateUniform,
            curandGenerateUniformDouble,
            curandGenerateNormal,
            curandGenerateNormalDouble,
            curandGenerateLogNormal,
            curandGenerateLogNormalDouble,
            curandCreatePoissonDistribution,
            curandDestroyDistribution,
            curandGeneratePoisson,
            curandGeneratePoissonMethod,
            curandGenerateBinomial,
            curandGenerateBinomialMethod,
            curandGenerateSeeds,
            curandGetDirectionVectors32,
            curandGetScrambleConstants32,
            curandGetDirectionVectors64,
            curandGetScrambleConstants64,
        })
    }
    pub unsafe fn curandCreateGenerator(
        &self,
        generator: *mut curandGenerator_t,
        rng_type: curandRngType_t,
    ) -> curandStatus_t {
        (self
            .curandCreateGenerator
            .as_ref()
            .expect("Expected function, got error."))(generator, rng_type)
    }
    pub unsafe fn curandCreateGeneratorHost(
        &self,
        generator: *mut curandGenerator_t,
        rng_type: curandRngType_t,
    ) -> curandStatus_t {
        (self
            .curandCreateGeneratorHost
            .as_ref()
            .expect("Expected function, got error."))(generator, rng_type)
    }
    pub unsafe fn curandDestroyGenerator(&self, generator: curandGenerator_t) -> curandStatus_t {
        (self
            .curandDestroyGenerator
            .as_ref()
            .expect("Expected function, got error."))(generator)
    }
    pub unsafe fn curandGetVersion(&self, version: *mut ::core::ffi::c_int) -> curandStatus_t {
        (self
            .curandGetVersion
            .as_ref()
            .expect("Expected function, got error."))(version)
    }
    pub unsafe fn curandGetProperty(
        &self,
        type_: libraryPropertyType,
        value: *mut ::core::ffi::c_int,
    ) -> curandStatus_t {
        (self
            .curandGetProperty
            .as_ref()
            .expect("Expected function, got error."))(type_, value)
    }
    pub unsafe fn curandSetStream(
        &self,
        generator: curandGenerator_t,
        stream: cudaStream_t,
    ) -> curandStatus_t {
        (self
            .curandSetStream
            .as_ref()
            .expect("Expected function, got error."))(generator, stream)
    }
    pub unsafe fn curandSetPseudoRandomGeneratorSeed(
        &self,
        generator: curandGenerator_t,
        seed: ::core::ffi::c_ulonglong,
    ) -> curandStatus_t {
        (self
            .curandSetPseudoRandomGeneratorSeed
            .as_ref()
            .expect("Expected function, got error."))(generator, seed)
    }
    pub unsafe fn curandSetGeneratorOffset(
        &self,
        generator: curandGenerator_t,
        offset: ::core::ffi::c_ulonglong,
    ) -> curandStatus_t {
        (self
            .curandSetGeneratorOffset
            .as_ref()
            .expect("Expected function, got error."))(generator, offset)
    }
    pub unsafe fn curandSetGeneratorOrdering(
        &self,
        generator: curandGenerator_t,
        order: curandOrdering_t,
    ) -> curandStatus_t {
        (self
            .curandSetGeneratorOrdering
            .as_ref()
            .expect("Expected function, got error."))(generator, order)
    }
    pub unsafe fn curandSetQuasiRandomGeneratorDimensions(
        &self,
        generator: curandGenerator_t,
        num_dimensions: ::core::ffi::c_uint,
    ) -> curandStatus_t {
        (self
            .curandSetQuasiRandomGeneratorDimensions
            .as_ref()
            .expect("Expected function, got error."))(generator, num_dimensions)
    }
    pub unsafe fn curandGenerate(
        &self,
        generator: curandGenerator_t,
        outputPtr: *mut ::core::ffi::c_uint,
        num: usize,
    ) -> curandStatus_t {
        (self
            .curandGenerate
            .as_ref()
            .expect("Expected function, got error."))(generator, outputPtr, num)
    }
    pub unsafe fn curandGenerateLongLong(
        &self,
        generator: curandGenerator_t,
        outputPtr: *mut ::core::ffi::c_ulonglong,
        num: usize,
    ) -> curandStatus_t {
        (self
            .curandGenerateLongLong
            .as_ref()
            .expect("Expected function, got error."))(generator, outputPtr, num)
    }
    pub unsafe fn curandGenerateUniform(
        &self,
        generator: curandGenerator_t,
        outputPtr: *mut f32,
        num: usize,
    ) -> curandStatus_t {
        (self
            .curandGenerateUniform
            .as_ref()
            .expect("Expected function, got error."))(generator, outputPtr, num)
    }
    pub unsafe fn curandGenerateUniformDouble(
        &self,
        generator: curandGenerator_t,
        outputPtr: *mut f64,
        num: usize,
    ) -> curandStatus_t {
        (self
            .curandGenerateUniformDouble
            .as_ref()
            .expect("Expected function, got error."))(generator, outputPtr, num)
    }
    pub unsafe fn curandGenerateNormal(
        &self,
        generator: curandGenerator_t,
        outputPtr: *mut f32,
        n: usize,
        mean: f32,
        stddev: f32,
    ) -> curandStatus_t {
        (self
            .curandGenerateNormal
            .as_ref()
            .expect("Expected function, got error."))(generator, outputPtr, n, mean, stddev)
    }
    pub unsafe fn curandGenerateNormalDouble(
        &self,
        generator: curandGenerator_t,
        outputPtr: *mut f64,
        n: usize,
        mean: f64,
        stddev: f64,
    ) -> curandStatus_t {
        (self
            .curandGenerateNormalDouble
            .as_ref()
            .expect("Expected function, got error."))(generator, outputPtr, n, mean, stddev)
    }
    pub unsafe fn curandGenerateLogNormal(
        &self,
        generator: curandGenerator_t,
        outputPtr: *mut f32,
        n: usize,
        mean: f32,
        stddev: f32,
    ) -> curandStatus_t {
        (self
            .curandGenerateLogNormal
            .as_ref()
            .expect("Expected function, got error."))(generator, outputPtr, n, mean, stddev)
    }
    pub unsafe fn curandGenerateLogNormalDouble(
        &self,
        generator: curandGenerator_t,
        outputPtr: *mut f64,
        n: usize,
        mean: f64,
        stddev: f64,
    ) -> curandStatus_t {
        (self
            .curandGenerateLogNormalDouble
            .as_ref()
            .expect("Expected function, got error."))(generator, outputPtr, n, mean, stddev)
    }
    pub unsafe fn curandCreatePoissonDistribution(
        &self,
        lambda: f64,
        discrete_distribution: *mut curandDiscreteDistribution_t,
    ) -> curandStatus_t {
        (self
            .curandCreatePoissonDistribution
            .as_ref()
            .expect("Expected function, got error."))(lambda, discrete_distribution)
    }
    pub unsafe fn curandDestroyDistribution(
        &self,
        discrete_distribution: curandDiscreteDistribution_t,
    ) -> curandStatus_t {
        (self
            .curandDestroyDistribution
            .as_ref()
            .expect("Expected function, got error."))(discrete_distribution)
    }
    pub unsafe fn curandGeneratePoisson(
        &self,
        generator: curandGenerator_t,
        outputPtr: *mut ::core::ffi::c_uint,
        n: usize,
        lambda: f64,
    ) -> curandStatus_t {
        (self
            .curandGeneratePoisson
            .as_ref()
            .expect("Expected function, got error."))(generator, outputPtr, n, lambda)
    }
    pub unsafe fn curandGeneratePoissonMethod(
        &self,
        generator: curandGenerator_t,
        outputPtr: *mut ::core::ffi::c_uint,
        n: usize,
        lambda: f64,
        method: curandMethod_t,
    ) -> curandStatus_t {
        (self
            .curandGeneratePoissonMethod
            .as_ref()
            .expect("Expected function, got error."))(
            generator, outputPtr, n, lambda, method
        )
    }
    pub unsafe fn curandGenerateBinomial(
        &self,
        generator: curandGenerator_t,
        outputPtr: *mut ::core::ffi::c_uint,
        num: usize,
        n: ::core::ffi::c_uint,
        p: f64,
    ) -> curandStatus_t {
        (self
            .curandGenerateBinomial
            .as_ref()
            .expect("Expected function, got error."))(generator, outputPtr, num, n, p)
    }
    pub unsafe fn curandGenerateBinomialMethod(
        &self,
        generator: curandGenerator_t,
        outputPtr: *mut ::core::ffi::c_uint,
        num: usize,
        n: ::core::ffi::c_uint,
        p: f64,
        method: curandMethod_t,
    ) -> curandStatus_t {
        (self
            .curandGenerateBinomialMethod
            .as_ref()
            .expect("Expected function, got error."))(
            generator, outputPtr, num, n, p, method
        )
    }
    pub unsafe fn curandGenerateSeeds(&self, generator: curandGenerator_t) -> curandStatus_t {
        (self
            .curandGenerateSeeds
            .as_ref()
            .expect("Expected function, got error."))(generator)
    }
    pub unsafe fn curandGetDirectionVectors32(
        &self,
        vectors: *mut *mut curandDirectionVectors32_t,
        set: curandDirectionVectorSet_t,
    ) -> curandStatus_t {
        (self
            .curandGetDirectionVectors32
            .as_ref()
            .expect("Expected function, got error."))(vectors, set)
    }
    pub unsafe fn curandGetScrambleConstants32(
        &self,
        constants: *mut *mut ::core::ffi::c_uint,
    ) -> curandStatus_t {
        (self
            .curandGetScrambleConstants32
            .as_ref()
            .expect("Expected function, got error."))(constants)
    }
    pub unsafe fn curandGetDirectionVectors64(
        &self,
        vectors: *mut *mut curandDirectionVectors64_t,
        set: curandDirectionVectorSet_t,
    ) -> curandStatus_t {
        (self
            .curandGetDirectionVectors64
            .as_ref()
            .expect("Expected function, got error."))(vectors, set)
    }
    pub unsafe fn curandGetScrambleConstants64(
        &self,
        constants: *mut *mut ::core::ffi::c_ulonglong,
    ) -> curandStatus_t {
        (self
            .curandGetScrambleConstants64
            .as_ref()
            .expect("Expected function, got error."))(constants)
    }
}
