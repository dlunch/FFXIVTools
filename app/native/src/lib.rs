// for napi-rs macros
#![allow(clippy::zero_ptr)]
#![allow(clippy::unnecessary_mut_passed)]
#![cfg(not(test))]

use std::convert::TryInto;

use napi::{register_module, CallContext, Env, JsNumber, JsObject, Module, Result, Task};
use napi_derive::js_function;

register_module!(example, init);

struct AsyncTask(u32);

impl Task for AsyncTask {
    type Output = u32;
    type JsValue = JsNumber;

    fn compute(&mut self) -> Result<Self::Output> {
        use std::thread::sleep;
        use std::time::Duration;
        sleep(Duration::from_millis(self.0 as u64));
        Ok(self.0 * 2)
    }

    fn resolve(&self, env: &mut Env, output: Self::Output) -> Result<Self::JsValue> {
        env.create_uint32(output)
    }
}

fn init(module: &mut Module) -> Result<()> {
    module.create_named_method("sync", sync_fn)?;

    module.create_named_method("sleep", sleep)?;
    Ok(())
}

#[js_function(1)]
fn sync_fn(ctx: CallContext) -> Result<JsNumber> {
    let argument: u32 = ctx.get::<JsNumber>(0)?.try_into()?;

    ctx.env.create_uint32(argument + 100)
}

#[js_function(1)]
fn sleep(ctx: CallContext) -> Result<JsObject> {
    let argument: u32 = ctx.get::<JsNumber>(0)?.try_into()?;
    let task = AsyncTask(argument);
    ctx.env.spawn(task)
}
