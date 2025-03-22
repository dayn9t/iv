pub mod ff;
pub mod image;
pub mod meta;

pub const IV_MM_DIR: &str = env!("CARGO_MANIFEST_DIR");
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
