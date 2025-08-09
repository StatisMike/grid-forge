pub mod option;
pub mod data;
pub mod queue;
pub mod grid;
pub mod error;
pub mod pattern;
pub mod tile;

#[macro_export]
macro_rules! __impl_debug_subscriber {
    (
        struct_name: $name:ident,
    ) => {
        /// Basic Subscriber for debugging purposes.
        ///
        /// Upon collapsing a tile, it will print the collapsed tile position, its `tile_type_id` and (if applicable) `pattern_id`.
        #[derive(Debug, Default)]
        pub struct $name {
            file: Option<File>,
        }

        impl $name {
            pub fn new(file: Option<File>) -> Self {
                Self { file }
            }
        }
    }
}
