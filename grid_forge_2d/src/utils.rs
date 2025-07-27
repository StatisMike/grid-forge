macro_rules! asset_path {
    ($path:expr) => {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("assets")
            .join($path)
    };
}

pub(crate) use asset_path;
