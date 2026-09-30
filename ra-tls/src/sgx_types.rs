#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
pub const _STDINT_H: u32 = 1;
pub const _FEATURES_H: u32 = 1;
pub const _DEFAULT_SOURCE: u32 = 1;
pub const __GLIBC_USE_ISOC2X: u32 = 0;
pub const __USE_ISOC11: u32 = 1;
pub const __USE_ISOC99: u32 = 1;
pub const __USE_ISOC95: u32 = 1;
pub const __USE_POSIX_IMPLICITLY: u32 = 1;
pub const _POSIX_SOURCE: u32 = 1;
pub const _POSIX_C_SOURCE: u32 = 200809;
pub const __USE_POSIX: u32 = 1;
pub const __USE_POSIX2: u32 = 1;
pub const __USE_POSIX199309: u32 = 1;
pub const __USE_POSIX199506: u32 = 1;
pub const __USE_XOPEN2K: u32 = 1;
pub const __USE_XOPEN2K8: u32 = 1;
pub const _ATFILE_SOURCE: u32 = 1;
pub const __WORDSIZE: u32 = 64;
pub const __WORDSIZE_TIME64_COMPAT32: u32 = 1;
pub const __SYSCALL_WORDSIZE: u32 = 64;
pub const __TIMESIZE: u32 = 64;
pub const __USE_MISC: u32 = 1;
pub const __USE_ATFILE: u32 = 1;
pub const __USE_FORTIFY_LEVEL: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_GETS: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_SCANF: u32 = 0;
pub const __GLIBC_USE_C2X_STRTOL: u32 = 0;
pub const _STDC_PREDEF_H: u32 = 1;
pub const __STDC_IEC_559__: u32 = 1;
pub const __STDC_IEC_60559_BFP__: u32 = 201404;
pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
pub const __STDC_IEC_60559_COMPLEX__: u32 = 201404;
pub const __STDC_ISO_10646__: u32 = 201706;
pub const __GNU_LIBRARY__: u32 = 6;
pub const __GLIBC__: u32 = 2;
pub const __GLIBC_MINOR__: u32 = 39;
pub const _SYS_CDEFS_H: u32 = 1;
pub const __glibc_c99_flexarr_available: u32 = 1;
pub const __LDOUBLE_REDIRECTS_TO_FLOAT128_ABI: u32 = 0;
pub const __HAVE_GENERIC_SELECTION: u32 = 1;
pub const __GLIBC_USE_LIB_EXT2: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT_C2X: u32 = 0;
pub const __GLIBC_USE_IEC_60559_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT_C2X: u32 = 0;
pub const __GLIBC_USE_IEC_60559_TYPES_EXT: u32 = 0;
pub const _BITS_TYPES_H: u32 = 1;
pub const _BITS_TYPESIZES_H: u32 = 1;
pub const __OFF_T_MATCHES_OFF64_T: u32 = 1;
pub const __INO_T_MATCHES_INO64_T: u32 = 1;
pub const __RLIM_T_MATCHES_RLIM64_T: u32 = 1;
pub const __STATFS_MATCHES_STATFS64: u32 = 1;
pub const __KERNEL_OLD_TIMEVAL_MATCHES_TIMEVAL64: u32 = 1;
pub const __FD_SETSIZE: u32 = 1024;
pub const _BITS_TIME64_H: u32 = 1;
pub const _BITS_WCHAR_H: u32 = 1;
pub const _BITS_STDINT_INTN_H: u32 = 1;
pub const _BITS_STDINT_UINTN_H: u32 = 1;
pub const _BITS_STDINT_LEAST_H: u32 = 1;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i64 = -9223372036854775808;
pub const INT_FAST32_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u64 = 9223372036854775807;
pub const INT_FAST32_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: i32 = -1;
pub const UINT_FAST32_MAX: i32 = -1;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const UINTPTR_MAX: i32 = -1;
pub const PTRDIFF_MIN: i64 = -9223372036854775808;
pub const PTRDIFF_MAX: u64 = 9223372036854775807;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const SIZE_MAX: i32 = -1;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 4294967295;
pub const _STDIO_H: u32 = 1;
pub const _____fpos_t_defined: u32 = 1;
pub const ____mbstate_t_defined: u32 = 1;
pub const _____fpos64_t_defined: u32 = 1;
pub const ____FILE_defined: u32 = 1;
pub const __FILE_defined: u32 = 1;
pub const __struct_FILE_defined: u32 = 1;
pub const _IO_EOF_SEEN: u32 = 16;
pub const _IO_ERR_SEEN: u32 = 32;
pub const _IO_USER_LOCK: u32 = 32768;
pub const __cookie_io_functions_t_defined: u32 = 1;
pub const _IOFBF: u32 = 0;
pub const _IOLBF: u32 = 1;
pub const _IONBF: u32 = 2;
pub const BUFSIZ: u32 = 8192;
pub const EOF: i32 = -1;
pub const SEEK_SET: u32 = 0;
pub const SEEK_CUR: u32 = 1;
pub const SEEK_END: u32 = 2;
pub const P_tmpdir: &[u8; 5] = b"/tmp\0";
pub const L_tmpnam: u32 = 20;
pub const TMP_MAX: u32 = 238328;
pub const _BITS_STDIO_LIM_H: u32 = 1;
pub const FILENAME_MAX: u32 = 4096;
pub const L_ctermid: u32 = 9;
pub const FOPEN_MAX: u32 = 16;
pub const __HAVE_FLOAT128: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT128: u32 = 0;
pub const __HAVE_FLOAT64X: u32 = 1;
pub const __HAVE_FLOAT64X_LONG_DOUBLE: u32 = 1;
pub const __HAVE_FLOAT16: u32 = 0;
pub const __HAVE_FLOAT32: u32 = 1;
pub const __HAVE_FLOAT64: u32 = 1;
pub const __HAVE_FLOAT32X: u32 = 1;
pub const __HAVE_FLOAT128X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT16: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT32: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT64: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT32X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT64X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT128X: u32 = 0;
pub const __HAVE_FLOATN_NOT_TYPEDEF: u32 = 0;
pub const RA_TLS_EPID_API_KEY: &[u8; 20] = b"RA_TLS_EPID_API_KEY\0";
pub const RA_TLS_ALLOW_OUTDATED_TCB_INSECURE: &[u8; 35] = b"RA_TLS_ALLOW_OUTDATED_TCB_INSECURE\0";
pub const RA_TLS_ALLOW_HW_CONFIG_NEEDED: &[u8; 30] = b"RA_TLS_ALLOW_HW_CONFIG_NEEDED\0";
pub const RA_TLS_ALLOW_SW_HARDENING_NEEDED: &[u8; 33] = b"RA_TLS_ALLOW_SW_HARDENING_NEEDED\0";
pub const RA_TLS_ALLOW_DEBUG_ENCLAVE_INSECURE: &[u8; 36] = b"RA_TLS_ALLOW_DEBUG_ENCLAVE_INSECURE\0";
pub const RA_TLS_MRSIGNER: &[u8; 16] = b"RA_TLS_MRSIGNER\0";
pub const RA_TLS_MRENCLAVE: &[u8; 17] = b"RA_TLS_MRENCLAVE\0";
pub const RA_TLS_ISV_PROD_ID: &[u8; 19] = b"RA_TLS_ISV_PROD_ID\0";
pub const RA_TLS_ISV_SVN: &[u8; 15] = b"RA_TLS_ISV_SVN\0";
pub const RA_TLS_IAS_PUB_KEY_PEM: &[u8; 23] = b"RA_TLS_IAS_PUB_KEY_PEM\0";
pub const RA_TLS_IAS_REPORT_URL: &[u8; 22] = b"RA_TLS_IAS_REPORT_URL\0";
pub const RA_TLS_IAS_SIGRL_URL: &[u8; 21] = b"RA_TLS_IAS_SIGRL_URL\0";
pub const RA_TLS_CERT_TIMESTAMP_NOT_BEFORE: &[u8; 33] = b"RA_TLS_CERT_TIMESTAMP_NOT_BEFORE\0";
pub const RA_TLS_CERT_TIMESTAMP_NOT_AFTER: &[u8; 32] = b"RA_TLS_CERT_TIMESTAMP_NOT_AFTER\0";
pub const __bool_true_false_are_defined: u32 = 1;
pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub const RED_ZONE_SIZE: u32 = 128;
pub const _ASSERT_H: u32 = 1;
pub const SE_KEY_SIZE: u32 = 384;
pub const SE_EXPONENT_SIZE: u32 = 4;
pub const SGX_HASH_SIZE: u32 = 32;
pub const SGX_MAC_SIZE: u32 = 16;
pub const SGX_CPUSVN_SIZE: u32 = 16;
pub const SGX_CONFIGID_SIZE: u32 = 64;
pub const SGX_KEYID_SIZE: u32 = 32;
pub const SGX_REPORT_DATA_SIZE: u32 = 64;
pub const SGX_ISVEXT_PROD_ID_SIZE: u32 = 16;
pub const SGX_ISV_FAMILY_ID_SIZE: u32 = 16;
pub const SGX_FLAGS_INITIALIZED: u32 = 1;
pub const SGX_FLAGS_DEBUG: u32 = 2;
pub const SGX_FLAGS_MODE64BIT: u32 = 4;
pub const SGX_FLAGS_PROVISION_KEY: u32 = 16;
pub const SGX_FLAGS_LICENSE_KEY: u32 = 32;
pub const SGX_FLAGS_MASK_CONST: i32 = -1;
pub const SGX_XFRM_LEGACY: u32 = 3;
pub const SGX_XFRM_AVX: u32 = 4;
pub const SGX_XFRM_MPX: u32 = 24;
pub const SGX_XFRM_AVX512: u32 = 228;
pub const SGX_XFRM_PKRU: u32 = 512;
pub const SGX_XFRM_AMX: u32 = 393216;
pub const SGX_XFRM_RESERVED: i32 = -393984;
pub const SGX_XFRM_MASK_CONST: i32 = -393445;
pub const SGX_MISCSELECT_EXINFO: u32 = 1;
pub const SGX_MISCSELECT_MASK_CONST: u32 = 4294967295;
pub const TCS_FLAGS_DBGOPTIN: u32 = 1;
pub const ERRCD_P: u32 = 1;
pub const ERRCD_W: u32 = 2;
pub const ERRCD_U: u32 = 4;
pub const ERRCD_I: u32 = 16;
pub const ERRCD_PK: u32 = 32;
pub const ERRCD_SS: u32 = 64;
pub const ERRCD_SGX: u32 = 32768;
pub const SGX_SECINFO_FLAGS_R: u32 = 1;
pub const SGX_SECINFO_FLAGS_W: u32 = 2;
pub const SGX_SECINFO_FLAGS_X: u32 = 4;
pub const SGX_SECINFO_FLAGS_PENDING: u32 = 8;
pub const SGX_SECINFO_FLAGS_MODIFIED: u32 = 16;
pub const SGX_SECINFO_FLAGS_PR: u32 = 32;
pub const SGX_SECINFO_FLAGS_TYPE_SHIFT: u32 = 8;
pub const SGX_REPORT_ACTUAL_SIZE: u32 = 432;
pub const EREPORT: u32 = 0;
pub const EGETKEY: u32 = 1;
pub const EENTER: u32 = 2;
pub const ERESUME: u32 = 3;
pub const EEXIT: u32 = 4;
pub const EACCEPT: u32 = 5;
pub const EMODPE: u32 = 6;
pub const EACCEPTCOPY: u32 = 7;
pub const SGX_LAUNCH_KEY: u32 = 0;
pub const SGX_PROVISION_KEY: u32 = 1;
pub const SGX_PROVISION_SEAL_KEY: u32 = 2;
pub const SGX_REPORT_KEY: u32 = 3;
pub const SGX_SEAL_KEY: u32 = 4;
pub const SGX_KEYPOLICY_MRENCLAVE: u32 = 1;
pub const SGX_KEYPOLICY_MRSIGNER: u32 = 2;
pub const RFLAGS_DF: u32 = 1024;
pub const RFLAGS_AC: u32 = 262144;
pub const SGX_QUOTE_MAX_SIZE: u32 = 8192;
pub type __u_char = ::std::os::raw::c_uchar;
pub type __u_short = ::std::os::raw::c_ushort;
pub type __u_int = ::std::os::raw::c_uint;
pub type __u_long = ::std::os::raw::c_ulong;
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_long;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type __int_least8_t = __int8_t;
pub type __uint_least8_t = __uint8_t;
pub type __int_least16_t = __int16_t;
pub type __uint_least16_t = __uint16_t;
pub type __int_least32_t = __int32_t;
pub type __uint_least32_t = __uint32_t;
pub type __int_least64_t = __int64_t;
pub type __uint_least64_t = __uint64_t;
pub type __quad_t = ::std::os::raw::c_long;
pub type __u_quad_t = ::std::os::raw::c_ulong;
pub type __intmax_t = ::std::os::raw::c_long;
pub type __uintmax_t = ::std::os::raw::c_ulong;
pub type __dev_t = ::std::os::raw::c_ulong;
pub type __uid_t = ::std::os::raw::c_uint;
pub type __gid_t = ::std::os::raw::c_uint;
pub type __ino_t = ::std::os::raw::c_ulong;
pub type __ino64_t = ::std::os::raw::c_ulong;
pub type __mode_t = ::std::os::raw::c_uint;
pub type __nlink_t = ::std::os::raw::c_ulong;
pub type __off_t = ::std::os::raw::c_long;
pub type __off64_t = ::std::os::raw::c_long;
pub type __pid_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __fsid_t {
    pub __val: [::std::os::raw::c_int; 2usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __fsid_t"][::std::mem::size_of::<__fsid_t>() - 8usize];
    ["Alignment of __fsid_t"][::std::mem::align_of::<__fsid_t>() - 4usize];
    ["Offset of field: __fsid_t::__val"][::std::mem::offset_of!(__fsid_t, __val) - 0usize];
};
pub type __clock_t = ::std::os::raw::c_long;
pub type __rlim_t = ::std::os::raw::c_ulong;
pub type __rlim64_t = ::std::os::raw::c_ulong;
pub type __id_t = ::std::os::raw::c_uint;
pub type __time_t = ::std::os::raw::c_long;
pub type __useconds_t = ::std::os::raw::c_uint;
pub type __suseconds_t = ::std::os::raw::c_long;
pub type __suseconds64_t = ::std::os::raw::c_long;
pub type __daddr_t = ::std::os::raw::c_int;
pub type __key_t = ::std::os::raw::c_int;
pub type __clockid_t = ::std::os::raw::c_int;
pub type __timer_t = *mut ::std::os::raw::c_void;
pub type __blksize_t = ::std::os::raw::c_long;
pub type __blkcnt_t = ::std::os::raw::c_long;
pub type __blkcnt64_t = ::std::os::raw::c_long;
pub type __fsblkcnt_t = ::std::os::raw::c_ulong;
pub type __fsblkcnt64_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt64_t = ::std::os::raw::c_ulong;
pub type __fsword_t = ::std::os::raw::c_long;
pub type __ssize_t = ::std::os::raw::c_long;
pub type __syscall_slong_t = ::std::os::raw::c_long;
pub type __syscall_ulong_t = ::std::os::raw::c_ulong;
pub type __loff_t = __off64_t;
pub type __caddr_t = *mut ::std::os::raw::c_char;
pub type __intptr_t = ::std::os::raw::c_long;
pub type __socklen_t = ::std::os::raw::c_uint;
pub type __sig_atomic_t = ::std::os::raw::c_int;
pub type int_least8_t = __int_least8_t;
pub type int_least16_t = __int_least16_t;
pub type int_least32_t = __int_least32_t;
pub type int_least64_t = __int_least64_t;
pub type uint_least8_t = __uint_least8_t;
pub type uint_least16_t = __uint_least16_t;
pub type uint_least32_t = __uint_least32_t;
pub type uint_least64_t = __uint_least64_t;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_long;
pub type int_fast32_t = ::std::os::raw::c_long;
pub type int_fast64_t = ::std::os::raw::c_long;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_ulong;
pub type uint_fast32_t = ::std::os::raw::c_ulong;
pub type uint_fast64_t = ::std::os::raw::c_ulong;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type __gnuc_va_list = __builtin_va_list;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __mbstate_t {
    pub __count: ::std::os::raw::c_int,
    pub __value: __mbstate_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __mbstate_t__bindgen_ty_1 {
    pub __wch: ::std::os::raw::c_uint,
    pub __wchb: [::std::os::raw::c_char; 4usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __mbstate_t__bindgen_ty_1"]
        [::std::mem::size_of::<__mbstate_t__bindgen_ty_1>() - 4usize];
    ["Alignment of __mbstate_t__bindgen_ty_1"]
        [::std::mem::align_of::<__mbstate_t__bindgen_ty_1>() - 4usize];
    ["Offset of field: __mbstate_t__bindgen_ty_1::__wch"]
        [::std::mem::offset_of!(__mbstate_t__bindgen_ty_1, __wch) - 0usize];
    ["Offset of field: __mbstate_t__bindgen_ty_1::__wchb"]
        [::std::mem::offset_of!(__mbstate_t__bindgen_ty_1, __wchb) - 0usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __mbstate_t"][::std::mem::size_of::<__mbstate_t>() - 8usize];
    ["Alignment of __mbstate_t"][::std::mem::align_of::<__mbstate_t>() - 4usize];
    ["Offset of field: __mbstate_t::__count"]
        [::std::mem::offset_of!(__mbstate_t, __count) - 0usize];
    ["Offset of field: __mbstate_t::__value"]
        [::std::mem::offset_of!(__mbstate_t, __value) - 4usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _G_fpos_t {
    pub __pos: __off_t,
    pub __state: __mbstate_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _G_fpos_t"][::std::mem::size_of::<_G_fpos_t>() - 16usize];
    ["Alignment of _G_fpos_t"][::std::mem::align_of::<_G_fpos_t>() - 8usize];
    ["Offset of field: _G_fpos_t::__pos"][::std::mem::offset_of!(_G_fpos_t, __pos) - 0usize];
    ["Offset of field: _G_fpos_t::__state"][::std::mem::offset_of!(_G_fpos_t, __state) - 8usize];
};
pub type __fpos_t = _G_fpos_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _G_fpos64_t {
    pub __pos: __off64_t,
    pub __state: __mbstate_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _G_fpos64_t"][::std::mem::size_of::<_G_fpos64_t>() - 16usize];
    ["Alignment of _G_fpos64_t"][::std::mem::align_of::<_G_fpos64_t>() - 8usize];
    ["Offset of field: _G_fpos64_t::__pos"][::std::mem::offset_of!(_G_fpos64_t, __pos) - 0usize];
    ["Offset of field: _G_fpos64_t::__state"]
        [::std::mem::offset_of!(_G_fpos64_t, __state) - 8usize];
};
pub type __fpos64_t = _G_fpos64_t;
pub type __FILE = _IO_FILE;
pub type FILE = _IO_FILE;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_marker {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_codecvt {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_wide_data {
    _unused: [u8; 0],
}
pub type _IO_lock_t = ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_FILE {
    pub _flags: ::std::os::raw::c_int,
    pub _IO_read_ptr: *mut ::std::os::raw::c_char,
    pub _IO_read_end: *mut ::std::os::raw::c_char,
    pub _IO_read_base: *mut ::std::os::raw::c_char,
    pub _IO_write_base: *mut ::std::os::raw::c_char,
    pub _IO_write_ptr: *mut ::std::os::raw::c_char,
    pub _IO_write_end: *mut ::std::os::raw::c_char,
    pub _IO_buf_base: *mut ::std::os::raw::c_char,
    pub _IO_buf_end: *mut ::std::os::raw::c_char,
    pub _IO_save_base: *mut ::std::os::raw::c_char,
    pub _IO_backup_base: *mut ::std::os::raw::c_char,
    pub _IO_save_end: *mut ::std::os::raw::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::std::os::raw::c_int,
    pub _flags2: ::std::os::raw::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::std::os::raw::c_ushort,
    pub _vtable_offset: ::std::os::raw::c_schar,
    pub _shortbuf: [::std::os::raw::c_char; 1usize],
    pub _lock: *mut _IO_lock_t,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::std::os::raw::c_void,
    pub __pad5: usize,
    pub _mode: ::std::os::raw::c_int,
    pub _unused2: [::std::os::raw::c_char; 20usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _IO_FILE"][::std::mem::size_of::<_IO_FILE>() - 216usize];
    ["Alignment of _IO_FILE"][::std::mem::align_of::<_IO_FILE>() - 8usize];
    ["Offset of field: _IO_FILE::_flags"][::std::mem::offset_of!(_IO_FILE, _flags) - 0usize];
    ["Offset of field: _IO_FILE::_IO_read_ptr"]
        [::std::mem::offset_of!(_IO_FILE, _IO_read_ptr) - 8usize];
    ["Offset of field: _IO_FILE::_IO_read_end"]
        [::std::mem::offset_of!(_IO_FILE, _IO_read_end) - 16usize];
    ["Offset of field: _IO_FILE::_IO_read_base"]
        [::std::mem::offset_of!(_IO_FILE, _IO_read_base) - 24usize];
    ["Offset of field: _IO_FILE::_IO_write_base"]
        [::std::mem::offset_of!(_IO_FILE, _IO_write_base) - 32usize];
    ["Offset of field: _IO_FILE::_IO_write_ptr"]
        [::std::mem::offset_of!(_IO_FILE, _IO_write_ptr) - 40usize];
    ["Offset of field: _IO_FILE::_IO_write_end"]
        [::std::mem::offset_of!(_IO_FILE, _IO_write_end) - 48usize];
    ["Offset of field: _IO_FILE::_IO_buf_base"]
        [::std::mem::offset_of!(_IO_FILE, _IO_buf_base) - 56usize];
    ["Offset of field: _IO_FILE::_IO_buf_end"]
        [::std::mem::offset_of!(_IO_FILE, _IO_buf_end) - 64usize];
    ["Offset of field: _IO_FILE::_IO_save_base"]
        [::std::mem::offset_of!(_IO_FILE, _IO_save_base) - 72usize];
    ["Offset of field: _IO_FILE::_IO_backup_base"]
        [::std::mem::offset_of!(_IO_FILE, _IO_backup_base) - 80usize];
    ["Offset of field: _IO_FILE::_IO_save_end"]
        [::std::mem::offset_of!(_IO_FILE, _IO_save_end) - 88usize];
    ["Offset of field: _IO_FILE::_markers"][::std::mem::offset_of!(_IO_FILE, _markers) - 96usize];
    ["Offset of field: _IO_FILE::_chain"][::std::mem::offset_of!(_IO_FILE, _chain) - 104usize];
    ["Offset of field: _IO_FILE::_fileno"][::std::mem::offset_of!(_IO_FILE, _fileno) - 112usize];
    ["Offset of field: _IO_FILE::_flags2"][::std::mem::offset_of!(_IO_FILE, _flags2) - 116usize];
    ["Offset of field: _IO_FILE::_old_offset"]
        [::std::mem::offset_of!(_IO_FILE, _old_offset) - 120usize];
    ["Offset of field: _IO_FILE::_cur_column"]
        [::std::mem::offset_of!(_IO_FILE, _cur_column) - 128usize];
    ["Offset of field: _IO_FILE::_vtable_offset"]
        [::std::mem::offset_of!(_IO_FILE, _vtable_offset) - 130usize];
    ["Offset of field: _IO_FILE::_shortbuf"]
        [::std::mem::offset_of!(_IO_FILE, _shortbuf) - 131usize];
    ["Offset of field: _IO_FILE::_lock"][::std::mem::offset_of!(_IO_FILE, _lock) - 136usize];
    ["Offset of field: _IO_FILE::_offset"][::std::mem::offset_of!(_IO_FILE, _offset) - 144usize];
    ["Offset of field: _IO_FILE::_codecvt"][::std::mem::offset_of!(_IO_FILE, _codecvt) - 152usize];
    ["Offset of field: _IO_FILE::_wide_data"]
        [::std::mem::offset_of!(_IO_FILE, _wide_data) - 160usize];
    ["Offset of field: _IO_FILE::_freeres_list"]
        [::std::mem::offset_of!(_IO_FILE, _freeres_list) - 168usize];
    ["Offset of field: _IO_FILE::_freeres_buf"]
        [::std::mem::offset_of!(_IO_FILE, _freeres_buf) - 176usize];
    ["Offset of field: _IO_FILE::__pad5"][::std::mem::offset_of!(_IO_FILE, __pad5) - 184usize];
    ["Offset of field: _IO_FILE::_mode"][::std::mem::offset_of!(_IO_FILE, _mode) - 192usize];
    ["Offset of field: _IO_FILE::_unused2"][::std::mem::offset_of!(_IO_FILE, _unused2) - 196usize];
};
pub type cookie_read_function_t = ::std::option::Option<
    unsafe extern "C" fn(
        __cookie: *mut ::std::os::raw::c_void,
        __buf: *mut ::std::os::raw::c_char,
        __nbytes: usize,
    ) -> __ssize_t,
>;
pub type cookie_write_function_t = ::std::option::Option<
    unsafe extern "C" fn(
        __cookie: *mut ::std::os::raw::c_void,
        __buf: *const ::std::os::raw::c_char,
        __nbytes: usize,
    ) -> __ssize_t,
>;
pub type cookie_seek_function_t = ::std::option::Option<
    unsafe extern "C" fn(
        __cookie: *mut ::std::os::raw::c_void,
        __pos: *mut __off64_t,
        __w: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
pub type cookie_close_function_t = ::std::option::Option<
    unsafe extern "C" fn(__cookie: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_cookie_io_functions_t {
    pub read: cookie_read_function_t,
    pub write: cookie_write_function_t,
    pub seek: cookie_seek_function_t,
    pub close: cookie_close_function_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _IO_cookie_io_functions_t"]
        [::std::mem::size_of::<_IO_cookie_io_functions_t>() - 32usize];
    ["Alignment of _IO_cookie_io_functions_t"]
        [::std::mem::align_of::<_IO_cookie_io_functions_t>() - 8usize];
    ["Offset of field: _IO_cookie_io_functions_t::read"]
        [::std::mem::offset_of!(_IO_cookie_io_functions_t, read) - 0usize];
    ["Offset of field: _IO_cookie_io_functions_t::write"]
        [::std::mem::offset_of!(_IO_cookie_io_functions_t, write) - 8usize];
    ["Offset of field: _IO_cookie_io_functions_t::seek"]
        [::std::mem::offset_of!(_IO_cookie_io_functions_t, seek) - 16usize];
    ["Offset of field: _IO_cookie_io_functions_t::close"]
        [::std::mem::offset_of!(_IO_cookie_io_functions_t, close) - 24usize];
};
pub type cookie_io_functions_t = _IO_cookie_io_functions_t;
pub type va_list = __gnuc_va_list;
pub type off_t = __off_t;
pub type fpos_t = __fpos_t;
extern "C" {
    pub static mut stdin: *mut FILE;
}
extern "C" {
    pub static mut stdout: *mut FILE;
}
extern "C" {
    pub static mut stderr: *mut FILE;
}
pub type _Float32 = f32;
pub type _Float64 = f64;
pub type _Float32x = f64;
pub type _Float64x = u128;
pub const ra_tls_attestation_scheme_t_RA_TLS_ATTESTATION_SCHEME_UNKNOWN:
    ra_tls_attestation_scheme_t = 0;
pub const ra_tls_attestation_scheme_t_RA_TLS_ATTESTATION_SCHEME_EPID: ra_tls_attestation_scheme_t =
    1;
pub const ra_tls_attestation_scheme_t_RA_TLS_ATTESTATION_SCHEME_DCAP: ra_tls_attestation_scheme_t =
    2;
pub type ra_tls_attestation_scheme_t = ::std::os::raw::c_uint;
pub const ra_tls_err_loc_t_AT_NONE: ra_tls_err_loc_t = 0;
pub const ra_tls_err_loc_t_AT_INIT: ra_tls_err_loc_t = 1;
pub const ra_tls_err_loc_t_AT_EXTRACT_QUOTE: ra_tls_err_loc_t = 2;
pub const ra_tls_err_loc_t_AT_VERIFY_EXTERNAL: ra_tls_err_loc_t = 3;
pub const ra_tls_err_loc_t_AT_VERIFY_ENCLAVE_ATTRS: ra_tls_err_loc_t = 4;
pub const ra_tls_err_loc_t_AT_VERIFY_ENCLAVE_MEASUREMENTS: ra_tls_err_loc_t = 5;
pub type ra_tls_err_loc_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ra_tls_verify_callback_results {
    pub attestation_scheme: ra_tls_attestation_scheme_t,
    pub err_loc: ra_tls_err_loc_t,
    pub __bindgen_anon_1: ra_tls_verify_callback_results__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ra_tls_verify_callback_results__bindgen_ty_1 {
    pub epid: ra_tls_verify_callback_results__bindgen_ty_1__bindgen_ty_1,
    pub dcap: ra_tls_verify_callback_results__bindgen_ty_1__bindgen_ty_2,
    pub misc: ra_tls_verify_callback_results__bindgen_ty_1__bindgen_ty_3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ra_tls_verify_callback_results__bindgen_ty_1__bindgen_ty_1 {
    pub ias_enclave_quote_status: [::std::os::raw::c_char; 128usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ra_tls_verify_callback_results__bindgen_ty_1__bindgen_ty_1"][::std::mem::size_of::<
        ra_tls_verify_callback_results__bindgen_ty_1__bindgen_ty_1,
    >() - 128usize];
    ["Alignment of ra_tls_verify_callback_results__bindgen_ty_1__bindgen_ty_1"][::std::mem::align_of::<
        ra_tls_verify_callback_results__bindgen_ty_1__bindgen_ty_1,
    >() - 1usize];
    ["Offset of field: ra_tls_verify_callback_results__bindgen_ty_1__bindgen_ty_1::ias_enclave_quote_status"] [:: std :: mem :: offset_of ! (ra_tls_verify_callback_results__bindgen_ty_1__bindgen_ty_1 , ias_enclave_quote_status) - 0usize] ;
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ra_tls_verify_callback_results__bindgen_ty_1__bindgen_ty_2 {
    pub func_verify_quote_result: ::std::os::raw::c_int,
    pub quote_verification_result: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ra_tls_verify_callback_results__bindgen_ty_1__bindgen_ty_2"][::std::mem::size_of::<
        ra_tls_verify_callback_results__bindgen_ty_1__bindgen_ty_2,
    >() - 8usize];
    ["Alignment of ra_tls_verify_callback_results__bindgen_ty_1__bindgen_ty_2"][::std::mem::align_of::<
        ra_tls_verify_callback_results__bindgen_ty_1__bindgen_ty_2,
    >() - 4usize];
    ["Offset of field: ra_tls_verify_callback_results__bindgen_ty_1__bindgen_ty_2::func_verify_quote_result"] [:: std :: mem :: offset_of ! (ra_tls_verify_callback_results__bindgen_ty_1__bindgen_ty_2 , func_verify_quote_result) - 0usize] ;
    ["Offset of field: ra_tls_verify_callback_results__bindgen_ty_1__bindgen_ty_2::quote_verification_result"] [:: std :: mem :: offset_of ! (ra_tls_verify_callback_results__bindgen_ty_1__bindgen_ty_2 , quote_verification_result) - 4usize] ;
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ra_tls_verify_callback_results__bindgen_ty_1__bindgen_ty_3 {
    pub reserved: [::std::os::raw::c_char; 128usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ra_tls_verify_callback_results__bindgen_ty_1__bindgen_ty_3"][::std::mem::size_of::<
        ra_tls_verify_callback_results__bindgen_ty_1__bindgen_ty_3,
    >() - 128usize];
    ["Alignment of ra_tls_verify_callback_results__bindgen_ty_1__bindgen_ty_3"][::std::mem::align_of::<
        ra_tls_verify_callback_results__bindgen_ty_1__bindgen_ty_3,
    >() - 1usize];
    ["Offset of field: ra_tls_verify_callback_results__bindgen_ty_1__bindgen_ty_3::reserved"][::std::mem::offset_of!(
        ra_tls_verify_callback_results__bindgen_ty_1__bindgen_ty_3,
        reserved
    )
        - 0usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ra_tls_verify_callback_results__bindgen_ty_1"]
        [::std::mem::size_of::<ra_tls_verify_callback_results__bindgen_ty_1>() - 128usize];
    ["Alignment of ra_tls_verify_callback_results__bindgen_ty_1"]
        [::std::mem::align_of::<ra_tls_verify_callback_results__bindgen_ty_1>() - 4usize];
    ["Offset of field: ra_tls_verify_callback_results__bindgen_ty_1::epid"]
        [::std::mem::offset_of!(ra_tls_verify_callback_results__bindgen_ty_1, epid) - 0usize];
    ["Offset of field: ra_tls_verify_callback_results__bindgen_ty_1::dcap"]
        [::std::mem::offset_of!(ra_tls_verify_callback_results__bindgen_ty_1, dcap) - 0usize];
    ["Offset of field: ra_tls_verify_callback_results__bindgen_ty_1::misc"]
        [::std::mem::offset_of!(ra_tls_verify_callback_results__bindgen_ty_1, misc) - 0usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ra_tls_verify_callback_results"]
        [::std::mem::size_of::<ra_tls_verify_callback_results>() - 136usize];
    ["Alignment of ra_tls_verify_callback_results"]
        [::std::mem::align_of::<ra_tls_verify_callback_results>() - 4usize];
    ["Offset of field: ra_tls_verify_callback_results::attestation_scheme"]
        [::std::mem::offset_of!(ra_tls_verify_callback_results, attestation_scheme) - 0usize];
    ["Offset of field: ra_tls_verify_callback_results::err_loc"]
        [::std::mem::offset_of!(ra_tls_verify_callback_results, err_loc) - 4usize];
};
pub type verify_measurements_cb_t = ::std::option::Option<
    unsafe extern "C" fn(
        mrenclave: *const ::std::os::raw::c_char,
        mrsigner: *const ::std::os::raw::c_char,
        isv_prod_id: *const ::std::os::raw::c_char,
        isv_svn: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int,
>;
pub type wchar_t = ::std::os::raw::c_int;
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: u128,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of max_align_t"][::std::mem::size_of::<max_align_t>() - 32usize];
    ["Alignment of max_align_t"][::std::mem::align_of::<max_align_t>() - 16usize];
    ["Offset of field: max_align_t::__clang_max_align_nonce1"]
        [::std::mem::offset_of!(max_align_t, __clang_max_align_nonce1) - 0usize];
    ["Offset of field: max_align_t::__clang_max_align_nonce2"]
        [::std::mem::offset_of!(max_align_t, __clang_max_align_nonce2) - 16usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _sgx_measurement_t {
    pub m: [u8; 32usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _sgx_measurement_t"][::std::mem::size_of::<_sgx_measurement_t>() - 32usize];
    ["Alignment of _sgx_measurement_t"][::std::mem::align_of::<_sgx_measurement_t>() - 1usize];
    ["Offset of field: _sgx_measurement_t::m"]
        [::std::mem::offset_of!(_sgx_measurement_t, m) - 0usize];
};
pub type sgx_measurement_t = _sgx_measurement_t;
pub type sgx_mac_t = [u8; 16usize];
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct _sgx_attributes_t {
    pub flags: u64,
    pub xfrm: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _sgx_attributes_t"][::std::mem::size_of::<_sgx_attributes_t>() - 16usize];
    ["Alignment of _sgx_attributes_t"][::std::mem::align_of::<_sgx_attributes_t>() - 1usize];
    ["Offset of field: _sgx_attributes_t::flags"]
        [::std::mem::offset_of!(_sgx_attributes_t, flags) - 0usize];
    ["Offset of field: _sgx_attributes_t::xfrm"]
        [::std::mem::offset_of!(_sgx_attributes_t, xfrm) - 8usize];
};
pub type sgx_attributes_t = _sgx_attributes_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _sgx_cpu_svn_t {
    pub svn: [u8; 16usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _sgx_cpu_svn_t"][::std::mem::size_of::<_sgx_cpu_svn_t>() - 16usize];
    ["Alignment of _sgx_cpu_svn_t"][::std::mem::align_of::<_sgx_cpu_svn_t>() - 1usize];
    ["Offset of field: _sgx_cpu_svn_t::svn"][::std::mem::offset_of!(_sgx_cpu_svn_t, svn) - 0usize];
};
pub type sgx_cpu_svn_t = _sgx_cpu_svn_t;
pub type sgx_cet_attributes_t = u8;
pub type sgx_misc_select_t = u32;
pub type sgx_prod_id_t = u16;
pub type sgx_isv_svn_t = u16;
pub type sgx_config_svn_t = u16;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sgx_config_id_t {
    pub data: [u8; 64usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of sgx_config_id_t"][::std::mem::size_of::<sgx_config_id_t>() - 64usize];
    ["Alignment of sgx_config_id_t"][::std::mem::align_of::<sgx_config_id_t>() - 1usize];
    ["Offset of field: sgx_config_id_t::data"]
        [::std::mem::offset_of!(sgx_config_id_t, data) - 0usize];
};
pub type sgx_isvext_prod_id_t = [u8; 16usize];
pub type sgx_isvfamily_id_t = [u8; 16usize];
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct sgx_arch_secs_t {
    pub size: u64,
    pub base: u64,
    pub ssa_frame_size: u32,
    pub misc_select: sgx_misc_select_t,
    pub cet_legacy_bitmap_offset: [u8; 8usize],
    pub cet_attributes: sgx_cet_attributes_t,
    pub reserved1: [u8; 15usize],
    pub attributes: sgx_attributes_t,
    pub mr_enclave: sgx_measurement_t,
    pub reserved2: [u8; 32usize],
    pub mr_signer: sgx_measurement_t,
    pub reserved3: [u8; 32usize],
    pub config_id: sgx_config_id_t,
    pub isv_prod_id: sgx_prod_id_t,
    pub isv_svn: sgx_isv_svn_t,
    pub config_svn: sgx_config_svn_t,
    pub reserved4: [u8; 3834usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of sgx_arch_secs_t"][::std::mem::size_of::<sgx_arch_secs_t>() - 4096usize];
    ["Alignment of sgx_arch_secs_t"][::std::mem::align_of::<sgx_arch_secs_t>() - 1usize];
    ["Offset of field: sgx_arch_secs_t::size"]
        [::std::mem::offset_of!(sgx_arch_secs_t, size) - 0usize];
    ["Offset of field: sgx_arch_secs_t::base"]
        [::std::mem::offset_of!(sgx_arch_secs_t, base) - 8usize];
    ["Offset of field: sgx_arch_secs_t::ssa_frame_size"]
        [::std::mem::offset_of!(sgx_arch_secs_t, ssa_frame_size) - 16usize];
    ["Offset of field: sgx_arch_secs_t::misc_select"]
        [::std::mem::offset_of!(sgx_arch_secs_t, misc_select) - 20usize];
    ["Offset of field: sgx_arch_secs_t::cet_legacy_bitmap_offset"]
        [::std::mem::offset_of!(sgx_arch_secs_t, cet_legacy_bitmap_offset) - 24usize];
    ["Offset of field: sgx_arch_secs_t::cet_attributes"]
        [::std::mem::offset_of!(sgx_arch_secs_t, cet_attributes) - 32usize];
    ["Offset of field: sgx_arch_secs_t::reserved1"]
        [::std::mem::offset_of!(sgx_arch_secs_t, reserved1) - 33usize];
    ["Offset of field: sgx_arch_secs_t::attributes"]
        [::std::mem::offset_of!(sgx_arch_secs_t, attributes) - 48usize];
    ["Offset of field: sgx_arch_secs_t::mr_enclave"]
        [::std::mem::offset_of!(sgx_arch_secs_t, mr_enclave) - 64usize];
    ["Offset of field: sgx_arch_secs_t::reserved2"]
        [::std::mem::offset_of!(sgx_arch_secs_t, reserved2) - 96usize];
    ["Offset of field: sgx_arch_secs_t::mr_signer"]
        [::std::mem::offset_of!(sgx_arch_secs_t, mr_signer) - 128usize];
    ["Offset of field: sgx_arch_secs_t::reserved3"]
        [::std::mem::offset_of!(sgx_arch_secs_t, reserved3) - 160usize];
    ["Offset of field: sgx_arch_secs_t::config_id"]
        [::std::mem::offset_of!(sgx_arch_secs_t, config_id) - 192usize];
    ["Offset of field: sgx_arch_secs_t::isv_prod_id"]
        [::std::mem::offset_of!(sgx_arch_secs_t, isv_prod_id) - 256usize];
    ["Offset of field: sgx_arch_secs_t::isv_svn"]
        [::std::mem::offset_of!(sgx_arch_secs_t, isv_svn) - 258usize];
    ["Offset of field: sgx_arch_secs_t::config_svn"]
        [::std::mem::offset_of!(sgx_arch_secs_t, config_svn) - 260usize];
    ["Offset of field: sgx_arch_secs_t::reserved4"]
        [::std::mem::offset_of!(sgx_arch_secs_t, reserved4) - 262usize];
};
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct sgx_arch_tcs_t {
    pub reserved0: u64,
    pub flags: u64,
    pub ossa: u64,
    pub cssa: u32,
    pub nssa: u32,
    pub oentry: u64,
    pub reserved1: u64,
    pub ofs_base: u64,
    pub ogs_base: u64,
    pub ofs_limit: u32,
    pub ogs_limit: u32,
    pub reserved3: [u8; 4024usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of sgx_arch_tcs_t"][::std::mem::size_of::<sgx_arch_tcs_t>() - 4096usize];
    ["Alignment of sgx_arch_tcs_t"][::std::mem::align_of::<sgx_arch_tcs_t>() - 1usize];
    ["Offset of field: sgx_arch_tcs_t::reserved0"]
        [::std::mem::offset_of!(sgx_arch_tcs_t, reserved0) - 0usize];
    ["Offset of field: sgx_arch_tcs_t::flags"]
        [::std::mem::offset_of!(sgx_arch_tcs_t, flags) - 8usize];
    ["Offset of field: sgx_arch_tcs_t::ossa"]
        [::std::mem::offset_of!(sgx_arch_tcs_t, ossa) - 16usize];
    ["Offset of field: sgx_arch_tcs_t::cssa"]
        [::std::mem::offset_of!(sgx_arch_tcs_t, cssa) - 24usize];
    ["Offset of field: sgx_arch_tcs_t::nssa"]
        [::std::mem::offset_of!(sgx_arch_tcs_t, nssa) - 28usize];
    ["Offset of field: sgx_arch_tcs_t::oentry"]
        [::std::mem::offset_of!(sgx_arch_tcs_t, oentry) - 32usize];
    ["Offset of field: sgx_arch_tcs_t::reserved1"]
        [::std::mem::offset_of!(sgx_arch_tcs_t, reserved1) - 40usize];
    ["Offset of field: sgx_arch_tcs_t::ofs_base"]
        [::std::mem::offset_of!(sgx_arch_tcs_t, ofs_base) - 48usize];
    ["Offset of field: sgx_arch_tcs_t::ogs_base"]
        [::std::mem::offset_of!(sgx_arch_tcs_t, ogs_base) - 56usize];
    ["Offset of field: sgx_arch_tcs_t::ofs_limit"]
        [::std::mem::offset_of!(sgx_arch_tcs_t, ofs_limit) - 64usize];
    ["Offset of field: sgx_arch_tcs_t::ogs_limit"]
        [::std::mem::offset_of!(sgx_arch_tcs_t, ogs_limit) - 68usize];
    ["Offset of field: sgx_arch_tcs_t::reserved3"]
        [::std::mem::offset_of!(sgx_arch_tcs_t, reserved3) - 72usize];
};
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct sgx_pal_gpr_t {
    pub rax: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rbx: u64,
    pub rsp: u64,
    pub rbp: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rflags: u64,
    pub rip: u64,
    pub ursp: u64,
    pub urbp: u64,
    pub exitinfo: u32,
    pub reserved: u32,
    pub fsbase: u64,
    pub gsbase: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of sgx_pal_gpr_t"][::std::mem::size_of::<sgx_pal_gpr_t>() - 184usize];
    ["Alignment of sgx_pal_gpr_t"][::std::mem::align_of::<sgx_pal_gpr_t>() - 1usize];
    ["Offset of field: sgx_pal_gpr_t::rax"][::std::mem::offset_of!(sgx_pal_gpr_t, rax) - 0usize];
    ["Offset of field: sgx_pal_gpr_t::rcx"][::std::mem::offset_of!(sgx_pal_gpr_t, rcx) - 8usize];
    ["Offset of field: sgx_pal_gpr_t::rdx"][::std::mem::offset_of!(sgx_pal_gpr_t, rdx) - 16usize];
    ["Offset of field: sgx_pal_gpr_t::rbx"][::std::mem::offset_of!(sgx_pal_gpr_t, rbx) - 24usize];
    ["Offset of field: sgx_pal_gpr_t::rsp"][::std::mem::offset_of!(sgx_pal_gpr_t, rsp) - 32usize];
    ["Offset of field: sgx_pal_gpr_t::rbp"][::std::mem::offset_of!(sgx_pal_gpr_t, rbp) - 40usize];
    ["Offset of field: sgx_pal_gpr_t::rsi"][::std::mem::offset_of!(sgx_pal_gpr_t, rsi) - 48usize];
    ["Offset of field: sgx_pal_gpr_t::rdi"][::std::mem::offset_of!(sgx_pal_gpr_t, rdi) - 56usize];
    ["Offset of field: sgx_pal_gpr_t::r8"][::std::mem::offset_of!(sgx_pal_gpr_t, r8) - 64usize];
    ["Offset of field: sgx_pal_gpr_t::r9"][::std::mem::offset_of!(sgx_pal_gpr_t, r9) - 72usize];
    ["Offset of field: sgx_pal_gpr_t::r10"][::std::mem::offset_of!(sgx_pal_gpr_t, r10) - 80usize];
    ["Offset of field: sgx_pal_gpr_t::r11"][::std::mem::offset_of!(sgx_pal_gpr_t, r11) - 88usize];
    ["Offset of field: sgx_pal_gpr_t::r12"][::std::mem::offset_of!(sgx_pal_gpr_t, r12) - 96usize];
    ["Offset of field: sgx_pal_gpr_t::r13"][::std::mem::offset_of!(sgx_pal_gpr_t, r13) - 104usize];
    ["Offset of field: sgx_pal_gpr_t::r14"][::std::mem::offset_of!(sgx_pal_gpr_t, r14) - 112usize];
    ["Offset of field: sgx_pal_gpr_t::r15"][::std::mem::offset_of!(sgx_pal_gpr_t, r15) - 120usize];
    ["Offset of field: sgx_pal_gpr_t::rflags"]
        [::std::mem::offset_of!(sgx_pal_gpr_t, rflags) - 128usize];
    ["Offset of field: sgx_pal_gpr_t::rip"][::std::mem::offset_of!(sgx_pal_gpr_t, rip) - 136usize];
    ["Offset of field: sgx_pal_gpr_t::ursp"]
        [::std::mem::offset_of!(sgx_pal_gpr_t, ursp) - 144usize];
    ["Offset of field: sgx_pal_gpr_t::urbp"]
        [::std::mem::offset_of!(sgx_pal_gpr_t, urbp) - 152usize];
    ["Offset of field: sgx_pal_gpr_t::exitinfo"]
        [::std::mem::offset_of!(sgx_pal_gpr_t, exitinfo) - 160usize];
    ["Offset of field: sgx_pal_gpr_t::reserved"]
        [::std::mem::offset_of!(sgx_pal_gpr_t, reserved) - 164usize];
    ["Offset of field: sgx_pal_gpr_t::fsbase"]
        [::std::mem::offset_of!(sgx_pal_gpr_t, fsbase) - 168usize];
    ["Offset of field: sgx_pal_gpr_t::gsbase"]
        [::std::mem::offset_of!(sgx_pal_gpr_t, gsbase) - 176usize];
};
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct sgx_cpu_context_t {
    pub rax: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rbx: u64,
    pub rsp: u64,
    pub rbp: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rflags: u64,
    pub rip: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of sgx_cpu_context_t"][::std::mem::size_of::<sgx_cpu_context_t>() - 144usize];
    ["Alignment of sgx_cpu_context_t"][::std::mem::align_of::<sgx_cpu_context_t>() - 1usize];
    ["Offset of field: sgx_cpu_context_t::rax"]
        [::std::mem::offset_of!(sgx_cpu_context_t, rax) - 0usize];
    ["Offset of field: sgx_cpu_context_t::rcx"]
        [::std::mem::offset_of!(sgx_cpu_context_t, rcx) - 8usize];
    ["Offset of field: sgx_cpu_context_t::rdx"]
        [::std::mem::offset_of!(sgx_cpu_context_t, rdx) - 16usize];
    ["Offset of field: sgx_cpu_context_t::rbx"]
        [::std::mem::offset_of!(sgx_cpu_context_t, rbx) - 24usize];
    ["Offset of field: sgx_cpu_context_t::rsp"]
        [::std::mem::offset_of!(sgx_cpu_context_t, rsp) - 32usize];
    ["Offset of field: sgx_cpu_context_t::rbp"]
        [::std::mem::offset_of!(sgx_cpu_context_t, rbp) - 40usize];
    ["Offset of field: sgx_cpu_context_t::rsi"]
        [::std::mem::offset_of!(sgx_cpu_context_t, rsi) - 48usize];
    ["Offset of field: sgx_cpu_context_t::rdi"]
        [::std::mem::offset_of!(sgx_cpu_context_t, rdi) - 56usize];
    ["Offset of field: sgx_cpu_context_t::r8"]
        [::std::mem::offset_of!(sgx_cpu_context_t, r8) - 64usize];
    ["Offset of field: sgx_cpu_context_t::r9"]
        [::std::mem::offset_of!(sgx_cpu_context_t, r9) - 72usize];
    ["Offset of field: sgx_cpu_context_t::r10"]
        [::std::mem::offset_of!(sgx_cpu_context_t, r10) - 80usize];
    ["Offset of field: sgx_cpu_context_t::r11"]
        [::std::mem::offset_of!(sgx_cpu_context_t, r11) - 88usize];
    ["Offset of field: sgx_cpu_context_t::r12"]
        [::std::mem::offset_of!(sgx_cpu_context_t, r12) - 96usize];
    ["Offset of field: sgx_cpu_context_t::r13"]
        [::std::mem::offset_of!(sgx_cpu_context_t, r13) - 104usize];
    ["Offset of field: sgx_cpu_context_t::r14"]
        [::std::mem::offset_of!(sgx_cpu_context_t, r14) - 112usize];
    ["Offset of field: sgx_cpu_context_t::r15"]
        [::std::mem::offset_of!(sgx_cpu_context_t, r15) - 120usize];
    ["Offset of field: sgx_cpu_context_t::rflags"]
        [::std::mem::offset_of!(sgx_cpu_context_t, rflags) - 128usize];
    ["Offset of field: sgx_cpu_context_t::rip"]
        [::std::mem::offset_of!(sgx_cpu_context_t, rip) - 136usize];
};
pub const sgx_arch_exception_vector_SGX_EXCEPTION_VECTOR_DE: sgx_arch_exception_vector = 0;
pub const sgx_arch_exception_vector_SGX_EXCEPTION_VECTOR_DB: sgx_arch_exception_vector = 1;
pub const sgx_arch_exception_vector_SGX_EXCEPTION_VECTOR_BP: sgx_arch_exception_vector = 3;
pub const sgx_arch_exception_vector_SGX_EXCEPTION_VECTOR_BR: sgx_arch_exception_vector = 5;
pub const sgx_arch_exception_vector_SGX_EXCEPTION_VECTOR_UD: sgx_arch_exception_vector = 6;
pub const sgx_arch_exception_vector_SGX_EXCEPTION_VECTOR_GP: sgx_arch_exception_vector = 13;
pub const sgx_arch_exception_vector_SGX_EXCEPTION_VECTOR_PF: sgx_arch_exception_vector = 14;
pub const sgx_arch_exception_vector_SGX_EXCEPTION_VECTOR_MF: sgx_arch_exception_vector = 16;
pub const sgx_arch_exception_vector_SGX_EXCEPTION_VECTOR_AC: sgx_arch_exception_vector = 17;
pub const sgx_arch_exception_vector_SGX_EXCEPTION_VECTOR_XM: sgx_arch_exception_vector = 19;
pub const sgx_arch_exception_vector_SGX_EXCEPTION_VECTOR_CP: sgx_arch_exception_vector = 21;
pub type sgx_arch_exception_vector = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sgx_arch_exit_info_t {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of sgx_arch_exit_info_t"][::std::mem::size_of::<sgx_arch_exit_info_t>() - 4usize];
    ["Alignment of sgx_arch_exit_info_t"][::std::mem::align_of::<sgx_arch_exit_info_t>() - 1usize];
};
impl sgx_arch_exit_info_t {
    #[inline]
    pub fn vector(&self) -> sgx_arch_exception_vector {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_vector(&mut self, val: sgx_arch_exception_vector) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn exit_type(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 3u8) as u32) }
    }
    #[inline]
    pub fn set_exit_type(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(11usize, 20u8) as u32) }
    }
    #[inline]
    pub fn set_reserved(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(11usize, 20u8, val as u64)
        }
    }
    #[inline]
    pub fn valid(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(31usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_valid(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(31usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        vector: sgx_arch_exception_vector,
        exit_type: u32,
        reserved: u32,
        valid: u32,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 8u8, {
            let vector: u32 = unsafe { ::std::mem::transmute(vector) };
            vector as u64
        });
        __bindgen_bitfield_unit.set(8usize, 3u8, {
            let exit_type: u32 = unsafe { ::std::mem::transmute(exit_type) };
            exit_type as u64
        });
        __bindgen_bitfield_unit.set(11usize, 20u8, {
            let reserved: u32 = unsafe { ::std::mem::transmute(reserved) };
            reserved as u64
        });
        __bindgen_bitfield_unit.set(31usize, 1u8, {
            let valid: u32 = unsafe { ::std::mem::transmute(valid) };
            valid as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct sgx_arch_exinfo_t {
    pub maddr: u64,
    pub __bindgen_anon_1: sgx_arch_exinfo_t__bindgen_ty_1,
    pub reserved: u32,
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union sgx_arch_exinfo_t__bindgen_ty_1 {
    pub errcd: sgx_arch_exinfo_t__bindgen_ty_1__bindgen_ty_1,
    pub error_code_val: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sgx_arch_exinfo_t__bindgen_ty_1__bindgen_ty_1 {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of sgx_arch_exinfo_t__bindgen_ty_1__bindgen_ty_1"]
        [::std::mem::size_of::<sgx_arch_exinfo_t__bindgen_ty_1__bindgen_ty_1>() - 4usize];
    ["Alignment of sgx_arch_exinfo_t__bindgen_ty_1__bindgen_ty_1"]
        [::std::mem::align_of::<sgx_arch_exinfo_t__bindgen_ty_1__bindgen_ty_1>() - 1usize];
};
impl sgx_arch_exinfo_t__bindgen_ty_1__bindgen_ty_1 {
    #[inline]
    pub fn p(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_p(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn w(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_w(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn u(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_u(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn rsvd(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_rsvd(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn i(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_i(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn pk(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_pk(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved1(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(6usize, 9u8) as u32) }
    }
    #[inline]
    pub fn set_reserved1(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(6usize, 9u8, val as u64)
        }
    }
    #[inline]
    pub fn sgx(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(15usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_sgx(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(15usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved2(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(16usize, 16u8) as u32) }
    }
    #[inline]
    pub fn set_reserved2(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        p: u32,
        w: u32,
        u: u32,
        rsvd: u32,
        i: u32,
        pk: u32,
        reserved1: u32,
        sgx: u32,
        reserved2: u32,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let p: u32 = unsafe { ::std::mem::transmute(p) };
            p as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let w: u32 = unsafe { ::std::mem::transmute(w) };
            w as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let u: u32 = unsafe { ::std::mem::transmute(u) };
            u as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let rsvd: u32 = unsafe { ::std::mem::transmute(rsvd) };
            rsvd as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let i: u32 = unsafe { ::std::mem::transmute(i) };
            i as u64
        });
        __bindgen_bitfield_unit.set(5usize, 1u8, {
            let pk: u32 = unsafe { ::std::mem::transmute(pk) };
            pk as u64
        });
        __bindgen_bitfield_unit.set(6usize, 9u8, {
            let reserved1: u32 = unsafe { ::std::mem::transmute(reserved1) };
            reserved1 as u64
        });
        __bindgen_bitfield_unit.set(15usize, 1u8, {
            let sgx: u32 = unsafe { ::std::mem::transmute(sgx) };
            sgx as u64
        });
        __bindgen_bitfield_unit.set(16usize, 16u8, {
            let reserved2: u32 = unsafe { ::std::mem::transmute(reserved2) };
            reserved2 as u64
        });
        __bindgen_bitfield_unit
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of sgx_arch_exinfo_t__bindgen_ty_1"]
        [::std::mem::size_of::<sgx_arch_exinfo_t__bindgen_ty_1>() - 4usize];
    ["Alignment of sgx_arch_exinfo_t__bindgen_ty_1"]
        [::std::mem::align_of::<sgx_arch_exinfo_t__bindgen_ty_1>() - 1usize];
    ["Offset of field: sgx_arch_exinfo_t__bindgen_ty_1::errcd"]
        [::std::mem::offset_of!(sgx_arch_exinfo_t__bindgen_ty_1, errcd) - 0usize];
    ["Offset of field: sgx_arch_exinfo_t__bindgen_ty_1::error_code_val"]
        [::std::mem::offset_of!(sgx_arch_exinfo_t__bindgen_ty_1, error_code_val) - 0usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of sgx_arch_exinfo_t"][::std::mem::size_of::<sgx_arch_exinfo_t>() - 16usize];
    ["Alignment of sgx_arch_exinfo_t"][::std::mem::align_of::<sgx_arch_exinfo_t>() - 1usize];
    ["Offset of field: sgx_arch_exinfo_t::maddr"]
        [::std::mem::offset_of!(sgx_arch_exinfo_t, maddr) - 0usize];
    ["Offset of field: sgx_arch_exinfo_t::reserved"]
        [::std::mem::offset_of!(sgx_arch_exinfo_t, reserved) - 12usize];
};
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct sgx_arch_page_info_t {
    pub lin_addr: u64,
    pub src_pge: u64,
    pub sec_info: u64,
    pub secs: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of sgx_arch_page_info_t"][::std::mem::size_of::<sgx_arch_page_info_t>() - 32usize];
    ["Alignment of sgx_arch_page_info_t"][::std::mem::align_of::<sgx_arch_page_info_t>() - 1usize];
    ["Offset of field: sgx_arch_page_info_t::lin_addr"]
        [::std::mem::offset_of!(sgx_arch_page_info_t, lin_addr) - 0usize];
    ["Offset of field: sgx_arch_page_info_t::src_pge"]
        [::std::mem::offset_of!(sgx_arch_page_info_t, src_pge) - 8usize];
    ["Offset of field: sgx_arch_page_info_t::sec_info"]
        [::std::mem::offset_of!(sgx_arch_page_info_t, sec_info) - 16usize];
    ["Offset of field: sgx_arch_page_info_t::secs"]
        [::std::mem::offset_of!(sgx_arch_page_info_t, secs) - 24usize];
};
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct sgx_arch_sec_info_t {
    pub flags: u64,
    pub reserved: [u64; 7usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of sgx_arch_sec_info_t"][::std::mem::size_of::<sgx_arch_sec_info_t>() - 64usize];
    ["Alignment of sgx_arch_sec_info_t"][::std::mem::align_of::<sgx_arch_sec_info_t>() - 1usize];
    ["Offset of field: sgx_arch_sec_info_t::flags"]
        [::std::mem::offset_of!(sgx_arch_sec_info_t, flags) - 0usize];
    ["Offset of field: sgx_arch_sec_info_t::reserved"]
        [::std::mem::offset_of!(sgx_arch_sec_info_t, reserved) - 8usize];
};
pub const sgx_page_type_SGX_PAGE_TYPE_SECS: sgx_page_type = 0;
pub const sgx_page_type_SGX_PAGE_TYPE_TCS: sgx_page_type = 1;
pub const sgx_page_type_SGX_PAGE_TYPE_REG: sgx_page_type = 2;
pub const sgx_page_type_SGX_PAGE_TYPE_VA: sgx_page_type = 3;
pub const sgx_page_type_SGX_PAGE_TYPE_TRIM: sgx_page_type = 4;
pub type sgx_page_type = ::std::os::raw::c_uint;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct sgx_sigstruct_t {
    pub header: [u8; 16usize],
    pub vendor: u32,
    pub date: u32,
    pub header2: [u8; 16usize],
    pub swdefined: u32,
    pub reserved1: [u8; 84usize],
    pub modulus: [u8; 384usize],
    pub exponent: [u8; 4usize],
    pub signature: [u8; 384usize],
    pub misc_select: sgx_misc_select_t,
    pub misc_mask: sgx_misc_select_t,
    pub cet_attributes: sgx_cet_attributes_t,
    pub cet_attributes_mask: sgx_cet_attributes_t,
    pub reserved2: [u8; 2usize],
    pub isv_family_id: sgx_isvfamily_id_t,
    pub attributes: sgx_attributes_t,
    pub attribute_mask: sgx_attributes_t,
    pub enclave_hash: sgx_measurement_t,
    pub reserved3: [u8; 16usize],
    pub isvext_prod_id: sgx_isvext_prod_id_t,
    pub isv_prod_id: sgx_prod_id_t,
    pub isv_svn: sgx_isv_svn_t,
    pub reserved4: [u8; 12usize],
    pub q1: [u8; 384usize],
    pub q2: [u8; 384usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of sgx_sigstruct_t"][::std::mem::size_of::<sgx_sigstruct_t>() - 1808usize];
    ["Alignment of sgx_sigstruct_t"][::std::mem::align_of::<sgx_sigstruct_t>() - 1usize];
    ["Offset of field: sgx_sigstruct_t::header"]
        [::std::mem::offset_of!(sgx_sigstruct_t, header) - 0usize];
    ["Offset of field: sgx_sigstruct_t::vendor"]
        [::std::mem::offset_of!(sgx_sigstruct_t, vendor) - 16usize];
    ["Offset of field: sgx_sigstruct_t::date"]
        [::std::mem::offset_of!(sgx_sigstruct_t, date) - 20usize];
    ["Offset of field: sgx_sigstruct_t::header2"]
        [::std::mem::offset_of!(sgx_sigstruct_t, header2) - 24usize];
    ["Offset of field: sgx_sigstruct_t::swdefined"]
        [::std::mem::offset_of!(sgx_sigstruct_t, swdefined) - 40usize];
    ["Offset of field: sgx_sigstruct_t::reserved1"]
        [::std::mem::offset_of!(sgx_sigstruct_t, reserved1) - 44usize];
    ["Offset of field: sgx_sigstruct_t::modulus"]
        [::std::mem::offset_of!(sgx_sigstruct_t, modulus) - 128usize];
    ["Offset of field: sgx_sigstruct_t::exponent"]
        [::std::mem::offset_of!(sgx_sigstruct_t, exponent) - 512usize];
    ["Offset of field: sgx_sigstruct_t::signature"]
        [::std::mem::offset_of!(sgx_sigstruct_t, signature) - 516usize];
    ["Offset of field: sgx_sigstruct_t::misc_select"]
        [::std::mem::offset_of!(sgx_sigstruct_t, misc_select) - 900usize];
    ["Offset of field: sgx_sigstruct_t::misc_mask"]
        [::std::mem::offset_of!(sgx_sigstruct_t, misc_mask) - 904usize];
    ["Offset of field: sgx_sigstruct_t::cet_attributes"]
        [::std::mem::offset_of!(sgx_sigstruct_t, cet_attributes) - 908usize];
    ["Offset of field: sgx_sigstruct_t::cet_attributes_mask"]
        [::std::mem::offset_of!(sgx_sigstruct_t, cet_attributes_mask) - 909usize];
    ["Offset of field: sgx_sigstruct_t::reserved2"]
        [::std::mem::offset_of!(sgx_sigstruct_t, reserved2) - 910usize];
    ["Offset of field: sgx_sigstruct_t::isv_family_id"]
        [::std::mem::offset_of!(sgx_sigstruct_t, isv_family_id) - 912usize];
    ["Offset of field: sgx_sigstruct_t::attributes"]
        [::std::mem::offset_of!(sgx_sigstruct_t, attributes) - 928usize];
    ["Offset of field: sgx_sigstruct_t::attribute_mask"]
        [::std::mem::offset_of!(sgx_sigstruct_t, attribute_mask) - 944usize];
    ["Offset of field: sgx_sigstruct_t::enclave_hash"]
        [::std::mem::offset_of!(sgx_sigstruct_t, enclave_hash) - 960usize];
    ["Offset of field: sgx_sigstruct_t::reserved3"]
        [::std::mem::offset_of!(sgx_sigstruct_t, reserved3) - 992usize];
    ["Offset of field: sgx_sigstruct_t::isvext_prod_id"]
        [::std::mem::offset_of!(sgx_sigstruct_t, isvext_prod_id) - 1008usize];
    ["Offset of field: sgx_sigstruct_t::isv_prod_id"]
        [::std::mem::offset_of!(sgx_sigstruct_t, isv_prod_id) - 1024usize];
    ["Offset of field: sgx_sigstruct_t::isv_svn"]
        [::std::mem::offset_of!(sgx_sigstruct_t, isv_svn) - 1026usize];
    ["Offset of field: sgx_sigstruct_t::reserved4"]
        [::std::mem::offset_of!(sgx_sigstruct_t, reserved4) - 1028usize];
    ["Offset of field: sgx_sigstruct_t::q1"]
        [::std::mem::offset_of!(sgx_sigstruct_t, q1) - 1040usize];
    ["Offset of field: sgx_sigstruct_t::q2"]
        [::std::mem::offset_of!(sgx_sigstruct_t, q2) - 1424usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _sgx_key_id_t {
    pub id: [u8; 32usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _sgx_key_id_t"][::std::mem::size_of::<_sgx_key_id_t>() - 32usize];
    ["Alignment of _sgx_key_id_t"][::std::mem::align_of::<_sgx_key_id_t>() - 1usize];
    ["Offset of field: _sgx_key_id_t::id"][::std::mem::offset_of!(_sgx_key_id_t, id) - 0usize];
};
pub type sgx_key_id_t = _sgx_key_id_t;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct launch_body_t {
    pub valid: u32,
    pub reserved1: [u32; 11usize],
    pub attributes: sgx_attributes_t,
    pub mr_enclave: sgx_measurement_t,
    pub reserved2: [u8; 32usize],
    pub mr_signer: sgx_measurement_t,
    pub reserved3: [u8; 32usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of launch_body_t"][::std::mem::size_of::<launch_body_t>() - 192usize];
    ["Alignment of launch_body_t"][::std::mem::align_of::<launch_body_t>() - 1usize];
    ["Offset of field: launch_body_t::valid"]
        [::std::mem::offset_of!(launch_body_t, valid) - 0usize];
    ["Offset of field: launch_body_t::reserved1"]
        [::std::mem::offset_of!(launch_body_t, reserved1) - 4usize];
    ["Offset of field: launch_body_t::attributes"]
        [::std::mem::offset_of!(launch_body_t, attributes) - 48usize];
    ["Offset of field: launch_body_t::mr_enclave"]
        [::std::mem::offset_of!(launch_body_t, mr_enclave) - 64usize];
    ["Offset of field: launch_body_t::reserved2"]
        [::std::mem::offset_of!(launch_body_t, reserved2) - 96usize];
    ["Offset of field: launch_body_t::mr_signer"]
        [::std::mem::offset_of!(launch_body_t, mr_signer) - 128usize];
    ["Offset of field: launch_body_t::reserved3"]
        [::std::mem::offset_of!(launch_body_t, reserved3) - 160usize];
};
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct sgx_arch_token_t {
    pub body: launch_body_t,
    pub cpu_svn_le: sgx_cpu_svn_t,
    pub isv_prod_id_le: sgx_prod_id_t,
    pub isv_svn_le: sgx_isv_svn_t,
    pub reserved2: [u8; 24usize],
    pub masked_misc_select_le: sgx_misc_select_t,
    pub attributes_le: sgx_attributes_t,
    pub key_id: sgx_key_id_t,
    pub mac: sgx_mac_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of sgx_arch_token_t"][::std::mem::size_of::<sgx_arch_token_t>() - 304usize];
    ["Alignment of sgx_arch_token_t"][::std::mem::align_of::<sgx_arch_token_t>() - 1usize];
    ["Offset of field: sgx_arch_token_t::body"]
        [::std::mem::offset_of!(sgx_arch_token_t, body) - 0usize];
    ["Offset of field: sgx_arch_token_t::cpu_svn_le"]
        [::std::mem::offset_of!(sgx_arch_token_t, cpu_svn_le) - 192usize];
    ["Offset of field: sgx_arch_token_t::isv_prod_id_le"]
        [::std::mem::offset_of!(sgx_arch_token_t, isv_prod_id_le) - 208usize];
    ["Offset of field: sgx_arch_token_t::isv_svn_le"]
        [::std::mem::offset_of!(sgx_arch_token_t, isv_svn_le) - 210usize];
    ["Offset of field: sgx_arch_token_t::reserved2"]
        [::std::mem::offset_of!(sgx_arch_token_t, reserved2) - 212usize];
    ["Offset of field: sgx_arch_token_t::masked_misc_select_le"]
        [::std::mem::offset_of!(sgx_arch_token_t, masked_misc_select_le) - 236usize];
    ["Offset of field: sgx_arch_token_t::attributes_le"]
        [::std::mem::offset_of!(sgx_arch_token_t, attributes_le) - 240usize];
    ["Offset of field: sgx_arch_token_t::key_id"]
        [::std::mem::offset_of!(sgx_arch_token_t, key_id) - 256usize];
    ["Offset of field: sgx_arch_token_t::mac"]
        [::std::mem::offset_of!(sgx_arch_token_t, mac) - 288usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _sgx_report_data_t {
    pub d: [u8; 64usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _sgx_report_data_t"][::std::mem::size_of::<_sgx_report_data_t>() - 64usize];
    ["Alignment of _sgx_report_data_t"][::std::mem::align_of::<_sgx_report_data_t>() - 1usize];
    ["Offset of field: _sgx_report_data_t::d"]
        [::std::mem::offset_of!(_sgx_report_data_t, d) - 0usize];
};
pub type sgx_report_data_t = _sgx_report_data_t;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct _report_body_t {
    pub cpu_svn: sgx_cpu_svn_t,
    pub misc_select: sgx_misc_select_t,
    pub cet_attributes: sgx_cet_attributes_t,
    pub reserved1: [u8; 11usize],
    pub isv_ext_prod_id: sgx_isvext_prod_id_t,
    pub attributes: sgx_attributes_t,
    pub mr_enclave: sgx_measurement_t,
    pub reserved2: [u8; 32usize],
    pub mr_signer: sgx_measurement_t,
    pub reserved3: [u8; 32usize],
    pub config_id: sgx_config_id_t,
    pub isv_prod_id: sgx_prod_id_t,
    pub isv_svn: sgx_isv_svn_t,
    pub config_svn: sgx_config_svn_t,
    pub reserved4: [u8; 42usize],
    pub isv_family_id: sgx_isvfamily_id_t,
    pub report_data: sgx_report_data_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _report_body_t"][::std::mem::size_of::<_report_body_t>() - 384usize];
    ["Alignment of _report_body_t"][::std::mem::align_of::<_report_body_t>() - 1usize];
    ["Offset of field: _report_body_t::cpu_svn"]
        [::std::mem::offset_of!(_report_body_t, cpu_svn) - 0usize];
    ["Offset of field: _report_body_t::misc_select"]
        [::std::mem::offset_of!(_report_body_t, misc_select) - 16usize];
    ["Offset of field: _report_body_t::cet_attributes"]
        [::std::mem::offset_of!(_report_body_t, cet_attributes) - 20usize];
    ["Offset of field: _report_body_t::reserved1"]
        [::std::mem::offset_of!(_report_body_t, reserved1) - 21usize];
    ["Offset of field: _report_body_t::isv_ext_prod_id"]
        [::std::mem::offset_of!(_report_body_t, isv_ext_prod_id) - 32usize];
    ["Offset of field: _report_body_t::attributes"]
        [::std::mem::offset_of!(_report_body_t, attributes) - 48usize];
    ["Offset of field: _report_body_t::mr_enclave"]
        [::std::mem::offset_of!(_report_body_t, mr_enclave) - 64usize];
    ["Offset of field: _report_body_t::reserved2"]
        [::std::mem::offset_of!(_report_body_t, reserved2) - 96usize];
    ["Offset of field: _report_body_t::mr_signer"]
        [::std::mem::offset_of!(_report_body_t, mr_signer) - 128usize];
    ["Offset of field: _report_body_t::reserved3"]
        [::std::mem::offset_of!(_report_body_t, reserved3) - 160usize];
    ["Offset of field: _report_body_t::config_id"]
        [::std::mem::offset_of!(_report_body_t, config_id) - 192usize];
    ["Offset of field: _report_body_t::isv_prod_id"]
        [::std::mem::offset_of!(_report_body_t, isv_prod_id) - 256usize];
    ["Offset of field: _report_body_t::isv_svn"]
        [::std::mem::offset_of!(_report_body_t, isv_svn) - 258usize];
    ["Offset of field: _report_body_t::config_svn"]
        [::std::mem::offset_of!(_report_body_t, config_svn) - 260usize];
    ["Offset of field: _report_body_t::reserved4"]
        [::std::mem::offset_of!(_report_body_t, reserved4) - 262usize];
    ["Offset of field: _report_body_t::isv_family_id"]
        [::std::mem::offset_of!(_report_body_t, isv_family_id) - 304usize];
    ["Offset of field: _report_body_t::report_data"]
        [::std::mem::offset_of!(_report_body_t, report_data) - 320usize];
};
pub type sgx_report_body_t = _report_body_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _report_t {
    pub body: sgx_report_body_t,
    pub key_id: sgx_key_id_t,
    pub mac: sgx_mac_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _report_t"][::std::mem::size_of::<_report_t>() - 432usize];
    ["Alignment of _report_t"][::std::mem::align_of::<_report_t>() - 1usize];
    ["Offset of field: _report_t::body"][::std::mem::offset_of!(_report_t, body) - 0usize];
    ["Offset of field: _report_t::key_id"][::std::mem::offset_of!(_report_t, key_id) - 384usize];
    ["Offset of field: _report_t::mac"][::std::mem::offset_of!(_report_t, mac) - 416usize];
};
pub type sgx_report_t = _report_t;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct sgx_target_info_t {
    pub mr_enclave: sgx_measurement_t,
    pub attributes: sgx_attributes_t,
    pub cet_attributes: sgx_cet_attributes_t,
    pub reserved1: u8,
    pub config_svn: sgx_config_svn_t,
    pub misc_select: sgx_misc_select_t,
    pub reserved2: [u8; 8usize],
    pub config_id: sgx_config_id_t,
    pub reserved3: [u8; 384usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of sgx_target_info_t"][::std::mem::size_of::<sgx_target_info_t>() - 512usize];
    ["Alignment of sgx_target_info_t"][::std::mem::align_of::<sgx_target_info_t>() - 1usize];
    ["Offset of field: sgx_target_info_t::mr_enclave"]
        [::std::mem::offset_of!(sgx_target_info_t, mr_enclave) - 0usize];
    ["Offset of field: sgx_target_info_t::attributes"]
        [::std::mem::offset_of!(sgx_target_info_t, attributes) - 32usize];
    ["Offset of field: sgx_target_info_t::cet_attributes"]
        [::std::mem::offset_of!(sgx_target_info_t, cet_attributes) - 48usize];
    ["Offset of field: sgx_target_info_t::reserved1"]
        [::std::mem::offset_of!(sgx_target_info_t, reserved1) - 49usize];
    ["Offset of field: sgx_target_info_t::config_svn"]
        [::std::mem::offset_of!(sgx_target_info_t, config_svn) - 50usize];
    ["Offset of field: sgx_target_info_t::misc_select"]
        [::std::mem::offset_of!(sgx_target_info_t, misc_select) - 52usize];
    ["Offset of field: sgx_target_info_t::reserved2"]
        [::std::mem::offset_of!(sgx_target_info_t, reserved2) - 56usize];
    ["Offset of field: sgx_target_info_t::config_id"]
        [::std::mem::offset_of!(sgx_target_info_t, config_id) - 64usize];
    ["Offset of field: sgx_target_info_t::reserved3"]
        [::std::mem::offset_of!(sgx_target_info_t, reserved3) - 128usize];
};
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct _key_request_t {
    pub key_name: u16,
    pub key_policy: u16,
    pub isv_svn: sgx_isv_svn_t,
    pub reserved1: u16,
    pub cpu_svn: sgx_cpu_svn_t,
    pub attribute_mask: sgx_attributes_t,
    pub key_id: sgx_key_id_t,
    pub misc_mask: sgx_misc_select_t,
    pub config_svn: sgx_config_svn_t,
    pub reserved2: [u8; 434usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _key_request_t"][::std::mem::size_of::<_key_request_t>() - 512usize];
    ["Alignment of _key_request_t"][::std::mem::align_of::<_key_request_t>() - 1usize];
    ["Offset of field: _key_request_t::key_name"]
        [::std::mem::offset_of!(_key_request_t, key_name) - 0usize];
    ["Offset of field: _key_request_t::key_policy"]
        [::std::mem::offset_of!(_key_request_t, key_policy) - 2usize];
    ["Offset of field: _key_request_t::isv_svn"]
        [::std::mem::offset_of!(_key_request_t, isv_svn) - 4usize];
    ["Offset of field: _key_request_t::reserved1"]
        [::std::mem::offset_of!(_key_request_t, reserved1) - 6usize];
    ["Offset of field: _key_request_t::cpu_svn"]
        [::std::mem::offset_of!(_key_request_t, cpu_svn) - 8usize];
    ["Offset of field: _key_request_t::attribute_mask"]
        [::std::mem::offset_of!(_key_request_t, attribute_mask) - 24usize];
    ["Offset of field: _key_request_t::key_id"]
        [::std::mem::offset_of!(_key_request_t, key_id) - 40usize];
    ["Offset of field: _key_request_t::misc_mask"]
        [::std::mem::offset_of!(_key_request_t, misc_mask) - 72usize];
    ["Offset of field: _key_request_t::config_svn"]
        [::std::mem::offset_of!(_key_request_t, config_svn) - 76usize];
    ["Offset of field: _key_request_t::reserved2"]
        [::std::mem::offset_of!(_key_request_t, reserved2) - 78usize];
};
pub type sgx_key_request_t = _key_request_t;
pub type sgx_key_128bit_t = [u8; 16usize];
pub const sgx_ql_attestation_algorithm_id_t_SGX_QL_ALG_EPID: sgx_ql_attestation_algorithm_id_t = 0;
pub const sgx_ql_attestation_algorithm_id_t_SGX_QL_ALG_RESERVED_1:
    sgx_ql_attestation_algorithm_id_t = 1;
pub const sgx_ql_attestation_algorithm_id_t_SGX_QL_ALG_ECDSA_P256:
    sgx_ql_attestation_algorithm_id_t = 2;
pub const sgx_ql_attestation_algorithm_id_t_SGX_QL_ALG_ECDSA_P384:
    sgx_ql_attestation_algorithm_id_t = 3;
pub const sgx_ql_attestation_algorithm_id_t_SGX_QL_ALG_CNT: sgx_ql_attestation_algorithm_id_t = 4;
pub type sgx_ql_attestation_algorithm_id_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _att_key_id_t {
    pub att_key_id: [u8; 256usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _att_key_id_t"][::std::mem::size_of::<_att_key_id_t>() - 256usize];
    ["Alignment of _att_key_id_t"][::std::mem::align_of::<_att_key_id_t>() - 1usize];
    ["Offset of field: _att_key_id_t::att_key_id"]
        [::std::mem::offset_of!(_att_key_id_t, att_key_id) - 0usize];
};
pub type sgx_att_key_id_t = _att_key_id_t;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct _sgx_ql_att_key_id_t {
    pub id: u16,
    pub version: u16,
    pub mrsigner_length: u16,
    pub mrsigner: [u8; 48usize],
    pub prod_id: u32,
    pub extended_prod_id: [u8; 16usize],
    pub config_id: [u8; 64usize],
    pub family_id: [u8; 16usize],
    pub algorithm_id: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _sgx_ql_att_key_id_t"][::std::mem::size_of::<_sgx_ql_att_key_id_t>() - 158usize];
    ["Alignment of _sgx_ql_att_key_id_t"][::std::mem::align_of::<_sgx_ql_att_key_id_t>() - 1usize];
    ["Offset of field: _sgx_ql_att_key_id_t::id"]
        [::std::mem::offset_of!(_sgx_ql_att_key_id_t, id) - 0usize];
    ["Offset of field: _sgx_ql_att_key_id_t::version"]
        [::std::mem::offset_of!(_sgx_ql_att_key_id_t, version) - 2usize];
    ["Offset of field: _sgx_ql_att_key_id_t::mrsigner_length"]
        [::std::mem::offset_of!(_sgx_ql_att_key_id_t, mrsigner_length) - 4usize];
    ["Offset of field: _sgx_ql_att_key_id_t::mrsigner"]
        [::std::mem::offset_of!(_sgx_ql_att_key_id_t, mrsigner) - 6usize];
    ["Offset of field: _sgx_ql_att_key_id_t::prod_id"]
        [::std::mem::offset_of!(_sgx_ql_att_key_id_t, prod_id) - 54usize];
    ["Offset of field: _sgx_ql_att_key_id_t::extended_prod_id"]
        [::std::mem::offset_of!(_sgx_ql_att_key_id_t, extended_prod_id) - 58usize];
    ["Offset of field: _sgx_ql_att_key_id_t::config_id"]
        [::std::mem::offset_of!(_sgx_ql_att_key_id_t, config_id) - 74usize];
    ["Offset of field: _sgx_ql_att_key_id_t::family_id"]
        [::std::mem::offset_of!(_sgx_ql_att_key_id_t, family_id) - 138usize];
    ["Offset of field: _sgx_ql_att_key_id_t::algorithm_id"]
        [::std::mem::offset_of!(_sgx_ql_att_key_id_t, algorithm_id) - 154usize];
};
pub type sgx_ql_att_key_id_t = _sgx_ql_att_key_id_t;
pub type sgx_epid_group_id_t = [u8; 4usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _sgx_basename_t {
    pub name: [u8; 32usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _sgx_basename_t"][::std::mem::size_of::<_sgx_basename_t>() - 32usize];
    ["Alignment of _sgx_basename_t"][::std::mem::align_of::<_sgx_basename_t>() - 1usize];
    ["Offset of field: _sgx_basename_t::name"]
        [::std::mem::offset_of!(_sgx_basename_t, name) - 0usize];
};
pub type sgx_basename_t = _sgx_basename_t;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct _sgx_quote_body_t {
    pub version: u16,
    pub sign_type: u16,
    pub epid_group_id: sgx_epid_group_id_t,
    pub qe_svn: sgx_isv_svn_t,
    pub pce_svn: sgx_isv_svn_t,
    pub xeid: u32,
    pub basename: sgx_basename_t,
    pub report_body: sgx_report_body_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _sgx_quote_body_t"][::std::mem::size_of::<_sgx_quote_body_t>() - 432usize];
    ["Alignment of _sgx_quote_body_t"][::std::mem::align_of::<_sgx_quote_body_t>() - 1usize];
    ["Offset of field: _sgx_quote_body_t::version"]
        [::std::mem::offset_of!(_sgx_quote_body_t, version) - 0usize];
    ["Offset of field: _sgx_quote_body_t::sign_type"]
        [::std::mem::offset_of!(_sgx_quote_body_t, sign_type) - 2usize];
    ["Offset of field: _sgx_quote_body_t::epid_group_id"]
        [::std::mem::offset_of!(_sgx_quote_body_t, epid_group_id) - 4usize];
    ["Offset of field: _sgx_quote_body_t::qe_svn"]
        [::std::mem::offset_of!(_sgx_quote_body_t, qe_svn) - 8usize];
    ["Offset of field: _sgx_quote_body_t::pce_svn"]
        [::std::mem::offset_of!(_sgx_quote_body_t, pce_svn) - 10usize];
    ["Offset of field: _sgx_quote_body_t::xeid"]
        [::std::mem::offset_of!(_sgx_quote_body_t, xeid) - 12usize];
    ["Offset of field: _sgx_quote_body_t::basename"]
        [::std::mem::offset_of!(_sgx_quote_body_t, basename) - 16usize];
    ["Offset of field: _sgx_quote_body_t::report_body"]
        [::std::mem::offset_of!(_sgx_quote_body_t, report_body) - 48usize];
};
pub type sgx_quote_body_t = _sgx_quote_body_t;
#[repr(C, packed)]
pub struct _sgx_quote_t {
    pub body: sgx_quote_body_t,
    pub signature_size: u32,
    pub signature: __IncompleteArrayField<u8>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _sgx_quote_t"][::std::mem::size_of::<_sgx_quote_t>() - 436usize];
    ["Alignment of _sgx_quote_t"][::std::mem::align_of::<_sgx_quote_t>() - 1usize];
    ["Offset of field: _sgx_quote_t::body"][::std::mem::offset_of!(_sgx_quote_t, body) - 0usize];
    ["Offset of field: _sgx_quote_t::signature_size"]
        [::std::mem::offset_of!(_sgx_quote_t, signature_size) - 432usize];
    ["Offset of field: _sgx_quote_t::signature"]
        [::std::mem::offset_of!(_sgx_quote_t, signature) - 436usize];
};
pub type sgx_quote_t = _sgx_quote_t;
pub type sgx_spid_t = [u8; 16usize];
pub type sgx_quote_nonce_t = [u8; 16usize];
pub const SGX_UNLINKABLE_SIGNATURE: _bindgen_ty_1 = 0;
pub const SGX_LINKABLE_SIGNATURE: _bindgen_ty_1 = 1;
pub type _bindgen_ty_1 = ::std::os::raw::c_uint;
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __va_list_tag"][::std::mem::size_of::<__va_list_tag>() - 24usize];
    ["Alignment of __va_list_tag"][::std::mem::align_of::<__va_list_tag>() - 8usize];
    ["Offset of field: __va_list_tag::gp_offset"]
        [::std::mem::offset_of!(__va_list_tag, gp_offset) - 0usize];
    ["Offset of field: __va_list_tag::fp_offset"]
        [::std::mem::offset_of!(__va_list_tag, fp_offset) - 4usize];
    ["Offset of field: __va_list_tag::overflow_arg_area"]
        [::std::mem::offset_of!(__va_list_tag, overflow_arg_area) - 8usize];
    ["Offset of field: __va_list_tag::reg_save_area"]
        [::std::mem::offset_of!(__va_list_tag, reg_save_area) - 16usize];
};
