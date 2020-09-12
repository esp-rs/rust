use crate::spec::{LinkerFlavor, PanicStrategy, RelocModel, TargetOptions};
use crate::abi::Endian;

pub fn opts() -> TargetOptions {
    TargetOptions {
        os: "none".into(),
        endian: Endian::Little,
        c_int_width: "32".into(),
        linker_flavor: LinkerFlavor::Gcc,
        executables: true,
        panic_strategy: PanicStrategy::Abort,
        relocation_model: RelocModel::Static,
        emit_debug_gdb_scripts: false,
        atomic_cas: false,
        ..Default::default()
    }
}
