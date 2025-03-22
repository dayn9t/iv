mod camera;

pub use camera::*;
pub const IV_CAMERA_DIR: &str = env!("CARGO_MANIFEST_DIR");
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
