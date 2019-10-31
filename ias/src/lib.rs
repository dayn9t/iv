use rx_core::package_function;

pub use alarm::*;
pub use basic::*;
pub use cfg::*;
pub use sensor::*;

mod alarm;
mod basic;
mod cfg;
mod sensor;

pub mod app;
pub mod scene;
pub mod dump;
pub mod view;
pub mod adapter;

package_function!(package_info);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
