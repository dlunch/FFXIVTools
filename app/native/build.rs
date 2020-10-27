fn main() {
    #[cfg(feature = "electron")]
    std::env::set_var("npm_config_runtime", "electron");

    napi_build::setup();
}
