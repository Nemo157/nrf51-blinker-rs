#![no_std]
#![feature(conservative_impl_trait)]
#![feature(never_type)]
#![feature(generators)]
#![feature(proc_macro)]

extern crate futures_await as futures;

use futures::prelude::{async, await};
use futures::io::{CoreAsyncRead, CoreAsyncReadExt, CoreAsyncWrite, CoreAsyncWriteExt};

#[derive(Debug)]
pub enum Error<TxError, RxError> {
    Tx(TxError),
    Rx(RxError),
}

#[async]
#[allow(unreachable_code, unreachable_patterns)]
pub fn main<'a, Tx, Rx>(
    mut tx: Tx,
    mut rx: Rx) -> Result<!, Error<Tx::Error, Rx::Error>>
where
    Tx: CoreAsyncWrite + 'a,
    Rx: CoreAsyncRead + 'a,
{
    loop {
        let (_, bytes) = await!((&mut rx).read_exact([0; 5])).map_err(Error::Rx)?;
        let (tx_, _) = await!(tx.write_all(bytes)).map_err(Error::Tx)?;
        tx = tx_;
    }
}
