
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

pub struct TcbRw {
	pub(crate) state: TaskState,
    pub(crate) stack_top:  PtrSize,
    pub(crate) stack_addr: PtrSize,
}

pub struct TcbRo {
    pub(crate) stack_size: PtrSize,
    pub(crate) base_priority: u8,
}


pub struct MinosScheduler {
	tasks_defs: &'static[TcbRo],
	tasks_vars: &'static mut[TcbRw]
}

// https://github.com/garasubo/erkos/blob/f972248092816d7b2c084f09c6178908af9cd14c/kernel/src/macros.rs#L4

/*
macro_rules! stack_allocate {
    ($nwords:expr) => {{
        #[link_section = ".bss"]  // .uninit
        static mut STACK: [PtrSize; $nwords] = [0; $nwords];

        unsafe { &STACK[0] as *const PtrSize as PtrSize }
    }};
}
*/

/*impl TcbRw {
	pub const fn nc(stacksize: u32) -> Self {
		//let st = stack_allocate!(stacksize);
		#[link_section = ".bss"]  // .uninit
        static mut STACK: [u32; stacksize] = [0; stacksize];
        let st = unsafe { &STACK[0] as *const u32 as u32 };
		let sp = st + 4*stacksize;
		TcbRw { state: TaskState::Ready, stack_top: st, stack_addr: sp }
	}
}*/


/// realy like usize but usable in const eval
type PtrSize = u32;
macro_rules! PtrSize {
    () => { u32
        
    };
}
const USIZEBYTES : PtrSize = 4; //(usize::BITS/8) as PtrSize;

//         unsafe { &STACK[0] as *const u8 as u32 + $n }

macro_rules! new_TcbRw {
    ($nwords:expr) => {{
		#[link_section = ".bss"]  // .uninit
        static mut STACK: [PtrSize; $nwords] = [0; $nwords];
        let st  = unsafe { &STACK[0] as *const PtrSize as PtrSize };
        let sp  = st + USIZEBYTES*$nwords;
        TcbRw { state: TaskState::Ready, stack_top: st, stack_addr: sp }
	}};
}
//macro_rules! count {
//    () => (0usize);
//    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
//}

#[macro_export]
macro_rules! Minos_Tasks {
    ( $num:expr, $( < $name:ident, $prio:expr, $stacksize:tt > ),*) => {{
 		const DEFS : [TcbRo; $num] = [  $( TcbRo{stack_size:$stacksize, base_priority:$prio} ),* ];
		static mut vars : [TcbRw; $num] = [ $( new_TcbRw!($stacksize) ),* ];
		let pvars  = unsafe { &mut vars};
		MinosScheduler { tasks_defs: &DEFS, tasks_vars: pvars }}
    };
}

pub fn testme () -> MinosScheduler {
	let s = Minos_Tasks!(2, <t1, 3, 1024> ,  <t2, 4, 1024>);
    s
}


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

pub fn minos_wakeup(_tasknum:u8) {
}


