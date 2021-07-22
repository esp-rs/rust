use crate::spec::{LinkerFlavor, PanicStrategy, RelocModel};
use crate::spec::{Target, TargetOptions};

pub fn target() -> Target {
    Target {
        data_layout: "e-m:e-p:32:32-i64:64-n32-S128".to_string(),
        llvm_target: "riscv32".to_string(),
        pointer_width: 32,
        arch: "riscv32".to_string(),

        options: TargetOptions {
            os_family: Some("unix".to_string()),
            os: "espidf".to_string(),
            env: "newlib".to_string(),
            vendor: "espressif".to_string(),
            linker_flavor: LinkerFlavor::Gcc,
            linker: Some("riscv32-esp-elf-gcc".to_string()),
            cpu: "generic-rv32".to_string(),
            max_atomic_width: Some(32),
            features: "+m,+a,+c".to_string(),
            executables: true,
            panic_strategy: PanicStrategy::Abort,
            relocation_model: RelocModel::Static,
            emit_debug_gdb_scripts: false,
            unsupported_abis: super::riscv_base::unsupported_abis(),
            eh_frame_header: false,
            ..Default::default()
        },
    }
}
