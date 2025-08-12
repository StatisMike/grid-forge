#[macro_export]
macro_rules! gf_asset_path {
    ($path:expr) => {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("assets")
            .join($path)
    };
}