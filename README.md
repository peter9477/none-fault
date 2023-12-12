# None-fault

This repo is a close to minimal crate to reproduce a hard-fault that
occurs on cortex-m4 (thumbv7em-none-eabihf and likely others)
with any Rust nightly *later than* 2023-08-08 (tested on 2023-08-09
and 2023-12-10 and some others in between).

It appears possible that this is related specifically to the Debug
derive on Option, and occurs only when attempting to format! an
Option::None (or probably write! or similar routines).

In addition to the hard fault, it has been possible by adjusting
the code (in the process of reducing this down) to cause a panic
in rtt-target's print routine where the call to critical-section's
borrow_ref_mut() says the RefCell is already borrowed, which should
of course not be possible.  I suspect this is a sign that the
problem relates to memory corruption of some kind, possibly stomping
on the RefCell's borrow counter.

# Reproducing the failure

I've been running on nRF52840 hardware, though there's nothing left
in the code that is specific to the 52840.

    cargo +nightly-2023-08-09-x86_64-unknown-linux-gnu build
    rust-objcopy target/thumbv7em-none-eabihf/debug/none-fault -O ihex none-fault.hex
    probe-rs download --chip nrf52840 --format hex none-fault.hex
    probe-rs reset --chip nrf52840
    RTT=$(grep _SEGGER_RTT$ build.map | cut -d' ' -f1); echo $RTT >.rttlast
    rtthost --chip nrf52840 --scan-region 0x$RTT

Produces this output:

    Attaching to RTT...
    Found control block at 0x200008e4
    ----------------
    fail now?
    ExceptionFrame {
        r0: 0x00000000,
        r1: 0x000038ed,
        r2: 0x200000e4,
        r3: 0x200000e4,
        r12: 0x000003f8,
        lr: 0x00001169,
        pc: 0x200000e4,
        xpsr: 0x40000000,
    }

Doing exactly the same but with nightly 2023-08-08 gives this output:

    Attaching to RTT...
    Found control block at 0x200008e4
    ----------------
    fail now?
    format!: None
    works
