use super::{InlineAsmArch, InlineAsmType};
use crate::spec::Target;
use rustc_macros::HashStable_Generic;
use std::fmt;

def_reg_class! {
    Xtensa XtensaInlineAsmRegClass {
        reg,
        freg,
        breg,
    }
}

impl XtensaInlineAsmRegClass {
    pub fn valid_modifiers(self, _arch: super::InlineAsmArch) -> &'static [char] {
        &[]
    }

    pub fn suggest_class(self, _arch: InlineAsmArch, _ty: InlineAsmType) -> Option<Self> {
        None
    }

    pub fn suggest_modifier(
        self,
        _arch: InlineAsmArch,
        _ty: InlineAsmType,
    ) -> Option<(char, &'static str)> {
        None
    }

    pub fn default_modifier(self, _arch: InlineAsmArch) -> Option<(char, &'static str)> {
        None
    }

    pub fn supported_types(
        self,
        _arch: InlineAsmArch,
    ) -> &'static [(InlineAsmType, Option<&'static str>)] {
        match self {
            Self::reg | Self::breg => types! { _: I8, I16, I32; },
            Self::freg => types! { "fp":F32; }, // TODO how does the dfpaccel feature interact F64 types? // _:F64;
        }
    }
}

// Xtensa has lots a features - macro to reduce boiler plate
macro_rules! feature {
    ($fnname:ident, $feature:expr) => {
        fn $fnname(
            _arch: InlineAsmArch,
            mut has_feature: impl FnMut(&str) -> bool,
            _target: &Target,
        ) -> Result<(), &'static str> {
            if has_feature($feature) {
                Ok(())
            } else {
                Err(concat!("target does not support ", $feature, " registers"))
            }
        }
    };
}

feature!(has_fp, "fp");
feature!(has_dfpaccel, "dfpaccel");
feature!(has_bool, "bool");
feature!(has_loop, "loop");
feature!(has_extendedl32r, "extendedl32r");
feature!(has_s32c1i, "s32c1i");
feature!(has_mac16, "mac16");
feature!(has_windowed, "windowed");
feature!(has_debug, "debug");
feature!(has_memctl, "memctl");
feature!(has_atomctl, "atomctl");
feature!(has_exception, "exception");
feature!(has_coprocessor, "coprocessor");
feature!(has_rvector, "rvector");
feature!(has_timerint, "timerint");
feature!(has_interrupt, "interrupt");
feature!(has_prid, "prid");
feature!(has_miscsr, "miscsr");
feature!(has_threadptr, "threadptr");

fn has_expstate(
    _arch: InlineAsmArch,
    _has_feature: impl FnMut(&str) -> bool,
    target: &Target,
) -> Result<(), &'static str> {
    match target.cpu.as_str() {
        "esp32" => Ok(()),
        _ => Err("target does not support expstate registers")
    }
}
fn has_gpio_out(
    _arch: InlineAsmArch,
    _has_feature: impl FnMut(&str) -> bool,
    target: &Target,
) -> Result<(), &'static str> {
    match target.cpu.as_str() {
        "esp32-s2" => Ok(()),
        _ => Err("target does not support gpio_out registers")
    }
}



