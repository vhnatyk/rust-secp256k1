#![allow(non_camel_case_types)]
use core::fmt;

pub type c_int = i32;
pub type c_uchar = u8;
pub type c_uint = u32;

/// This is an exact copy of https://doc.rust-lang.org/src/std/os/raw/mod.rs.html
/// `c_char` might be either signed or unsigned, defined by the platform,
/// This should match the right type using conditional compilations from stdlib.
#[cfg(any(all(target_os = "linux", any(target_arch = "aarch64",
                                       target_arch = "arm",
                                       target_arch = "powerpc",
                                       target_arch = "powerpc64",
                                       target_arch = "s390x")),
          all(target_os = "android", any(target_arch = "aarch64",
                                         target_arch = "arm")),
          all(target_os = "l4re", target_arch = "x86_64"),
          all(target_os = "freebsd", any(target_arch = "aarch64",
                                         target_arch = "arm",
                                         target_arch = "powerpc",
                                         target_arch = "powerpc64")),
          all(target_os = "netbsd", any(target_arch = "aarch64",
                                        target_arch = "arm",
                                        target_arch = "powerpc")),
          all(target_os = "openbsd", target_arch = "aarch64"),
          all(target_os = "fuchsia", target_arch = "aarch64")))]
pub type c_char = u8;
#[cfg(not(any(all(target_os = "linux", any(target_arch = "aarch64",
                                           target_arch = "arm",
                                           target_arch = "powerpc",
                                           target_arch = "powerpc64",
                                           target_arch = "s390x")),
              all(target_os = "android", any(target_arch = "aarch64",
                                             target_arch = "arm")),
              all(target_os = "l4re", target_arch = "x86_64"),
              all(target_os = "freebsd", any(target_arch = "aarch64",
                                             target_arch = "arm",
                                             target_arch = "powerpc",
                                             target_arch = "powerpc64")),
              all(target_os = "netbsd", any(target_arch = "aarch64",
                                            target_arch = "arm",
                                            target_arch = "powerpc")),
              all(target_os = "openbsd", target_arch = "aarch64"),
              all(target_os = "fuchsia", target_arch = "aarch64"))))]
pub type c_char = i8;

/// This is an exact copy of https://doc.rust-lang.org/core/ffi/enum.c_void.html
/// It should be Equivalent to C's void type when used as a pointer.
/// 
/// We can replace this with `core::ffi::c_void` once we update the rustc version to >=1.30.0.
#[repr(u8)]
pub enum c_void {
    #[doc(hidden)] __variant1,
    #[doc(hidden)] __variant2,
}

impl fmt::Debug for c_void {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad("c_void")
    }
}