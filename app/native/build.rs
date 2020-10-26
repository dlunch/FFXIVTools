use std::env;

fn main() {
    env::set_var("npm_config_runtime", "electron");

    napi_build::setup();
}
