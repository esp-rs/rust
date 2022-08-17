// reference: https://github.com/MabezDev/llvm-project/blob/xtensa_release_9.0.1_with_rust_patches-31-05-2020-cherry-pick/clang/lib/CodeGen/TargetInfo.cpp#L9668-L9767

use crate::abi::call::{ArgAbi, FnAbi, Reg, Uniform};
use crate::abi::{Abi, HasDataLayout, Size, TyAbiInterface};
use crate::spec::HasTargetSpec;

const NUM_ARG_GPRS: u64 = 6;
const NUM_RET_ARG_GPRS: u64 = 4;

fn classify_ret_ty<'a, Ty, C>(arg: &mut ArgAbi<'_, Ty>)
where
    Ty: TyAbiInterface<'a, C> + Copy,
{
    let mut arg_gprs_left = NUM_RET_ARG_GPRS;
    classify_arg_ty(arg, &mut arg_gprs_left, NUM_RET_ARG_GPRS);
    
    // classify_arg_ty can make the arg indirect by value which is not valid for ret args
    match arg.mode {
        super::PassMode::Indirect { attrs: _, extra_attrs: _, ref mut on_stack } => *on_stack = false,
        _ => {}
    }
}

fn classify_arg_ty<'a, Ty, C>(arg: &mut ArgAbi<'_, Ty>, arg_gprs_left: &mut u64, num_gprs: u64)
where
    Ty: TyAbiInterface<'a, C> + Copy,
{
    assert!(*arg_gprs_left <= num_gprs, "Arg GPR tracking underflow");

    if arg.layout.is_zst() {
        return; // ignore args that take no size
    }

    // TODO check is arg has non-trvial constructor

    let size = arg.layout.size.bits();
    let align = arg.layout.align.abi.bits();
    let mut must_use_stack = false;

    let mut required_gprs = (size + 31) / 32;

    if align == 64 {
        required_gprs += *arg_gprs_left % 2;
    }

    // Put on stack objects which are not fit to 6 registers,
    // also on stack object which alignment more then 16 bytes and
    // object with 16-byte alignment if it isn't the first argument.
    if required_gprs > *arg_gprs_left || align > 128 || *arg_gprs_left < 6 && align == 128 {
        must_use_stack = true;
        required_gprs = *arg_gprs_left;
    }
    *arg_gprs_left -= required_gprs;

    if arg.layout.is_aggregate() && !matches!(arg.layout.abi, Abi::Vector { element: _ , count: _ }) && !must_use_stack {
        if size < 32 && !must_use_stack {
            arg.extend_integer_width_to(32);
        } else if size == 64 {
            arg.cast_to(Reg::i64());
        } else {
            arg.cast_to(Reg::i32());
        }
        return;
    }

    // Aggregates which are <= 6*32 will be passed in registers if possible,
    // so coerce to integers.
    if size <= (num_gprs * 32) && !must_use_stack {
        if size <= 32 {
            arg.cast_to(Reg::i32())
        } else if align == 64 {
            arg.cast_to(Uniform { unit: Reg::i64(), total: Size::from_bits((required_gprs / 2) * 32) });
        } else {
            arg.cast_to(Uniform { unit: Reg::i32(), total: Size::from_bits(required_gprs * 32) });
        }
        return;
    }

    arg.make_indirect_byval()
}

pub fn compute_abi_info<'a, Ty, C>(_cx: &C, fn_abi: &mut FnAbi<'a, Ty>)
where
    Ty: TyAbiInterface<'a, C> + Copy,
    C: HasDataLayout + HasTargetSpec,
{
    if !fn_abi.ret.is_ignore() {
        classify_ret_ty(&mut fn_abi.ret);
    }
    
    let mut avail_gprs = NUM_ARG_GPRS;

    for arg in fn_abi.args.iter_mut(){
        if arg.is_ignore() {
            continue;
        }
        classify_arg_ty(
            arg,
            &mut avail_gprs,
            NUM_ARG_GPRS
        );
    }
}
