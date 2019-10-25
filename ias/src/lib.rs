//#[macro_use]
//extern crate serde_derive;

pub use alarm::*;
pub use basic::*;

mod alarm;
pub mod app;
mod basic;
pub mod dump;

/// 获取包信息
pub fn package_info() -> app::PackageInfo {
    app::PackageInfo {
        name: env!("CARGO_PKG_NAME"),
        version: env!("CARGO_PKG_VERSION"),
        authors: env!("CARGO_PKG_AUTHORS"),
        description: env!("CARGO_PKG_DESCRIPTION"),
        date: env!("VERGEN_BUILD_DATE"),
        sha_short: env!("VERGEN_SHA_SHORT"),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
