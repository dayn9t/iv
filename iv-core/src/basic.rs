#[macro_export]
macro_rules! valid_types {
    ($trait: ident, $($rust_type: ty),+) => {
        /// This sealed trait is implemented for types that are valid to use in corresponding context
        pub trait $trait: ::num::traits::NumAssign + PartialOrd + Default + Copy + private::Sealed {}

        mod private {
            pub trait Sealed {}
        }

        $(
            impl $trait for $rust_type {}
            impl private::Sealed for $rust_type {}
        )+
    };
}