def_regs! {
    Xtensa XtensaInlineAsmReg XtensaInlineAsmRegClass {
        a0: reg = ["a0"],
        sp: reg = ["sp", "a1"],
        a2: reg = ["a2"],
        a3: reg = ["a3"],
        a4: reg = ["a4"],
        a5: reg = ["a5"],
        a6: reg = ["a6"],
        a7: reg = ["a7"],
        a8: reg = ["a8"],
        a9: reg = ["a9"],
        a10: reg = ["a10"],
        a11: reg = ["a11"],
        a12: reg = ["a12"],
        a13: reg = ["a13"],
        a14: reg = ["a14"],
        a15: reg = ["a15"],
        sar: reg = ["sar"], // TODO what feature enables this, if any? 
        ddr: reg = ["ddr"], // TODO what feature enables this, if any? 
        ps: reg = ["ps"], // TODO what feature enables this, if any? 
        configid0: reg = ["configid0"], // TODO what feature enables this, if any? 
        configid1: reg = ["configid1"], // TODO what feature enables this, if any? 
        lbeg: reg = ["lbeg"] % has_loop,
        lend: reg = ["lend"] % has_loop,
        lcount: reg = ["lcount"] % has_loop,
        litbase: reg = ["litbase"] % has_extendedl32r,
        scompare1: reg = ["scompare1"] % has_s32c1i,
        acclo: reg = ["acclo"] % has_mac16,
        acchi: reg = ["acchi"] % has_mac16,
        m0: reg = ["m0"] % has_mac16,
        m1: reg = ["m1"] % has_mac16,
        m2: reg = ["m2"] % has_mac16,
        m3: reg = ["m3"] % has_mac16,
        windowbase: reg = ["windowbase"] % has_windowed,
        windowstart: reg = ["windowstart"] % has_windowed,
        ibreakenable: reg = ["ibreakenable"] % has_debug,
        ibreaka0: reg = ["ibreaka0"] % has_debug,
        ibreaka1: reg = ["ibreaka1"] % has_debug,
        dbreaka0: reg = ["dbreaka0"] % has_debug,
        dbreaka1: reg = ["dbreaka1"] % has_debug,
        dbreakc0: reg = ["dbreakc0"] % has_debug,
        dbreakc1: reg = ["dbreakc1"] % has_debug,
        icount: reg = ["icount"] % has_debug,
        icountlevel: reg = ["icountlevel"] % has_debug,
        debugcause: reg = ["debugcause"] % has_debug,
        memctl: reg = ["memctl"] % has_memctl,
        atomctl: reg = ["atomctl"] % has_atomctl,
        epc1: reg = ["epc1"] % has_exception,
        epc2: reg = ["epc2"] % has_exception,
        epc3: reg = ["epc3"] % has_exception,
        epc4: reg = ["epc4"] % has_exception,
        epc5: reg = ["epc5"] % has_exception,
        epc6: reg = ["epc6"] % has_exception,
        epc7: reg = ["epc7"] % has_exception,
        depc: reg = ["depc"] % has_exception,
        eps2: reg = ["eps2"] % has_exception,
        eps3: reg = ["eps3"] % has_exception,
        eps4: reg = ["eps4"] % has_exception,
        eps5: reg = ["eps5"] % has_exception,
        eps6: reg = ["eps6"] % has_exception,
        eps7: reg = ["eps7"] % has_exception,
        excsave1: reg = ["excsave1"] % has_exception,
        excsave2: reg = ["excsave2"] % has_exception,
        excsave3: reg = ["excsave3"] % has_exception,
        excsave4: reg = ["excsave4"] % has_exception,
        excsave5: reg = ["excsave5"] % has_exception,
        excsave6: reg = ["excsave6"] % has_exception,
        excsave7: reg = ["excsave7"] % has_exception,
        exccause: reg = ["exccause"] % has_exception,
        excvaddr: reg = ["excvaddr"] % has_exception,
        cpenable: reg = ["cpenable"] % has_coprocessor,
        vecbase: reg = ["vecbase"] % has_rvector,
        interrupt: reg = ["interrupt"] % has_interrupt,
        intclear: reg = ["intclear"] % has_interrupt,
        intenable: reg = ["intenable"] % has_interrupt,
        prid: reg = ["prid"] % has_prid,
        ccount: reg = ["ccount"] % has_timerint,
        ccompare0: reg = ["ccompare0"] % has_timerint,
        ccompare1: reg = ["ccompare1"] % has_timerint,
        ccompare2: reg = ["ccompare2"] % has_timerint,
        misc0: reg = ["misc0"] % has_miscsr,
        misc1: reg = ["misc1"] % has_miscsr,
        misc2: reg = ["misc2"] % has_miscsr,
        misc3: reg = ["misc3"] % has_miscsr,
        gpio_out: reg = ["gpio_out"] % has_gpio_out,
        expstate: reg = ["expstate"] % has_expstate,
        threadptr: reg = ["threadptr"] % has_threadptr,
        fcr: reg = ["fcr"] % has_dfpaccel,
        fsr: reg = ["fsr"] % has_dfpaccel,
        f64r_lo: reg = ["f64r_lo"] % has_dfpaccel,
        f64r_hi: reg = ["f64r_hi"] % has_dfpaccel,
        f64s: reg = ["f64s"] % has_dfpaccel,
        f0: freg = ["f0"] % has_fp,
        f1: freg = ["f1"] % has_fp,
        f2: freg = ["f2"] % has_fp,
        f3: freg = ["f3"] % has_fp,
        f4: freg = ["f4"] % has_fp,
        f5: freg = ["f5"] % has_fp,
        f6: freg = ["f6"] % has_fp,
        f7: freg = ["f7"] % has_fp,
        f8: freg = ["f8"] % has_fp,
        f9: freg = ["f9"] % has_fp,
        f10: freg = ["f10"] % has_fp,
        f11: freg = ["f11"] % has_fp,
        f12: freg = ["f12"] % has_fp,
        f13: freg = ["f13"] % has_fp,
        f14: freg = ["f14"] % has_fp,
        f15: freg = ["f15"] % has_fp,
        br: reg = ["br"] % has_bool,
        b0: breg = ["b0"] % has_bool,
        b1: breg = ["b1"] % has_bool,
        b2: breg = ["b2"] % has_bool,
        b3: breg = ["b3"] % has_bool,
        b4: breg = ["b4"] % has_bool,
        b5: breg = ["b5"] % has_bool,
        b6: breg = ["b6"] % has_bool,
        b7: breg = ["b7"] % has_bool,
        b8: breg = ["b8"] % has_bool,
        b9: breg = ["b9"] % has_bool,
        b10: breg = ["b10"] % has_bool,
        b11: breg = ["b11"] % has_bool,
        b12: breg = ["b12"] % has_bool,
        b13: breg = ["b13"] % has_bool,
        b14: breg = ["b14"] % has_bool,
        b15: breg = ["b15"] % has_bool,
    }
}

impl XtensaInlineAsmReg {
    pub fn emit(
        self,
        out: &mut dyn fmt::Write,
        _arch: InlineAsmArch,
        _modifier: Option<char>,
    ) -> fmt::Result {
        out.write_str(self.name())
    }
}
