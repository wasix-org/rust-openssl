use libc::*;

// Even wasm-32 has an I64 type, which openssl uses for BN_ULONG.
#[cfg(any(target_pointer_width = "64", target_family = "wasm"))]
pub type BN_ULONG = c_ulonglong;
#[cfg(all(target_pointer_width = "32", not(target_family = "wasm")))]
pub type BN_ULONG = c_uint;

#[cfg(ossl110)]
pub const BN_FLG_MALLOCED: c_int = 0x01;
#[cfg(ossl110)]
pub const BN_FLG_STATIC_DATA: c_int = 0x02;
#[cfg(ossl110)]
pub const BN_FLG_CONSTTIME: c_int = 0x04;
#[cfg(ossl110)]
pub const BN_FLG_SECURE: c_int = 0x08;
