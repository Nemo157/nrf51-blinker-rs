#![no_std]

#![feature(never_type)]

extern crate wfe_executor;
extern crate nrf51;
extern crate nrf51_hal;
extern crate nb;
extern crate embedded_hal;

extern crate nrf51_blinker;

use wfe_executor::Executor;
use nrf51_hal::{gpio::GpioExt, serial::{Serial, BAUDRATEW}};
use nb::io::{Read, Write};
use embedded_hal::serial::io::{reader, writer};

pub fn main() {
    let core = nrf51::CorePeripherals::take().unwrap();
    let peripherals = nrf51::Peripherals::take().unwrap();

    let gpio = peripherals.GPIO.split();

    let uart = Serial::uart0(
        peripherals.UART0,
        gpio.pin9.into_push_pull_output().downgrade(),
        gpio.pin11.into_floating_input().downgrade(),
        BAUDRATEW::BAUD1M);

    let (tx, rx) = uart.split();
    let future = nrf51_blinker::main(writer(tx).as_async(), reader(rx).as_async());

    match Executor::new(core).run_stable(future) {
        Err(err) => {
            panic!("{:?}", err);
        }
    }
}
