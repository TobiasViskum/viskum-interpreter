macro_rules! def_op {
    ($name:ident($instruction:ident, $vm:ident) $code:block) => {
        #[inline(always)]
        pub unsafe fn $name($instruction: *const OptimizedInstruction, $vm: &mut VM) {
            $code
        }
    };
    ($name:ident($instruction:ident, $vm:ident) $code:block) => {
        #[inline(always)]
        pub unsafe fn $name($instruction: *const OptimizedInstruction, $vm: &mut VM) {
            $code
        }
    };
}

macro_rules! get_rel_reg {
    ($vm:ident, $instruction:ident.$reg:ident) => {
        *{
            let register_offset = $vm.read_register_offset();
            ($vm.registers.get_unchecked((*$instruction).$reg as usize + register_offset))
        }
    };
    (mut $vm:ident, $instruction:ident.$reg:ident) => {
        *{
            let register_offset = $vm.read_register_offset();
            ($vm.registers.get_unchecked_mut((*$instruction).$reg as usize + register_offset))
        }
    };
}

macro_rules! get_abs_reg {
    ($vm:ident, $instruction:ident.$reg:ident) => {
        *{
            ($vm.registers.get_unchecked((*$instruction).$reg as usize))
        }
    };
}

macro_rules! next_op {
    ($instruction:expr, $vm:ident) => {
        ((*($instruction)).handler)($instruction, $vm)
    };
}

macro_rules! new_mod {
    (mod_name: $mod_name:ident, $($code:item)*) => {
        pub mod $mod_name {
            use crate::vm::{ OptimizedInstruction, VM };

            $($code)*
        }
    };
}

new_mod! {
    mod_name: util_fn_ptrs,

    def_op!(halt(_instruction, _vm) {
        // println!("Program done");
    });

    def_op!(goto(instruction, vm) {
        next_op!((*instruction).param1.jmp, vm)
    });

    def_op!(push_fn_ptr(instruction, vm) {
        next_op!((*instruction).param2.jmp, vm);
    });
}

new_mod! {
    mod_name: unary_fn_ptrs,

    macro_rules! unary_op_body {
        ($vm:ident, $instruction:ident, $code:block, $rhs:ident, $method:ident) => {
            let $rhs = $method!($vm, $instruction.reg2);
            get_rel_reg!(mut $vm, $instruction.reg1) = $code;
            next_op!($instruction.offset(1), $vm)
        }
    }

    macro_rules! def_unary_op {
        ($name:ident, |$rhs:ident| $code:block) => {
            paste::paste! {
                def_op!([<$name _r>](instruction, vm) {
                    unary_op_body!(vm, instruction, $code, $rhs, get_rel_reg);
                });
                def_op!([<$name _a>](instruction, vm) {
                    unary_op_body!(vm, instruction, $code, $rhs, get_abs_reg);
                });
            }
        }
    }

    def_unary_op!(neg_int, |rhs| {
        -rhs
    });

    def_unary_op!(not_int, |rhs| {
        (!rhs) as i64
    });

    def_unary_op!(copy, |rhs| {
        rhs
    });
}

