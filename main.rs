#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern crate libc;

include!(concat!(env!("OUT_DIR"), "/bindings.rs")); 

use core::panic::PanicInfo;
use cortex_m_rt::entry;

use defmt_rtt as _;
use panic_probe as _;

// The panic handler function
/*
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
*/
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

const GPIOC: *mut GPIO_TypeDef = GPIOC_BASE as *mut GPIO_TypeDef;
const GPIO_PIN_6: u16 = 0x0040;



#[entry]
fn main() -> ! {
    unsafe {
        // Initialize the HAL Library
        HAL_Init();
        
        // Configure the system clock
        //SystemClock_Config();
        
        // Initialize all configured peripherals
        //MX_GPIO_Init();
           
        
        // Blink the LED in a loop
        loop {
            // Assuming the LED is connected to pin 0 on port B
            HAL_GPIO_TogglePin(GPIOC, GPIO_PIN_6);
            defmt::println!("Output pin is low.");
            HAL_Delay(1000);  // Delay for 1 second
            HAL_GPIO_TogglePin(GPIOC, GPIO_PIN_6);
            HAL_Delay(1000);  // Delay for 1 second
        }
    }
}
