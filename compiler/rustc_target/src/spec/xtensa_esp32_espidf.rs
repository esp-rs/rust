use crate::spec::{abi::Abi, LinkerFlavor, PanicStrategy, Target, TargetOptions, RelocModel};
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
            os_family: Some("unix".to_string()),
            os: "espidf".to_string(),
            env: "newlib".to_string(),
            vendor: "espressif".to_string(),
            linker_flavor: LinkerFlavor::Gcc,

            executables: true,
            cpu: "esp32".to_string(),
            linker: Some("xtensa-esp32-elf-gcc".to_string()),

            max_atomic_width: Some(32),

            // Because these devices have very little resources having an
            // unwinder is too onerous so we default to "abort" because the
            // "unwind" strategy is very rare.
            panic_strategy: PanicStrategy::Abort,

            // Similarly, one almost always never wants to use relocatable
            // code because of the extra costs it involves.
            relocation_model: RelocModel::Static,

            emit_debug_gdb_scripts: false,

            unsupported_abis: vec![
                Abi::Stdcall { unwind: false },
                Abi::Stdcall { unwind: true },
                Abi::Fastcall,
                Abi::Vectorcall,
                Abi::Thiscall { unwind: false },
                Abi::Thiscall { unwind: true },
                Abi::Win64,
                Abi::SysV64,
            ],

            ..Default::default()
        },
    }
}