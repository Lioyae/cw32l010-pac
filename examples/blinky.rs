#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

use cw32l010_pac as pac;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = pac::Peripherals::take().unwrap();

    dp.sysctrl.ahben().modify(|_, w| {
        unsafe {
            w.key().bits(0x5A5A);
        }
        w.gpiob().set_bit()
    });

    dp.gpiob.analog().modify(|_, w| w.pin0().clear_bit());

    dp.gpiob.dir().modify(|_, w| w.pin0().clear_bit());

    dp.gpiob.opendrain().modify(|_, w| w.pin0().clear_bit());

    rprintln!("CW32L010 boot ok");

    let mut led_high = false;

    loop {
        if led_high {
            dp.gpiob.bsrr().write(|w| w.brr0().set_bit());
            rprintln!("PB0 low");
        } else {
            dp.gpiob.bsrr().write(|w| w.bss0().set_bit());
            rprintln!("PB0 high");
        }

        led_high = !led_high;

        delay(50_000);
    }
}

fn delay(mut n: u32) {
    while n != 0 {
        cortex_m::asm::nop();
        n -= 1;
    }
}
