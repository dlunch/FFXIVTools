// for napi-rs macros
#![allow(clippy::zero_ptr)]
#![allow(clippy::unnecessary_mut_passed)]
#![cfg(not(test))]

use napi::Result;
use napi_derive::napi;

#[napi]
fn create() -> Result<()> {
    Ok(())
}
