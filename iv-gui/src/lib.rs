pub mod image_win;

pub const PACKAGE_DIR: &str = env!("CARGO_MANIFEST_DIR");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