new_mod!(
    mod_name: jmp_cmp_fn_ptrs,

    macro_rules! jmp_cmp_op_body {
        ($vm:ident, $instruction:ident, $code:block, $lhs:ident, $rhs:ident, $method1:ident, $method2:ident) => {
            let ($lhs, $rhs) = (get_rel_reg!($vm, $instruction.reg1), get_rel_reg!($vm, $instruction.reg2));
            // This branchless is apparently no faster that the branch with the match
            // let mask = !-($code as i64);
            // let ptr1 = (*$instruction).param1.jmp;
            // let ptr2 = (*$instruction).param2.jmp;
            // let target_ptr = (((ptr1 as i64) & !mask) | ((ptr2 as i64) & mask)) as *const OptimizedInstruction;
            // next_op!(target_ptr, $vm);

            // Here is the match
            match $code {
                true => next_op!((*$instruction).param1.jmp, $vm),
                false => next_op!((*$instruction).param2.jmp, $vm)
            }
        }
    }

    macro_rules! def_jmp_cmp_op {
        ($name:ident, |$lhs:ident, $rhs:ident| $code:block) => {
            paste::paste! {
                def_op!([<$name _rr>](instruction, vm) {
                    jmp_cmp_op_body!(vm, instruction, $code, $lhs, $rhs, get_rel_reg, get_rel_reg);
                });
                def_op!([<$name _ra>](instruction, vm) {
                    jmp_cmp_op_body!(vm, instruction, $code, $lhs, $rhs, get_rel_reg, get_abs_reg);
                });
                def_op!([<$name _ar>](instruction, vm) {
                    jmp_cmp_op_body!(vm, instruction, $code, $lhs, $rhs, get_abs_reg, get_rel_reg);
                });
                def_op!([<$name _aa>](instruction, vm) {
                    jmp_cmp_op_body!(vm, instruction, $code, $lhs, $rhs, get_abs_reg, get_abs_reg);
                });
            }
        }
    }

    def_jmp_cmp_op!(jmp_cmp_eq_int, |lhs, rhs| {
        lhs == rhs
    });

    def_jmp_cmp_op!(jmp_cmp_ne_int, |lhs, rhs| {
        lhs != rhs
    });

    def_jmp_cmp_op!(jmp_cmp_ge_int, |lhs, rhs| {
        lhs >= rhs
    });

    def_jmp_cmp_op!(jmp_cmp_gt_int, |lhs, rhs| {
        lhs > rhs
    });

    def_jmp_cmp_op!(jmp_cmp_le_int, |lhs, rhs| {
        lhs <= rhs
    });

    def_jmp_cmp_op!(jmp_cmp_lt_int, |lhs, rhs| {
        lhs < rhs
    });
);

new_mod! {
    mod_name: binary_fn_ptrs,

    macro_rules! binary_op_body {
        ($vm:ident, $instruction:ident, $code:block, $lhs:ident, $rhs:ident, $method1:ident, $method2:ident) => {
            let ($lhs, $rhs) = ($method1!($vm, $instruction.reg2), $method2!($vm, $instruction.reg3));
            get_rel_reg!(mut $vm, $instruction.reg1) = $code;
            next_op!($instruction.offset(1), $vm)
        }
    }

    macro_rules! def_binary_op {
        ($name:ident, |$lhs:ident, $rhs:ident| $code:block) => {
            paste::paste! {
                def_op!([<$name _rr>](instruction, vm) {
                    binary_op_body!(vm, instruction, $code, $lhs, $rhs, get_rel_reg, get_rel_reg);
                });
                def_op!([<$name _ra>](instruction, vm) {
                    binary_op_body!(vm, instruction, $code, $lhs, $rhs, get_rel_reg, get_abs_reg);
                });
                def_op!([<$name _ar>](instruction, vm) {
                    binary_op_body!(vm, instruction, $code, $lhs, $rhs, get_abs_reg, get_rel_reg);
                });
                def_op!([<$name _aa>](instruction, vm) {
                    binary_op_body!(vm, instruction, $code, $lhs, $rhs, get_abs_reg, get_abs_reg);
                });
            }
        }
    }

    def_binary_op!(add_int, |lhs, rhs| {
        lhs + rhs
    });
    def_binary_op!(sub_int, |lhs, rhs| {
        lhs - rhs
    });
    def_binary_op!(mul_int, |lhs, rhs| {
        lhs * rhs
    });
    def_binary_op!(div_int, |lhs, rhs| {
        lhs / rhs
    });


    new_mod! {
        mod_name: comparison_fn_ptrs,
    
        def_binary_op!(cmp_eq_int, |lhs, rhs| {
            (lhs == rhs) as i64
        });
        def_binary_op!(cmp_ne_int, |lhs, rhs| {
            (lhs != rhs) as i64
        });
        def_binary_op!(cmp_ge_int, |lhs, rhs| {
            (lhs >= rhs) as i64
        });
        def_binary_op!(cmp_gt_int, |lhs, rhs| {
            (lhs > rhs) as i64
        });
        def_binary_op!(cmp_le_int, |lhs, rhs| {
            (lhs <= rhs) as i64
        });
        def_binary_op!(cmp_lt_int, |lhs, rhs| {
            (lhs < rhs) as i64
        });
      
    }
}
