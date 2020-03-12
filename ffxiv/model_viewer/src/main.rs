#[cfg(feature = "binary")]
fn main() {}

#[cfg(not(feature = "binary"))]
fn main() {}
