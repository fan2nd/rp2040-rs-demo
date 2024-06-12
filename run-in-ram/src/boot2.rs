use core::arch::global_asm;

macro_rules! cfg_global_asm {
    {@inner, [$($x:tt)*], } => {
        global_asm!{$($x)*}
    };
    (@inner, [$($x:tt)*], #[cfg($meta:meta)] $asm:literal, $($rest:tt)*) => {
        #[cfg($meta)]
        cfg_global_asm!{@inner, [$($x)* $asm,], $($rest)*}
        #[cfg(not($meta))]
        cfg_global_asm!{@inner, [$($x)*], $($rest)*}
    };
    {@inner, [$($x:tt)*], $asm:literal, $($rest:tt)*} => {
        cfg_global_asm!{@inner, [$($x)* $asm,], $($rest)*}
    };
    {$($asms:tt)*} => {
        cfg_global_asm!{@inner, [], $($asms)*}
    };
}

cfg_global_asm! {
    ".cfi_sections .debug_frame
     .section .ram_run_head, \"ax\"
     .global RamRun
     .type RamRun,%function
     .thumb_func",
    ".cfi_startproc
     RamRun:",

    "ldr r0, =_stack_start
     msr msp, r0",

    "ldr r0, =0xe000ed08
     ldr r1, =__vector_table
     str r1, [r0]",

    "bl Reset
     udf #0",

    ".cfi_endproc
     .size RamRun, . - RamRun",
}