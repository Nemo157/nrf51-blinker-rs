#![no_std]
#![feature(conservative_impl_trait)]
#![feature(never_type)]
#![feature(generators)]
#![feature(proc_macro)]

extern crate futures_await as futures;
extern crate futures_hal as hal;

use core::time::Duration;

use futures::prelude::{async, await};
use futures::future::{self, Either};
use futures::{FutureExt, StreamExt};
use hal::{Cancellable, Periodic, DetectingInputPin, OutputPin, Event};

#[async]
#[allow(unreachable_code, unreachable_patterns)]
pub fn main<'a, Timer, Led, Button>(
    mut timer: Timer,
    mut led: Led,
    button: Button) -> Result<!, !>
where
    Timer: Periodic + 'a,
    Led: OutputPin + 'a,
    Button: DetectingInputPin + 'a,
{
    let timeout = Duration::from_secs(3);
    let debounce = Duration::from_millis(200);

    let mut detector = button.detect(Event::HighToLow).into_future();

    loop {
        // If button is pushed, or timeout has passed
        match await!(timer.start(timeout).select(detector)) {
            Ok(Either::Left((timer_, detector_))) => {
                // Toggle LED
                if led.is_high() {
                    led.set_low();
                } else {
                    led.set_high();
                }

                // Timer was triggered, no additional action needed
                timer = timer_;
                detector = detector_;
            },
            Ok(Either::Right(((_, detector_), timer_))) => {
                // Toggle LED
                if led.is_high() {
                    led.set_low();
                } else {
                    led.set_high();
                }

                // Button was triggered, debounce the button by ignoring events
                // for debounce period.
                let debounce_timer = timer_.cancel().start(debounce);
                let ignored_detector = detector_.skip_while(|_| future::ok(true)).into_future();
                match await!(debounce_timer.select(ignored_detector)) {
                    Ok(Either::Left((timer_, detector_))) => {
                        timer = timer_;
                        // Extract the actual detector from the `StreamFuture<SkipWhile<D>>`
                        detector = detector_.into_inner().unwrap().into_inner().into_future();
                    }
                    Ok(Either::Right(_)) => {
                        unreachable!("Ignored button should never return");
                    }
                }
            },
        };
    }
}
