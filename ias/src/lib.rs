pub use alarm::*;
pub use basic::*;

mod alarm;
mod basic;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
