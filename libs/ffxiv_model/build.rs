use std::{io, path::Path};

use shader_builder::build_shaders;

fn main() -> io::Result<()> {
    build_shaders(Path::new("./shaders"))
}
