use rustc_abi::Endian;

use crate::spec::base::xtensa;
use crate::spec::{Arch, Env, Os, Target, TargetMetadata, TargetOptions, cvs};

pub(crate) fn target() -> Target {
    Target {
        llvm_target: "xtensa-none-elf".into(),
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-i8:8:32-i16:16:32-i64:64-n32".into(),
        arch: Arch::Xtensa,
        metadata: TargetMetadata { description: None, tier: Some(3), host_tools: None, std: None },

        options: TargetOptions {
            endian: Endian::Little,
            c_int_width: 32,
            families: cvs!["unix"],
            os: Os::EspIdf,
            env: Env::Newlib,
            vendor: "espressif".into(),

            executables: true,
            features: "+density,+fp,+loop,+mac16,+windowed,+bool,+sext,+nsa,+mul16,+mul32,+mul32high,+s32c1i,+threadptr,+div32,+dcache,+debug,+exception,+highpriinterrupts,+highpriinterrupts-level7,+coprocessor,+interrupt,+rvector,+timers3,+prid,+regprotect,+miscsr,+minmax,+clamps".into(),
            linker: Some("xtensa-esp32s3-elf-gcc".into()),

            // The esp32s3 only supports native 32bit atomics.
            max_atomic_width: Some(32),
            atomic_cas: true,

            ..xtensa::opts()
        },
    }
}
