#![no_std]
#![deny(warnings)]

use core::cell::UnsafeCell;

#[repr(transparent)]
pub struct SifiveTestDevice(UnsafeCell<u64>);

const FAIL: u16 = 0x3333;
const PASS: u16 = 0x5555;
const RESET: u16 = 0x7777;

impl SifiveTestDevice {
    #[inline]
    pub fn fail(&self, code: u16) -> ! {
        self.write(FAIL as u64 | (code as u64) << 16)
    }

    #[inline]
    pub fn pass(&self) -> ! {
        self.write(PASS as _)
    }

    #[inline]
    pub fn reset(&self) -> ! {
        self.write(RESET as _)
    }

    #[inline]
    fn write(&self, bits: u64) -> ! {
        unsafe { self.0.get().write_volatile(bits) };
        unreachable!()
    }
}
