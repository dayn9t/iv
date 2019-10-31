use rx_core::package_function;

pub use basic::*;
pub use cfg::*;
pub use node::*;
pub use sensor::*;

mod basic;
mod cfg;
mod node;
mod sensor;

pub mod adapter;
pub mod app;
pub mod dump;
pub mod scene;
pub mod view;

package_function!(package_info);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
