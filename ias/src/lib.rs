pub use alarm::*;
pub use basic::*;

mod alarm;
pub mod app;
mod basic;
pub mod dump;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}