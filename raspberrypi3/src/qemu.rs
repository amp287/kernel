// see https://developer.arm.com/docs/dui0003/b for semihosting info
// code from https://github.com/andre-richter/qemu-exit/blob/master/src/aarch64.rs

#[allow(non_upper_case_globals)]
const ADP_Stopped_ApplicationExit: u64 = 0x20026;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum QemuExitCode {
    Success = 0x0,
    Failed = 0x1,
}

pub fn qemu_exit(code: QemuExitCode) -> !{
    let block = qemu_parameter_block {
        arg0: ADP_Stopped_ApplicationExit,
        arg1: code as u64,
    };

    semihosting_sys_exit_call(&block)
}

/// The parameter block layout that is expected by QEMU.
#[repr(C)]
struct qemu_parameter_block {
    arg0: u64,
    arg1: u64,
}

/// A Semihosting call using `0x18` - `SYS_EXIT`.
///
/// If QEMU finds `ADP_Stopped_ApplicationExit` in the first parameter, it uses the second parameter
/// as exit code.
///
/// If first paraemter != `ADP_Stopped_ApplicationExit`, exit code `1` is used.
extern "C" fn semihosting_sys_exit_call(block: &qemu_parameter_block) -> ! {
    unsafe {
        // move block address into x1 before we overwrite x0/w0 
        asm!(
            "mov x1, {}",
            "mov w0, 0x18",
            "hlt #0xF000",
            in(reg) block as *const _ as u64    
        );
    }

    // fallback in case it doesnt work (also for compilier error)
    loop {}
}