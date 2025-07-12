//! This example shows general implementation of the `vis` feature, which allows generating image out of created `GridMap`.
//! This is very useful in development state, as before creating maps out of final desired GridTile it is best to test
//! out the algorithms used, but is rarely useful in final build.
//!
//! Most examples use the `vis` feature to present visual representation of two dimensional [GridMap].

use grid_forge::{two_d::*, vis::{grid::init_map_image_buffer, tile::{TilePixConst, WithPixels}}, TileData};

use image::{imageops, Rgb};
use rand::{Rng, SeedableRng};

// Enum holding the easily discernable colors for the resulting tiles.
enum TileColor {
    Blue,
    Green,
}

impl TileColor {

    fn rgb(&self) -> Rgb::<u8> {
        match self {
            TileColor::Blue => Rgb::<u8>::from([52, 119, 235]),
            TileColor::Green => Rgb::<u8>::from([128, 235, 52]),
        }
    }
}

// GridTile struct besides required GridPos2D holds also the created enum.
struct TwoColoredTile {
    pixels: TilePixConst<1 ,1, Rgb<u8>>,
}

impl TwoColoredTile {
    fn new(color: TileColor) -> Self {
        Self {
            pixels: TilePixConst::from_slice(&[color.rgb()]),
        }
    }
}

impl TileData for TwoColoredTile {}

// Trait necessary
impl WithPixels<TilePixConst<1 ,1, Rgb<u8>>, Rgb<u8>> for TwoColoredTile {
    fn tile_pixels(&self) -> &TilePixConst<1 ,1, Rgb<u8>> {
        &self.pixels
    }

    fn tile_pixels_mut(&mut self) -> &mut TilePixConst<1 ,1, Rgb<u8>> {
        &mut self.pixels
    }
}

fn main() {
    // Seed for reproductability.
    let mut seed: [u8; 32] = [0; 32];

    for (i, byte) in "vis_example".as_bytes().iter().enumerate() {
        if i < 31 {
            seed[i] = *byte;
        }
    }
    let mut rng = rand_chacha::ChaChaRng::from_seed(seed);

    // Create an empty GridMap...
    let size = GridSize2D::new(100, 100);
    let mut map = GridMap2D::<TwoColoredTile>::new(size);

    // and fill it with colors at random.
    for pos in map.size().get_all_possible_positions() {
        let color = if rng.gen_bool(0.5) {
            TileColor::Blue
        } else {
            TileColor::Green
        };
        map.insert_data(&pos, TwoColoredTile::new(color));
    }

    // Create image and save it in examples dir.
    let mut image = init_map_image_buffer::<Rgb<u8>>(&size, (1,1));
    map.write_to_image_const(&mut image).unwrap();

    let image = imageops::resize(
        &image,
        map.size().x() * 5,
        map.size().y() * 5,
        imageops::FilterType::Nearest,
    );
    image
        .save(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/examples/outputs/vis_example.png"
        ))
        .unwrap();
}
