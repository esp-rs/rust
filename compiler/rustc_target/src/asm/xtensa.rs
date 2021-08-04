use super::{InlineAsmArch, InlineAsmType};
// use crate::spec::Target;
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
            Self::freg => types! { _: F32; },
        }
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
        lbeg: reg = ["lbeg"],
        lend: reg = ["lend"],
        lcount: reg = ["lcount"],
        sar: reg = ["sar"],
        br: reg = ["br"],
        litbase: reg = ["litbase"],
        scompare1: reg = ["scompare1"],
        acclo: reg = ["acclo"],
        acchi: reg = ["acchi"],
        m0: reg = ["m0"],
        m1: reg = ["m1"],
        m2: reg = ["m2"],
        m3: reg = ["m3"],
        windowbase: reg = ["windowbase"],
        windowstart: reg = ["windowstart"],
        ibreakenable: reg = ["ibreakenable"],
        memctl: reg = ["memctl"],
        atomctl: reg = ["atomctl"],
        ddr: reg = ["ddr"],
        ibreaka0: reg = ["ibreaka0"],
        ibreaka1: reg = ["ibreaka1"],
        dbreaka0: reg = ["dbreaka0"],
        dbreaka1: reg = ["dbreaka1"],
        dbreakc0: reg = ["dbreakc0"],
        dbreakc1: reg = ["dbreakc1"],
        configid0: reg = ["configid0"],
        epc1: reg = ["epc1"],
        epc2: reg = ["epc2"],
        epc3: reg = ["epc3"],
        epc4: reg = ["epc4"],
        epc5: reg = ["epc5"],
        epc6: reg = ["epc6"],
        epc7: reg = ["epc7"],
        depc: reg = ["depc"],
        eps2: reg = ["eps2"],
        eps3: reg = ["eps3"],
        eps4: reg = ["eps4"],
        eps5: reg = ["eps5"],
        eps6: reg = ["eps6"],
        eps7: reg = ["eps7"],
        configid1: reg = ["configid1"],
        excsave1: reg = ["excsave1"],
        excsave2: reg = ["excsave2"],
        excsave3: reg = ["excsave3"],
        excsave4: reg = ["excsave4"],
        excsave5: reg = ["excsave5"],
        excsave6: reg = ["excsave6"],
        excsave7: reg = ["excsave7"],
        cpenable: reg = ["cpenable"],
        interrupt: reg = ["interrupt"],
        intclear: reg = ["intclear"],
        intenable: reg = ["intenable"],
        ps: reg = ["ps"],
        vecbase: reg = ["vecbase"],
        exccause: reg = ["exccause"],
        debugcause: reg = ["debugcause"],
        ccount: reg = ["ccount"],
        prid: reg = ["prid"],
        icount: reg = ["icount"],
        icountlevel: reg = ["icountlevel"],
        excvaddr: reg = ["excvaddr"],
        ccompare0: reg = ["ccompare0"],
        ccompare1: reg = ["ccompare1"],
        ccompare2: reg = ["ccompare2"],
        misc0: reg = ["misc0"],
        misc1: reg = ["misc1"],
        misc2: reg = ["misc2"],
        misc3: reg = ["misc3"],
        gpio_out: reg = ["gpio_out"],
        expstate: reg = ["expstate"],
        threadptr: reg = ["threadptr"],
        fcr: reg = ["fcr"],
        fsr: reg = ["fsr"],
        f64r_lo: reg = ["f64r_lo"],
        f64r_hi: reg = ["f64r_hi"],
        f64s: reg = ["f64s"],
        f0: freg = ["f0"],
        f1: freg = ["f1"],
        f2: freg = ["f2"],
        f3: freg = ["f3"],
        f4: freg = ["f4"],
        f5: freg = ["f5"],
        f6: freg = ["f6"],
        f7: freg = ["f7"],
        f8: freg = ["f8"],
        f9: freg = ["f9"],
        f10: freg = ["f10"],
        f11: freg = ["f11"],
        f12: freg = ["f12"],
        f13: freg = ["f13"],
        f14: freg = ["f14"],
        f15: freg = ["f15"],
        b0: breg = ["b0"],
        b1: breg = ["b1"],
        b2: breg = ["b2"],
        b3: breg = ["b3"],
        b4: breg = ["b4"],
        b5: breg = ["b5"],
        b6: breg = ["b6"],
        b7: breg = ["b7"],
        b8: breg = ["b8"],
        b9: breg = ["b9"],
        b10: breg = ["b10"],
        b11: breg = ["b11"],
        b12: breg = ["b12"],
        b13: breg = ["b13"],
        b14: breg = ["b14"],
        b15: breg = ["b15"],
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
