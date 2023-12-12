#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)] // needed for embassy_executor::main
#![feature(alloc_error_handler)]
#![allow(unused_imports, unused_variables)]

#[macro_use]
extern crate alloc;

use embassy_executor::Spawner;
use embassy_time::{Duration, Instant, Timer, TimeoutError};

// use panic as _;
use panic_rtt_target as _;

use cortex_m_rt::{exception, ExceptionFrame};
#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    rprintln!("{:#?}", ef);
    loop {}
}

// #[exception]
// unsafe fn UsageFault() {
//     loop {
//         cortex_m::asm::bkpt();
//     }
// }


pub use rtt_target::{
    rprintln, rtt_init_default, set_print_channel,
};

use embedded_alloc::Heap;

#[global_allocator]
pub static ALLOCATOR: Heap = Heap::empty();

#[alloc_error_handler]
fn oom(_layout: core::alloc::Layout) -> ! {
    // rprintln!("oom {} {}", layout.size(), layout.align());
    // panic!("oom!");
    cortex_m::interrupt::disable();
    loop {
        use ::core::sync::atomic;
        atomic::compiler_fence(atomic::Ordering::SeqCst);
    }
}

struct Lol {
    a: UnsafeCell<MaybeUninit<[u8; SIZE]>>,
}

unsafe impl Sync for Lol {}

use core::{cell::UnsafeCell, sync::atomic::{compiler_fence, Ordering}};
use ::core::mem::MaybeUninit;
const SIZE: usize = 1 * 1024;
static HEAP: Lol = Lol { a: UnsafeCell::new(MaybeUninit::uninit()) };

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Initialize the allocator BEFORE you use it
    {
        assert_eq!(core::mem::size_of_val(&HEAP), SIZE);
        unsafe { ALLOCATOR.init(HEAP.a.get() as usize, SIZE) }
    }

    // initialize embassy
    embassy_nrf::init(Default::default());

    {
        let _ = spawner.spawn(task1());

        let channels = rtt_init_default!();
        set_print_channel(channels.up.0);

        rprintln!("----------------");
    }

    let _ = spawner.spawn(task2());

    // sleep_secs(1).await;
    // rprintln!("fail now?");
    compiler_fence(Ordering::SeqCst);
    let x = alloc::boxed::Box::new([0u8; 256]);
    // rprintln!("fail now3?");
    let x = format!("{:?}", None::<usize>);
    compiler_fence(Ordering::SeqCst);
    // rprintln!("fail now2?");
    // // let x = format!("{:?}", Some(23usize));
    rprintln!("format!: {}", x);
    // rprintln!("{:?}", None::<usize>);

    // let x: Option<usize> = None;
    // debug!("test: {:?}", x);

    // You won't see this if it hard-faults.
    rprintln!("works");

}

// In the process of pruning down to this nearly minimally reproducible build
// I found that removing this enum, or the Debug trait on it, removed
// the failure.  It may be directly related to the hard fault, because note
// the pattern here: this is almost exactly like Option, which has a Debug
// derive, plus a variant with no value and another case with a value.
// This enum originally had many other variants, but removing all the "value"
// variants, or removing just the no-value variant, prevents the failure
// *even though the code to format the value is not actually called!*
#[derive(Debug, PartialEq)]
pub enum Value {
    Something(u8),
    Null,
}

pub async fn sleep_secs(v: u32) {
    Timer::after(Duration::from_secs(v as u64)).await;
}

#[cfg(not(feature = "term"))]
#[embassy_executor::task]
pub async fn task1() {
    // return;
    sleep_secs(500).await;
    compiler_fence(Ordering::SeqCst);
    // core::future::pending::<()>().await; // does not fail with this instead

    // Removing these lines shifts stuff around and usually prevents failure:
    let first = Value::Something(1);
    format!("first {:?}", first);
}

#[embassy_executor::task]
async fn task2() {
    // return;
    sleep_secs(500).await;
    compiler_fence(Ordering::SeqCst);
    // core::future::pending::<()>().await; // does not fail with this instead

    // Removing these lines shifts stuff around and usually prevents failure:
    let x = Some(23);
    rprintln!("4 test: {:?}", x);
}
