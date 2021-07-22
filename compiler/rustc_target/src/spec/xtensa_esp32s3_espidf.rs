use crate::spec::{cvs, Target, TargetOptions};
use crate::abi::Endian;

pub fn target() -> Target {
    Target {
        llvm_target: "xtensa-none-elf".into(),
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-i64:64-i128:128-n32".into(),
        arch: "xtensa".into(),

        options: TargetOptions {
            endian: Endian::Little,
            c_int_width: "32".into(),
            families: cvs!["unix"],
            os: "espidf".into(),
            env: "newlib".into(),
            vendor: "espressif".into(),

            executables: true,
            cpu: "esp32-s3".into(),
            linker: Some("xtensa-esp32s3-elf-gcc".into()),

            // The esp32s3 only supports native 32bit atomics. However, esp-idf will emulate 64bit atomics 
            // so we claim a max atomic width of 64 here.
            max_atomic_width: Some(64),
            atomic_cas: true,

            ..super::xtensa_base::opts()
        },
    }
}