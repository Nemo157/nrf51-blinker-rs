#![no_std]

extern crate wfe_executor;
extern crate nrf51;
extern crate nrf51_hal;

extern crate nrf51_blinker;

use wfe_executor::Executor;
use nrf51_hal::{timer::Timer, gpio::GpioExt};

pub fn main() {
    let core = nrf51::CorePeripherals::take().unwrap();
    let peripherals = nrf51::Peripherals::take().unwrap();

    let gpio = peripherals.GPIO.split();

    let timer = Timer::new(peripherals.TIMER0);
    let led = gpio.pin21.into_open_drain_output();
    let button = gpio.pin15.into_pull_up_input();

    let future = nrf51_blinker::main(timer, led, button);

    Executor::new(core).run_stable(future);
}
