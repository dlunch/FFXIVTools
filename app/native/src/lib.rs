// for napi-rs macros
#![allow(clippy::zero_ptr)]
#![allow(clippy::unnecessary_mut_passed)]
#![cfg(not(test))]

use napi::{CallContext, JsObject, Result};
use napi_derive::{js_function, module_exports};

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
    exports.create_named_method("create", create)?;
    Ok(())
}

#[js_function(0)]
fn create(ctx: CallContext) -> Result<JsObject> {
    let result = ctx.env.create_object()?;

    Ok(result)
}
