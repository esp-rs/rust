// min-llvm-version: 10.0.1
// assembly-output: emit-asm
// compile-flags: --target xtensa-esp32-none-elf
// needs-llvm-components: xtensa

#![feature(no_core, lang_items, rustc_attrs, repr_simd)]
#![crate_type = "rlib"]
#![no_core]
#![allow(asm_sub_register, non_camel_case_types)]

#[rustc_builtin_macro]
macro_rules! asm {
    () => {};
}
#[rustc_builtin_macro]
macro_rules! concat {
    () => {};
}
#[rustc_builtin_macro]
macro_rules! stringify {
    () => {};
}

#[lang = "sized"]
trait Sized {}
#[lang = "copy"]
trait Copy {}

type ptr = *const i32;

impl Copy for i8 {}
impl Copy for i16 {}
impl Copy for i32 {}
impl Copy for f32 {}
impl Copy for f64 {}
impl Copy for ptr {}

extern "C" {
    fn extern_func();
    static extern_static: u8;
}

// Hack to avoid function merging
extern "Rust" {
    fn dont_merge(s: &str);
}

// CHECK-LABEL: sym_fn:
// CHECK: #APP
// CHECK: call4 extern_func
// CHECK: #NO_APP
#[no_mangle]
pub unsafe fn sym_fn() {
    asm!("call4 {}", sym extern_func);
}

// CHECK-LABEL: sym_static:
// CHECK: #APP
// CHECK: mov a5, extern_static
// CHECK: #NO_APP
#[no_mangle]
pub unsafe fn sym_static() {
    asm!("movi a5, {}", sym extern_static);
}

macro_rules! check_general_reg {
    ($func:ident $ty:ident $class:ident $mov:literal) => {
        #[no_mangle]
        pub unsafe fn $func(x: $ty) -> $ty {
            dont_merge(stringify!($func));

            let y;
            asm!(concat!($mov, " {}, {}"), out($class) y, in($class) x);
            y
        }
    };
}

// CHECK-LABEL: reg_i8:
// CHECK: #APP
// CHECK: mov a{{[0-9]+}}, a{{[0-9]+}}
// CHECK: #NO_APP
check_general_reg!(reg_i8 i8 reg "mov");

// CHECK-LABEL: reg_i16:
// CHECK: #APP
// CHECK: mov a{{[0-9]+}}, a{{[0-9]+}}
// CHECK: #NO_APP
check_general_reg!(reg_i16 i16 reg "mov");

// CHECK-LABEL: reg_i32:
// CHECK: #APP
// CHECK: mov a{{[0-9]+}}, a{{[0-9]+}}
// CHECK: #NO_APP
check_general_reg!(reg_i32 i32 reg "mov");

// CHECK-LABEL: reg_ptr:
// CHECK: #APP
// CHECK: mov a{{[0-9]+}}, a{{[0-9]+}}
// CHECK: #NO_APP
check_general_reg!(reg_ptr ptr reg "mov");

// CHECK-LABEL: freg_f32:
// CHECK: #APP
// CHECK: mov.s f{{[0-9]+}}, f{{[0-9]+}}
// CHECK: #NO_APP
check_general_reg!(freg_f32 f32 freg "mov.s");

// CHECK-LABEL: freg_f64:
// CHECK: #APP
// CHECK: mov.d f{{[0-9]+}}, f{{[0-9]+}}
// CHECK: #NO_APP
//check!(freg_f64 f64 freg "mov.d");    // TODO The ISA doc that I have doesn't have any F64 info

macro_rules! check_explicit_reg {
    ($func:ident $ty:ident $reg:tt $mov:literal) => {
        #[no_mangle]
        pub unsafe fn $func(x: $ty) -> $ty {
            dont_merge(stringify!($func));

            let y;
            asm!(concat!($mov, " ", $reg, ", ", $reg), lateout($reg) y, in($reg) x);
            y
        }
    };
}

