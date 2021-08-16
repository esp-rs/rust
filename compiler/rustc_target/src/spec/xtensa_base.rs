use crate::spec::{LinkerFlavor, PanicStrategy, RelocModel, TargetOptions};
use crate::{abi::Endian, spec::abi::Abi};

pub fn opts() -> TargetOptions {
    TargetOptions {
        os: "none".to_string(),
        env: String::new(),
        vendor: String::new(),
        endian: Endian::Little,
        c_int_width: "32".to_string(),
        linker_flavor: LinkerFlavor::Gcc,
        executables: true,
        max_atomic_width: Some(32),
        panic_strategy: PanicStrategy::Abort,
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
    }
}
