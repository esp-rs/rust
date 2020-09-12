use crate::spec::{LinkerFlavor, PanicStrategy, RelocModel, TargetOptions};
use crate::abi::Endian;

pub fn opts() -> TargetOptions {
    TargetOptions {
        os: "none".to_string(),
        env: String::new(),
        vendor: String::new(),
        endian: Endian::Little,
        c_int_width: "32".to_string(),
        linker_flavor: LinkerFlavor::Gcc,
        executables: true,
        panic_strategy: PanicStrategy::Abort,
        relocation_model: RelocModel::Static,
        emit_debug_gdb_scripts: false,
        ..Default::default()
    }
}
