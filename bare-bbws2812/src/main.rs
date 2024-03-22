#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{clock::ClockControl, gpio::{Gpio8, Output, PushPull}, peripherals::Peripherals, prelude::*, Delay};
use esp_println::println;

fn bitbang(delay: &mut Delay, ws2812_pin : &mut Gpio8<Output<PushPull>>, r : u8, g: u8, b: u8) {
    let _ = ws2812_pin.set_low().unwrap();

    //let mut w = (r as u32) << 24 + (g as u32) << 16 + (b as u32); 
    for _ in 0..24 {

        //let t_high = if (w & 0x01) == 0x01 {800} else {450};
        //w = w.wrapping_shr(1);
        //let t_low = 1250 - t_high;

        let t_high = 450;
        let t_low = 750;

        let _ = ws2812_pin.set_high().unwrap();
        delay.delay_nanos(t_high);
        let _ = ws2812_pin.set_low().unwrap();
        delay.delay_nanos(t_low);
    }
}


#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);


    let io = esp_hal::IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led_pin = io.pins.gpio5.into_push_pull_output();
    let _ = led_pin.set_high().unwrap();

    let mut ws2812_pin = io.pins.gpio8.into_push_pull_output();
    bitbang(&mut delay, &mut ws2812_pin,0,0,0);

    let sw_pin = io.pins.gpio9.into_pull_up_input();
    
    esp_println::logger::init_logger_from_env();
    log::info!("Logger is setup");
    println!("Starting bare-bbsw2812 sample v3");


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
