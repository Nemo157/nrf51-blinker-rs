#![no_std]
#![feature(conservative_impl_trait)]
#![feature(never_type)]
#![feature(generators)]
#![feature(proc_macro)]

extern crate futures_await as futures;

use futures::prelude::{async, await};
use futures::future::Either;
use futures::io::{AsyncReadExt, CoreAsyncRead, CoreAsyncWrite};

#[async]
#[allow(unreachable_code, unreachable_patterns)]
pub fn main<'a, Tx, Rx>(
    mut tx: Tx,
    mut rx: Rx) -> Result<!, Either<Rx::Error, Tx::Error>>
where
    Tx: CoreAsyncWrite + 'a,
    Rx: CoreAsyncRead + 'a,
{
    loop {
        let (rx, tx) = (&mut rx, &mut tx);
        await!(rx.copy_into_with_buffer(tx, [0; 5]))?;
    }
}
