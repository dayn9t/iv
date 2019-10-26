use rx_core::package_function;

pub use alarm::*;
pub use basic::*;
pub use cfg::*;

mod alarm;
mod basic;
mod cfg;

pub mod app;
pub mod dump;
pub mod view;

package_function!(package_info);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
