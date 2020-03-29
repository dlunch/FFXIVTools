use std::error::Error;

use clap::{App, Arg};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("sqpack_extractor")
        .arg(Arg::with_name("sqpack_path").takes_value(true).required(true))
        .get_matches();

    println!("{}", matches.value_of("config").unwrap());

    Ok(())
}
