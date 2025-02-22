
//#![no_std]
#![cfg_attr(not(test), no_std)]
#[cfg(test)]
extern crate std;


//#[cfg(not(test))]
//extern crate critical_section;
use core::ffi::c_int;
use core::sync::atomic::{compiler_fence, Ordering};
#[cfg(not(test))]
use cortex_m::{peripheral};




#[no_mangle]
pub extern "C" fn rs_main() -> !{
    loop {
    }
}

static mut tickcnt :u32 = 0;

#[no_mangle]
//#[interrupt]
pub unsafe extern "C"  fn SysTick_Handler() {
	tickcnt += 1;	
    if 0 == (tickcnt % 300) {
        cortex_m::peripheral::SCB::set_pendsv();
    }
}

#[no_mangle]
pub   extern "C" fn PendSV_Handler() {
	
}
