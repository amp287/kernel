use core::ptr::write_volatile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn qemu_exit(code: QemuExitCode) {
    unsafe {
        write_volatile(0xf4 as *mut u32, code as u32);
    }
}