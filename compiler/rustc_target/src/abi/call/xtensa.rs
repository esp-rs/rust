// reference: https://github.com/espressif/llvm-project/blob/e9f57cdbcf3e0a63f395e683ccfaf7c4e6e1b093/clang/lib/CodeGen/TargetInfo.cpp#L11241

use crate::abi::call::{ArgAbi, FnAbi, Reg, Uniform};
use crate::abi::{HasDataLayout, Size, TyAbiInterface, Abi};
use crate::spec::HasTargetSpec;

const NUM_ARG_GPRS: u64 = 6;
const NUM_RET_ARG_GPRS: u64 = 4;

fn classify_ret_ty<'a, Ty, C>(arg: &mut ArgAbi<'_, Ty>)
where
    Ty: TyAbiInterface<'a, C> + Copy,
{
    let mut arg_gprs_left = NUM_RET_ARG_GPRS;

    // let size = arg.layout.size.align_to(Align::from_bits(32).unwrap());
    // let size = arg.layout.size;

    // The rules for return and argument with type size more then 4 bytes
    // are the same, so defer to classifyArgumentType.
    // if size.bits() > 32 {
    classify_arg_ty(arg, &mut arg_gprs_left, NUM_RET_ARG_GPRS);

    // classify_arg_ty can make the arg indirect by value which is not valid for ret args
    match arg.mode {
        super::PassMode::Indirect { attrs: _, extra_attrs: _, ref mut on_stack } => {
            *on_stack = false
        }
        _ => {}
    }
    // } else {
    //     // LLVM DefaultABIInfo::classifyReturnType
    //     if arg.layout.is_aggregate() {
    //         arg.make_indirect()
    //     } else if (!arg.layout.is_aggregate() && !matches!(arg.layout.abi, Abi::Vector { .. })) {
    //         arg.extend_integer_width_to(32)
    //     } else {
    //         assert!(arg.layout.align.abi.bits() < 64); // higher align requires possible padding
    //         arg.cast_to(Uniform { unit: Reg::i32(), total: size });
    //     }
    // }
}

fn classify_arg_ty<'a, Ty, C>(arg: &mut ArgAbi<'_, Ty>, arg_gprs_left: &mut u64, num_gprs: u64)
where
    Ty: TyAbiInterface<'a, C> + Copy,
{
    assert!(*arg_gprs_left <= num_gprs, "GPR tracking underflow");

    if arg.layout.is_zst() {
        return; // ignore args that take no size
    }

    let size = arg.layout.size.bits();
    let align = arg.layout.align.abi.bits();
    let mut must_use_stack = false;

    let mut required_gprs = (size + 31) / 32;

    if align == 64 {
        required_gprs += *arg_gprs_left % 2;
    }

    // Put on stack objects which are not fit to num_gprs registers,
    // also on stack object which alignment more then 16 bytes and
    // object with 16-byte alignment if it isn't the first argument.
    if required_gprs > *arg_gprs_left || align > 128 || *arg_gprs_left < num_gprs && align == 128 {
        must_use_stack = true;
        required_gprs = *arg_gprs_left;
    }
    *arg_gprs_left -= required_gprs;

    if must_use_stack {
        arg.make_indirect_byval();
    } else {
        assert!(!arg.is_indirect());
        assert!(required_gprs <= num_gprs);
        if size < 32 && (!arg.layout.is_aggregate() && !matches!(arg.layout.abi, Abi::Vector { .. })) {
            arg.extend_integer_width_to(32);
        } else if align == 64 {
            if required_gprs * 32 != size {
                arg.pad_with(Reg::i32()); // pad argument to get correct alignment
            }
            arg.cast_to(Uniform { unit: Reg::i64(), total: Size::from_bits(size) });
        } else {
            println!("size = {size}, align = {align}, num gprs req = {required_gprs}");
            assert!(align < 64);
            // this also handles where size == 32 bits
            arg.cast_to(Uniform { unit: Reg::i32(), total: Size::from_bits(size) });
        }
    }
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

    for arg in fn_abi.args.iter_mut() {
        if arg.is_ignore() {
            continue;
        }
        classify_arg_ty(arg, &mut avail_gprs, NUM_ARG_GPRS);
    }
}