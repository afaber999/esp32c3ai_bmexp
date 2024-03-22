#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay};
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);


    let io = esp_hal::IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led_pin = io.pins.gpio5.into_push_pull_output();
    let _ = led_pin.set_high().unwrap();

    let sw_pin = io.pins.gpio9.into_pull_up_input();
    
    esp_println::logger::init_logger_from_env();
    log::info!("Logger is setup");
    println!("Starting bare-blink samples, fast blink when boot switch is pressed");


    let mut idx = 0;
    loop {
        idx += 1;
        
        let sw_pressed = sw_pin.is_low().unwrap();

        if sw_pressed {
            println!("Fast blink, loop {idx}");
            let _ = led_pin.toggle().unwrap();
            delay.delay_ms(100u32);
            let _ = led_pin.toggle().unwrap();
            delay.delay_ms(100u32);            
        } else {
            println!("Slow blink, loop {idx}");
            let _ = led_pin.toggle().unwrap();
            delay.delay_ms(500u32);
        }

    }
}
