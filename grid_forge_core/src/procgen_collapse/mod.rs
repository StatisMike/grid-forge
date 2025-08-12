pub mod data;
pub mod error;
pub mod grid;
pub mod option;
pub mod pattern;
pub mod queue;
pub mod tile;

#[macro_export]
#[doc(hidden)]
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
    };
}

#[macro_export]
macro_rules! __impl_collapse_history_subscriber {
    (
        struct_name: $name:ident,
        history_item_name: $history_item:ident,
        position: $position:ty,
    ) => {
        /// Event in the history of tile generation process, containing the position of the tile alongside its collapsed
        /// `tile_type_id` and `pattern_id` (if applicable).
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[derive(Debug, Clone)]
        pub struct $history_item {
            pub position: $position,
            pub tile_type_id: u64,
            #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
            pub pattern_id: Option<u64>,
        }

        /// Simple subscriber to collect history of tile generation process.
        ///
        /// Every new generation began by the resolver will clear the history.
        #[derive(Debug, Clone, Default)]
        pub struct $name {
            history: Vec<$history_item>,
        }

        impl $name {
            /// Returns history of tile generation process.
            pub fn history(&self) -> &[$history_item] {
                &self.history
            }
        }
    };
}
