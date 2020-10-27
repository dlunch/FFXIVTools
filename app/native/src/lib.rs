// for napi-rs macros
#![allow(clippy::zero_ptr)]
#![allow(clippy::unnecessary_mut_passed)]
#![cfg(not(test))]

use napi::{register_module, CallContext, JsObject, Module, Result};
use napi_derive::js_function;

register_module!(native, init);

fn init(module: &mut Module) -> Result<()> {
    module.create_named_method("create", create)?;
    Ok(())
}

#[js_function(0)]
fn create(ctx: CallContext) -> Result<JsObject> {
    let result = ctx.env.create_object()?;

    Ok(result)
}
