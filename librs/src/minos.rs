
//#![no_std]
//#![cfg_attr(not(test), no_std)]

#[cfg(test)]
extern crate std;


//#[cfg(not(test))]
//extern crate critical_section;
//use core::ffi::c_int;
use core::sync::atomic::{compiler_fence, Ordering};
#[cfg(not(test))]
use cortex_m::{peripheral};




#[no_mangle]
pub extern "C" fn rs_main() -> !{
    loop {
    }
}

static mut TICKCNT :u32 = 0;

#[no_mangle]
//#[interrupt]
//#[exception]
pub unsafe extern "C"  fn SysTick_Handler() {
	TICKCNT += 1;	
    if 0 == (TICKCNT % 300) {
        cortex_m::peripheral::SCB::set_pendsv();
    }
}

pub unsafe extern "C"  fn minos_yield() {
	
        cortex_m::peripheral::SCB::set_pendsv();
        /* it is unclear if a barrier is actually needed here
         * (spec for single core cortex-m)
         * most RTOS/Scheduler put it while saying it's not mandatory 
         * so we'll copy FreeRTOS here : "Barriers are normally not required but do ensure the code is 
         * completely within the specified behaviour for the architecture"
         */
        cortex_m::asm::dsb();
	    cortex_m::asm::isb();
	    // or should we use compiler fence ??
}

// https://github.com/mychenkaikai/neon-rtos/blob/master/src/kernel/task/tcb.rs
//#[repr(u16)]
enum TaskState {
    Ready,
    Running,
    Blocked //(BlockReason),
}

// TCB is in two part, one r/o which can
// go in flash, and one rw in RAM.

pub struct TCB_rw {
    pub(crate) stack_top:  usize,
    pub(crate) stack_addr: usize,
}

pub struct TCB_ro {
    pub(crate) state: TaskState,
    pub(crate) stack_size: usize,
    pub(crate) base_priority: u8,
}

pub struct Minos_Scheduler {
	tasks_defs: &'static[TCB_ro],
	tasks_vars: &'static mut[TCB_rw]
}


#[macro_export]
macro_rules! Minos_Tasks {
    ( $( < $name:ident, $prio:expr, $stacksize:tt > ),*) => {
       static TOTO :u32 = 0;
    };
}

Minos_Tasks!(<t1, 3, 1024>
          ,  <t2, 4, 1024>);

#[no_mangle]
pub   extern "C" fn PendSV_Handler() {
	
}

//             concat_idents!(minos, $name)($($arg_name),*)

use paste::paste;

#[macro_export]
macro_rules! syscall {
    ($name:ident($($arg_name:ident : $arg_type:ty),*)) => {
        pub fn $name($($arg_name: $arg_type),*) {
            paste::paste!{
                [<minos_ $name>]($($arg_name),*);
            }
        }
    };
}

// Use the macro to define the function

syscall!(wait_notif(l: u32));
syscall!(send_notif(tasknum:u8, l: u32));
syscall!(wait());
syscall!(wakeup(tasknum:u8));

pub fn minos_wait_notif(_l :u32) {
}

pub fn minos_send_notif(_t:u8, _l :u32) {
}


pub fn minos_wait() {
}

pub fn minos_wakeup(tasknum:u8) {
}


