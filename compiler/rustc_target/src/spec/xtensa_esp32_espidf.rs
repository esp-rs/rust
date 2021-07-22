use crate::spec::{LinkerFlavor, Target, TargetOptions};
use crate::abi::Endian;

pub fn target() -> Target {
    Target {
        llvm_target: "xtensa-none-elf".to_string(),
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-i8:8:32-i16:16:32-i64:64-n32".to_string(),
        arch: "xtensa".to_string(),

        options: TargetOptions {
            endian: Endian::Little,
            c_int_width: "32".to_string(),
            families: vec!["unix".to_string()],
            os: "espidf".to_string(),
            env: "newlib".to_string(),
            vendor: "espressif".to_string(),
            linker_flavor: LinkerFlavor::Gcc,

            executables: true,
            cpu: "esp32".to_string(),
            linker: Some("xtensa-esp32-elf-gcc".to_string()),

            // The esp32 only supports native 32bit atomics. However, esp-idf will emulate 64bit atomics 
            // so we claim a max atomic width of 64 here.
            max_atomic_width: Some(64),
            atomic_cas: true,

            ..super::xtensa_base::opts()
        },
    }
}