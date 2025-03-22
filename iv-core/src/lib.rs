use std::path::PathBuf;

pub mod basic;
pub mod geo;

pub fn get_crate_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", get_crate_dir().display());
    }
}
