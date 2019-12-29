#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, prelude::*, Delay, Leds};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

    let ms = 50_u8;
    loop {
            for inc in 0..8 {
                let mut next = inc + 1;
                if inc == 7{
                    next=0;
                   }
                   leds[next].on();
                   delay.delay_ms(ms);
                   leds[inc].off();
                   delay.delay_ms(ms);
            }
        }
}


