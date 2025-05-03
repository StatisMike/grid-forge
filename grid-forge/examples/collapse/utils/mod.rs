#![allow(dead_code)]

mod gif_subscribers;

use grid_forge::{
    r#gen::collapse::{grid::two_d::CollapsedGrid2D, two_d::CollapsibleTileGrid2D, CollapseError, CollapsedTileData, CollapsibleGrid}, id::{IdentTileDefaultBuilder, IdentifiableTileData}, two_d::{GridMap2D, TwoDim}, vis::{collection::VisCollection, ops::load_gridmap_identifiable_auto, DefaultVisPixel}
};
use image::{ImageBuffer, Rgb};
use rand::SeedableRng;
use rand_chacha::ChaChaRng;

pub use gif_subscribers::GifSingleSubscriber;

#[derive(Debug)]
pub struct ArgHelper {
    gif: bool,
    debug: bool,
    skip_position: bool,
    skip_entrophy: bool,
}

impl ArgHelper {
    pub const GIF: &'static str = "--gif";
    pub const DEBUG: &'static str = "--debug";
    pub const SKIP_POSITION: &'static str = "--skip-position";
    pub const SKIP_ENTROPHY: &'static str = "--skip-entrophy";

    pub fn gather() -> Self {
        let args = std::env::args().collect::<Vec<_>>();

        let gif = args.contains(&Self::GIF.to_owned());
        let debug = args.contains(&Self::DEBUG.to_owned());
        let skip_position = args.contains(&Self::SKIP_POSITION.to_owned());
        let skip_entrophy = args.contains(&Self::SKIP_ENTROPHY.to_owned());

        if gif && debug {
            panic!("cannot use both `--gif` and `--debug` flags at the same time");
        }

        Self {
            gif,
            debug,
            skip_position,
            skip_entrophy,
        }
    }

    pub fn gif(&self) -> bool {
        self.gif
    }

    pub fn debug(&self) -> bool {
        self.debug
    }

    pub fn skip_position(&self) -> bool {
        self.skip_position
    }

    pub fn skip_entrophy(&self) -> bool {
        self.skip_entrophy
    }
}

#[derive(Debug)]
pub struct RngHelper {
    seed: [u8; 32],
    pos: Option<u128>,
}

impl RngHelper {
    pub fn init_str(phrase: &str, fill: u8) -> Self {
        let mut seed: [u8; 32] = [fill; 32];

        for (i, byte) in phrase.as_bytes().iter().enumerate() {
            if i < 32 {
                seed[i] = *byte
            }
        }

        Self { seed, pos: None }
    }

    pub fn with_pos(mut self, pos: u128) -> Self {
        self.pos = Some(pos);
        self
    }

    pub fn print_state(rng: &ChaChaRng) {
        println!(
            "Seed: {:?}; Pos: {}, Stream: {}",
            rng.get_seed(),
            rng.get_word_pos(),
            rng.get_stream()
        )
    }
}

impl From<RngHelper> for ChaChaRng {
    fn from(value: RngHelper) -> ChaChaRng {
        let mut rng = rand_chacha::ChaChaRng::from_seed(value.seed);

        if let Some(pos) = value.pos {
            rng.set_word_pos(pos);
        }

        rng
    }
}

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
    collection: &'a mut VisCollection<DefaultVisPixel, 4, 4>,
}

impl<'a> VisGridLoaderHelper<'a> {
    pub fn new(collection: &'a mut VisCollection<DefaultVisPixel, 4, 4>) -> Self {
        Self { collection }
    }

    pub fn load_w_rotate(
        &mut self,
        paths: &[&str],
        rotations: &[VisRotate],
    ) -> Vec<GridMap2D<CollapsedTileData>> {
        let mut out = Vec::new();
        let builder = IdentTileDefaultBuilder::default();
        for path in paths {
            let image = self.load_image_grid(path);

            for rotation in rotations {
                out.push(if let Some(rotated) = rotation.rotate(&image) {
                    load_gridmap_identifiable_auto(&rotated, self.collection, &builder).unwrap()
                } else {
                    load_gridmap_identifiable_auto(&image, self.collection, &builder).unwrap()
                });
            }
        }
        out
    }

    fn load_image_grid(&self, path: &str) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        image::open(path).unwrap().into_rgb8()
    }
}

pub fn try_n_times_2d<Tile: IdentifiableTileData>(
    n: u32,
    mut f: impl FnMut() -> Result<CollapsibleTileGrid2D<Tile>, CollapseError<TwoDim>>,
) -> Result<CollapsedGrid2D, CollapseError<TwoDim>> {
    let mut current_iter = 0;
    loop {
        match f() {
            Ok(grid) => return Ok(grid.retrieve_collapsed()),
            Err(err) => {
                if current_iter == n {
                    return Err(err);
                }
                current_iter += 1;
            },
        }
    }
}
