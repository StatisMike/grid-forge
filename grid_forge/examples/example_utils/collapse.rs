use std::fs::File;

use grid_forge::{core::{GridPosition2D, GridSize2D}, id::{TypeIdMap, TypedData}, image::{ops::{init_map_image_buffer, write_tile}, TilePixConst}, procgen_collapse::{singular::{CollapsibleTileGrid2D, TileSubscriber2D}, CollapseError2D, CollapsedGrid2D}};
use grid_forge_2d::procgen_collapse::pattern::{CollapsiblePatternGrid2D, Pattern2DSubscriber};
use image::{ImageBuffer, Rgb};

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

pub fn try_n_times_2d<Tile: TypedData>(
    n: u32,
    mut f: impl FnMut() -> Result<CollapsibleTileGrid2D<Tile>, CollapseError2D>,
) -> Result<CollapsedGrid2D, CollapseError2D> {
    let mut current_iter = 0;
    loop {
        match f() {
            Ok(grid) => return Ok(grid.retrieve_collapsed()),
            Err(err) => {
                if current_iter == n {
                    return Err(err);
                }
                current_iter += 1;
            }
        }
    }
}

pub fn try_n_times_2d_overlap<const SIZE_X: usize, const SIZE_Y: usize, Tile: TypedData>(
    n: u32,
    mut f: impl FnMut() -> Result<CollapsiblePatternGrid2D<SIZE_X, SIZE_Y, Tile>, CollapseError2D>,
) -> Result<CollapsedGrid2D, CollapseError2D> {
    let mut current_iter = 0;
    loop {
        match f() {
            Ok(grid) => return Ok(grid.retrieve_collapsed()),
            Err(err) => {
                if current_iter == n {
                    return Err(err);
                }
                current_iter += 1;
            }
        }
    }
}

pub struct GifSubscriber {
    file: Option<File>,
    frame: ImageBuffer<Rgb<u8>, Vec<u8>>,
    encoder: Option<gif::Encoder<File>>,
    collection: TypeIdMap<TilePixConst<4, 4, Rgb<u8>>>,
    frame_size: (u16, u16),
    resize: bool,
    map_size: GridSize2D,
}

impl GifSubscriber {
    pub fn new(file: File, size: &GridSize2D, collection: TypeIdMap<TilePixConst<4, 4, Rgb<u8>>>) -> Self {
        let frame = init_map_image_buffer(size, (4, 4));
        let frame_size = (frame.width() as u16, frame.height() as u16);

        Self {
            file: Some(file),
            frame,
            encoder: None,
            collection,
            frame_size,
            resize: false,
            map_size: *size,
        }
    }

    pub fn with_rescale(mut self, rescale: u8) -> Self {
        self.frame_size = (
            self.frame.width() as u16 * rescale as u16,
            self.frame.height() as u16 * rescale as u16,
        );
        self.resize = true;

        self
    }

    fn begin(&mut self) {
        self.encoder = Some(
            gif::Encoder::new(
                self.file.take().unwrap(),
                self.frame_size.0,
                self.frame_size.1,
                &[],
            )
            .unwrap(),
        );
        self.encoder
            .as_mut()
            .unwrap()
            .set_repeat(gif::Repeat::Infinite)
            .unwrap();
        self.write_frame();
    }

    fn write_frame(&mut self) {
        let mut buffer = self.frame.clone();
        if self.resize {
            buffer = image::imageops::resize(
                &buffer,
                self.frame_size.0 as u32,
                self.frame_size.1 as u32,
                image::imageops::FilterType::Nearest,
            )
        }
        let mut frame =
            gif::Frame::from_rgb_speed(self.frame_size.0, self.frame_size.1, &buffer, 15);
        frame.delay = 5;
        self.encoder.as_mut().unwrap().write_frame(&frame).unwrap();
    }
}

impl TileSubscriber2D for GifSubscriber {
    fn on_collapse(&mut self, position: &GridPosition2D, tile_type_id: u64) {
        if self.encoder.is_none() {
            self.begin()
        }

        write_tile(&mut self.frame, *position, self.collection.get(&tile_type_id).unwrap()).unwrap();
        self.write_frame();
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Pattern2DSubscriber for GifSubscriber {
    fn on_collapse(&mut self, position: &GridPosition2D, tile_type_id: u64, _pattern_id: u64) {
        if self.encoder.is_none() {
            self.begin()
        }

        write_tile(&mut self.frame, *position, self.collection.get(&tile_type_id).unwrap()).unwrap();
        self.write_frame();
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}