// CHECK-LABEL: a5_i8:
// CHECK: #APP
// CHECK: mov a5, a5
// CHECK: #NO_APP
check_explicit_reg!(a5_i8 i8 "a5" "mov");

// CHECK-LABEL: a5_i16:
// CHECK: #APP
// CHECK: mov a5, a5
// CHECK: #NO_APP
check_explicit_reg!(a5_i16 i16 "a5" "mov");

// CHECK-LABEL: a0_i32:
// CHECK: #APP
// CHECK: mov a5, a5
// CHECK: #NO_APP
check_explicit_reg!(a5_i32 i32 "a5" "mov");

// CHECK-LABEL: a5_ptr:
// CHECK: #APP
// CHECK: mov a5, a5
// CHECK: #NO_APP
check_explicit_reg!(a5_ptr ptr "a5" "mov");

// CHECK-LABEL: f0_f32:
// CHECK: #APP
// CHECK: mov.s f0, f0
// CHECK: #NO_APP
check_explicit_reg!(f0_f32 f32 "f0" "mov.s");

// CHECK-LABEL: f0_f64:
// CHECK: #APP
// CHECK: fmv.d f0, f0
// CHECK: #NO_APP
// check_reg!(f0_f64 f64 "f0" "mov.d");    // TODO The ISA doc that I have doesn't have any F64 info

macro_rules! check_general_breg {
    ($func:ident $ty:ident $class:ident $mov:literal) => {
        #[no_mangle]
        pub unsafe fn $func(_x: $ty) -> $ty {
            dont_merge(stringify!($func));

            let y;
            asm!(concat!($mov, " {}, a9, {}"), out(reg) y, in($class) _x);
            y
        }
    };
}

// CHECK-LABEL: breg_i8:
// CHECK: #APP
// CHECK: movt a{{[0-9]+}}, a{{[0-9]+}}, b{{[0-9]+}}
// CHECK: #NO_APP
check_general_breg!(breg_i8 i8 breg "movt");

// CHECK-LABEL: breg_i16:
// CHECK: #APP
// CHECK: movt a{{[0-9]+}}, a{{[0-9]+}}, b{{[0-9]+}}
// CHECK: #NO_APP
check_general_breg!(breg_i16 i16 breg "movt");

// CHECK-LABEL: breg_i32:
// CHECK: #APP
// CHECK: mov a{{[0-9]+}}, a{{[0-9]+}}, b{{[0-9]+}}
// CHECK: #NO_APP
// check!(breg_i32 i32 reg breg "movt");
check_general_breg!(breg_i32 i32 breg "movt");

macro_rules! check_explicit_breg {
    ($func:ident $ty:ident $reg:tt $mov:literal) => {
        #[no_mangle]
        pub unsafe fn $func(x: $ty) -> $ty {
            dont_merge(stringify!($func));

            let y;
            asm!(concat!($mov, " ", $reg, ", ", $reg), lateout($reg) y, in($reg) x);
            y
        }
    };
}

// CHECK-LABEL: b0_i8:
// CHECK: #APP
// CHECK: movt a{{[0-9]+}}, a{{[0-9]+}}, b{{[0-9]+}}
// CHECK: #NO_APP
check_explicit_breg!(b0_i8 i8 "b0" "movt");

// CHECK-LABEL: b0_i16:
// CHECK: #APP
// CHECK: movt a{{[0-9]+}}, a{{[0-9]+}}, b{{[0-9]+}}
// CHECK: #NO_APP
check_explicit_breg!(b0_i16 i16 "b0" "movt");

// CHECK-LABEL: b0_i32:
// CHECK: #APP
// CHECK: movt a{{[0-9]+}}, a{{[0-9]+}}, b{{[0-9]+}}
// CHECK: #NO_APP
// check_breg!(b0_i32 i32 "a0" "b0" "movt");
check_explicit_breg!(b0_i32 i32 "b0" "movt");
