use crate::spec::{Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: "xtensa-none-elf".to_string(),
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-i8:8:32-i16:16:32-i64:64-n32".to_string(),
        arch: "xtensa".to_string(),
        
        options: TargetOptions {
            cpu: "esp32-s2".to_string(),
            linker: Some("xtensa-esp32s2-elf-gcc".to_string()),
            ..super::xtensa_base::opts()
        },
    }
}