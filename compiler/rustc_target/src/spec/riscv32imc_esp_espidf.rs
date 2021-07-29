use crate::spec::{LinkerFlavor, PanicStrategy, RelocModel};
use crate::spec::{Target, TargetOptions};

pub fn target() -> Target {
    Target {
        data_layout: "e-m:e-p:32:32-i64:64-n32-S128".to_string(),
        llvm_target: "riscv32".to_string(),
        pointer_width: 32,
        arch: "riscv32".to_string(),

        options: TargetOptions {
            families: vec!["unix".to_string()],
            os: "espidf".to_string(),
            env: "newlib".to_string(),
            vendor: "espressif".to_string(),
            linker_flavor: LinkerFlavor::Gcc,
            linker: Some("riscv32-esp-elf-gcc".to_string()),
            cpu: "generic-rv32".to_string(),

            // See https://github.com/espressif/rust-esp32-example/issues/3#issuecomment-861054477
            //
            // The RISCV32IMC architecture does not support atomics.
            // However, simultaneously claiming "max_atomic_width: Some(32)" **and** "atomic_cas: true",
            // forces the compiler to generate libcalls to functions that emulate atomics
            // and which are already implemented in the ESP-IDF main branch anyway.
            //
            // The plan forward is as follows:
            // - If the missing hardware instruction(s) end up being emulated in ESP-IDF, we will remove
            //   this target altogether and use the riscv32imac-esp-espidf target with ESP-IDF
            // - Otherwise, we'll use this target and remove the riscv32imac-esp-espidf one
            max_atomic_width: Some(32),
            atomic_cas: true,

            features: "+m,+c".to_string(),
            executables: true,
            panic_strategy: PanicStrategy::Abort,
            relocation_model: RelocModel::Static,
            emit_debug_gdb_scripts: false,
            eh_frame_header: false,
            ..Default::default()
        },
    }
}
