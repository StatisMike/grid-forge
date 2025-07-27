use grid_forge::{core::GridMap2D, id::{BasicTypedData, IdentTileDefaultBuilder, TypeIdMap}, image::{ops::load_from_image_const_typed_auto, TilePixConst}};
use image::{ImageBuffer, Rgb};

pub enum VisRotate {
    None,
    R90,
    R180,
    R270,
}

impl VisRotate {
    pub fn rotate(
        &self,
        buffer: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    ) -> Option<ImageBuffer<Rgb<u8>, Vec<u8>>> {
        match self {
            VisRotate::None => None,
            VisRotate::R90 => Some(image::imageops::rotate90(buffer)),
            VisRotate::R180 => Some(image::imageops::rotate180(buffer)),
            VisRotate::R270 => Some(image::imageops::rotate270(buffer)),
        }
    }
}

pub struct VisGridLoaderHelper<'a> {
    collection: &'a mut TypeIdMap<TilePixConst<4, 4, Rgb<u8>>>,
}

impl<'a> VisGridLoaderHelper<'a> {
    pub fn new(collection: &'a mut TypeIdMap<TilePixConst<4, 4, Rgb<u8>>>) -> Self {
        Self { collection }
    }

    pub fn load_w_rotate(
        &mut self,
        paths: &[&str],
        rotations: &[VisRotate],
    ) -> Vec<GridMap2D<BasicTypedData>> {
        let mut out = Vec::new();
        let builder = IdentTileDefaultBuilder::default();
        for path in paths {
            let image = self.load_image_grid(path);

            for rotation in rotations {
                out.push(if let Some(rotated) = rotation.rotate(&image) {
                    load_from_image_const_typed_auto(&rotated, &builder, self.collection).unwrap()
                } else {
                    load_from_image_const_typed_auto(&image, &builder, self.collection).unwrap()
                });
            }
        }
        out
    }

    fn load_image_grid(&self, path: &str) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        image::open(path).unwrap().into_rgb8()
    }
}