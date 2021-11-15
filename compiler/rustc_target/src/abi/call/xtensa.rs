// reference: https://github.com/MabezDev/llvm-project/blob/xtensa_release_9.0.1_with_rust_patches-31-05-2020-cherry-pick/clang/lib/CodeGen/TargetInfo.cpp#L9668-L9767

use crate::abi::call::{ArgAbi, FnAbi, Reg, Uniform};
use crate::abi::{Abi, HasDataLayout, Size, TyAbiInterface, Align};
use crate::spec::HasTargetSpec;

const NUM_ARG_GPRS: u64 = 6;
const NUM_RET_GPRS: u64 = 4;
const XLEN: u64 = 32;

fn classify_ret_ty<'a, Ty, C>(arg: &mut ArgAbi<'_, Ty>)
where
    Ty: TyAbiInterface<'a, C> + Copy,
{
    if arg.is_ignore() {
        return;
    }

    let ret_size = arg.layout.size.align_to(Align::from_bits(32).unwrap()).bits() / 32;

    // The rules for return and argument with type size more then 4 bytes
    // are the same, so defer to classify_arg_ty.
    if ret_size > 1 {
        let mut arg_gprs_left = NUM_RET_GPRS;
        classify_arg_ty(arg, &mut arg_gprs_left);
    }
    // TODO do we need to do anything else here?
}

fn classify_arg_ty<'a, Ty, C>(arg: &mut ArgAbi<'_, Ty>, arg_gprs_left: &mut u64)
where
    Ty: TyAbiInterface<'a, C> + Copy,
{
    assert!(*arg_gprs_left <= NUM_ARG_GPRS, "Arg GPR tracking underflow");

    // if arg.is_indirect() {
    //     arg.make_indirect_byval();
    //     *arg_gprs_left = arg_gprs_left.saturating_sub(1);
    //     return;
    // }

    // Ignore empty structs/unions.
    if arg.layout.is_zst() {
        return;
    }

    let size = arg.layout.size.bits();
    let needed_align = arg.layout.align.abi.bits();
    let mut must_use_stack = false;

    let mut needed_arg_gprs = (size + (XLEN - 1)) / XLEN;

    if needed_align == 2 * XLEN {
        needed_arg_gprs += *arg_gprs_left % 2;
    }

    if needed_arg_gprs > *arg_gprs_left || needed_align > Size::from_bytes(4).bits() || ((*arg_gprs_left < 6) || needed_align == Size::from_bytes(4).bits()) {
        must_use_stack = true;
        needed_arg_gprs = *arg_gprs_left;
    }
    *arg_gprs_left -= needed_arg_gprs;

    if !arg.layout.is_aggregate() && !matches!(arg.layout.abi, Abi::Vector { .. }) {
        // All integral types are promoted to `xlen`
        // width, unless passed on the stack.
        if size < XLEN && !must_use_stack {
            arg.extend_integer_width_to(XLEN);
            return;
        }
        return;
    }

    // Aggregates which are <= 6 * 32 will be passed in
    // registers if possible, so coerce to integers.
    if size <= Size::from_bits(NUM_ARG_GPRS * 32).bits() && !must_use_stack {
        let alignment = arg.layout.align.abi.bits();
        // Use a single `xlen` int if possible, 2 * `xlen` if 2 * `xlen` alignment
        // is required, and a 2-element `xlen` array if only `xlen` alignment is
        // required.
        if size <= XLEN {
            arg.cast_to(Reg::i32());
            return;
        } else if alignment == 2 * XLEN {
            arg.cast_to(Reg::i64());
            return;
        } else {
            let total = Size::from_bits(needed_arg_gprs * XLEN);
            arg.cast_to(Uniform { unit: Reg::i32(), total });
            return;
        }
    }

    arg.make_indirect();
}

pub fn compute_abi_info<'a, Ty, C>(_cx: &C, fn_abi: &mut FnAbi<'a, Ty>)
where
    Ty: TyAbiInterface<'a, C> + Copy,
    C: HasDataLayout + HasTargetSpec,
{
    if !fn_abi.ret.is_ignore() {
        classify_ret_ty(&mut fn_abi.ret);
    }

    let mut arg_gprs_left = NUM_ARG_GPRS;
    for arg in &mut fn_abi.args {
        // if arg.is_ignore() {
        //     continue;
        // }
        classify_arg_ty(arg, &mut arg_gprs_left);
    }
}
