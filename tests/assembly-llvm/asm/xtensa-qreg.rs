//@ add-minicore
//@ assembly-output: emit-asm
//@ compile-flags: --target xtensa-esp32s3-none-elf -Ctarget-feature=-windowed -Zmerge-functions=disabled
//@ min-llvm-version: 23
//@ needs-llvm-components: xtensa

#![feature(no_core, asm_experimental_arch, repr_simd)]
#![crate_type = "rlib"]
#![no_core]

extern crate minicore;
use minicore::simd::*;
use minicore::*;

#[allow(non_camel_case_types)]
#[repr(simd)]
struct u8x16([u8; 16]);
impl Copy for u8x16 {}

// CHECK-LABEL: qr_add:
// CHECK: #APP
// CHECK: ee.vadds.s8 q{{[0-7]}}, q{{[0-7]}}, q{{[0-7]}}
// CHECK: #NO_APP
#[no_mangle]
pub unsafe fn qr_add() {
    let mut a: i8x16;
    let b: i8x16;
    asm!("ee.zero.q {a}", a = out(qreg) a);
    asm!("ee.zero.q {b}", b = out(qreg) b);
    asm!("ee.vadds.s8 {dst}, {dst}, {rhs}", dst = inout(qreg) a, rhs = in(qreg) b);
}

// Exercise the custom `#[repr(simd)]` type shape from issue #265. Keep the
// vectors local so this test does not depend on the currently unsupported
// Xtensa vector function ABI.
// CHECK-LABEL: qr_custom_simd:
// CHECK: #APP
// CHECK: ee.vadds.s8 q{{[0-7]}}, q{{[0-7]}}, q{{[0-7]}}
// CHECK: #NO_APP
#[no_mangle]
pub unsafe fn qr_custom_simd() {
    let mut a: u8x16;
    let b: u8x16;
    asm!("ee.zero.q {a}", a = out(qreg) a);
    asm!("ee.zero.q {b}", b = out(qreg) b);
    asm!("ee.vadds.s8 {dst}, {dst}, {rhs}", dst = inout(qreg) a, rhs = in(qreg) b);
}

// Constant array initializers must be lowered through a constant-pool load
// instead of reaching instruction selection as an unsupported BUILD_VECTOR.
// CHECK-LABEL: qr_custom_simd_literal:
// CHECK: ee.vadds.s8 q{{[0-7]}}, q{{[0-7]}}, q{{[0-7]}}
#[no_mangle]
pub unsafe fn qr_custom_simd_literal() {
    let mut a = u8x16([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    let b: u8x16;
    asm!("ee.zero.q {b}", b = out(qreg) b);
    asm!("ee.vadds.s8 {dst}, {dst}, {rhs}", dst = inout(qreg) a, rhs = in(qreg) b);
}

// The ISA exposes a 32-bit word from a QR register; EXTUI then selects a byte.
// Exercise the raw instruction sequence for reading a 32-bit word and byte.
// CHECK-LABEL: qr_custom_simd_lane0:
// CHECK: ee.movi.32.a q{{[0-7]}}, a{{[0-9]+}}, 0
// CHECK: extui a{{[0-9]+}}, a{{[0-9]+}}, 0, 8
#[no_mangle]
pub unsafe fn qr_custom_simd_lane0() -> u8 {
    let q: u8x16;
    asm!("ee.zero.q {0}", out(qreg) q);
    let word: u32;
    let byte: u32;
    asm!(
        "ee.movi.32.a {q}, {word}, 0",
        "extui {byte}, {word}, 0, 8",
        q = in(qreg) q,
        word = out(reg) word,
        byte = out(reg) byte,
    );
    byte as u8
}

// The ESP32-S3 QR registers are caller-saved and must be part of the C ABI
// clobber set on this target.
// CHECK-LABEL: qr_clobber_abi:
#[no_mangle]
pub unsafe fn qr_clobber_abi() {
    asm!("", clobber_abi("C"));
}
