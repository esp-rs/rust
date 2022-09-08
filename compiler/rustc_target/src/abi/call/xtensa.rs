// reference: https://github.com/MabezDev/llvm-project/blob/xtensa_release_9.0.1_with_rust_patches-31-05-2020-cherry-pick/clang/lib/CodeGen/TargetInfo.cpp#L9668-L9767

use crate::abi::call::{ArgAbi, FnAbi, Reg, Uniform};
use crate::abi::{Abi, HasDataLayout, Size, TyAbiInterface};
use crate::spec::HasTargetSpec;

const NUM_ARG_GPRS: u64 = 6;
const NUM_RET_GPRS: u64 = 4;
const MAX_ARG_IN_REGS_SIZE: u64 = 6 * 32;
const MAX_RET_IN_REGS_SIZE: u64 = 4 * 32;

fn classify_ret_ty<'a, Ty, C>(arg: &mut ArgAbi<'_, Ty>, xlen: u64)
where
    Ty: TyAbiInterface<'a, C> + Copy,
{
    if arg.is_ignore() {
        return;
    }

    // The rules for return and argument types are the same,
    // so defer to `classify_arg_ty`.
    let mut arg_gprs_left = NUM_RET_GPRS;
    classify_arg_ty(arg, xlen, &mut arg_gprs_left, MAX_RET_IN_REGS_SIZE);
}

fn classify_arg_ty<'a, Ty, C>(
    arg: &mut ArgAbi<'_, Ty>,
    xlen: u64,
    arg_gprs_left: &mut u64,
    max_size: u64,
) where
    Ty: TyAbiInterface<'a, C> + Copy,
{
    assert!(*arg_gprs_left <= NUM_ARG_GPRS, "Arg GPR tracking underflow");

    // Ignore empty structs/unions.
    if arg.layout.is_zst() {
        return;
    }

    let size = arg.layout.size.bits();
    let needed_align = arg.layout.align.abi.bits();
    let mut must_use_stack = false;

    // Determine the number of GPRs needed to pass the current argument
    // according to the ABI. 2*XLen-aligned varargs are passed in "aligned"
    // register pairs, so may consume 3 registers.
    let mut needed_arg_gprs = 1u64;

    if needed_align == 2 * xlen {
        needed_arg_gprs = 2 + (*arg_gprs_left % 2);
    } else if size > xlen && size <= max_size {
        needed_arg_gprs = (size + xlen - 1) / xlen;
    }

    if needed_arg_gprs > *arg_gprs_left
        || needed_align > 128
        || *arg_gprs_left < (max_size / 32) && needed_align == 128
    {
        must_use_stack = true;
        needed_arg_gprs = *arg_gprs_left;
    }
    *arg_gprs_left -= needed_arg_gprs;

    if must_use_stack {
        arg.make_indirect_byval();
    } else {
        if is_xtensa_aggregate(arg) {
            // Aggregates which are <= max_size will be passed in
            // registers if possible, so coerce to integers.

            // Use a single `xlen` int if possible, 2 * `xlen` if 2 * `xlen` alignment
            // is required, and a 2-element `xlen` array if only `xlen` alignment is
            // required.
            if size <= xlen {
                arg.cast_to(Reg::i32());
            } else {
                let reg = if needed_align == 2 * xlen { Reg::i64() } else { Reg::i32() };
                let total = Size::from_bits(((size + xlen - 1) / xlen) * xlen);
                arg.cast_to(Uniform { unit: reg, total });
            }
        } else {
            // All integral types are promoted to `xlen`
            // width.
            //
            // We let the LLVM backend handle integral types >= xlen.
            if size < xlen {
                arg.extend_integer_width_to(xlen);
            }
        }
    }
}

pub fn compute_abi_info<'a, Ty, C>(cx: &C, fn_abi: &mut FnAbi<'a, Ty>)
where
    Ty: TyAbiInterface<'a, C> + Copy,
    C: HasDataLayout + HasTargetSpec,
{
    let xlen = cx.data_layout().pointer_size.bits();

    if !fn_abi.ret.is_ignore() {
        classify_ret_ty(&mut fn_abi.ret, xlen);
    }

    let mut arg_gprs_left = NUM_ARG_GPRS;

    for arg in &mut fn_abi.args {
        if arg.is_ignore() {
            continue;
        }
        classify_arg_ty(arg, xlen, &mut arg_gprs_left, MAX_ARG_IN_REGS_SIZE);
    }
}

fn is_xtensa_aggregate<'a, Ty>(arg: &ArgAbi<'a, Ty>) -> bool {
    match arg.layout.abi {
        Abi::Vector { .. } => true,
        _ => arg.layout.is_aggregate(),
    }
}
