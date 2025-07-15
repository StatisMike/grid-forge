use std::fmt::Display;

use crate::core::GridPosition2D;

/// Error returned by operations on image representations of two dimensional grids.
#[derive(Debug, Clone)]
pub struct VisError2D {
    pix_size: (usize, usize),
    kind: VisErrorKind,
}

impl VisError2D {
    pub(crate) fn new_nonexist(pos: GridPosition2D, pix_size: (usize, usize)) -> Self {
        Self {
            pix_size,
            kind: VisErrorKind::NonExistingTile(pos),
        }
    }

    pub(crate) fn new_grid_load(x: u32, y: u32, pix_size: (usize, usize)) -> Self {
        Self {
            pix_size,
            kind: VisErrorKind::WrongSizeGridLoad { x, y },
        }
    }

    pub(crate) fn new_grid_save(
        expected: (u32, u32),
        actual: (u32, u32),
        pix_size: (usize, usize),
    ) -> Self {
        Self {
            pix_size,
            kind: VisErrorKind::WrongSizeGridSave { expected, actual },
        }
    }

    pub(crate) fn new_tile_load(width: usize, height: usize, pix_size: (usize, usize)) -> Self {
        Self {
            pix_size,
            kind: VisErrorKind::WrongSizeTileLoad { width, height },
        }
    }

    pub(crate) fn new_nopix(tile_ids: &[u64], pix_size: (usize, usize)) -> Self {
        Self {
            pix_size,
            kind: VisErrorKind::NoPixelsForIdent(tile_ids.to_vec()),
        }
    }

    pub(crate) fn new_io(
        read: bool,
        tile_pos: GridPosition2D,
        pixel_pos: (u32, u32),
        pix_size: (usize, usize),
    ) -> Self {
        if read {
            Self {
                pix_size,
                kind: VisErrorKind::PixelRead {
                    tile_pos,
                    pixel_pos,
                },
            }
        } else {
            Self {
                pix_size,
                kind: VisErrorKind::PixelWrite {
                    tile_pos,
                    pixel_pos,
                },
            }
        }
    }
}

impl Display for VisError2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
          VisErrorKind::NonExistingTile(pos) => {
            write!(f, "tile at position: {pos:?} is not contained within used `VisCollection`. Make sure to register it first manually")
          }
            VisErrorKind::WrongSizeGridLoad { x, y } => {
                write!(f, "expected tile pixel size (x: {width}; y: {height}) is incompatible with GridMap image size: (x: {x}, y: {y})", width = self.pix_size.0, height = self.pix_size.1)
            }
            VisErrorKind::NoPixelsForIdent(tile_ids) => write!(
              f,
              "cannot draw tile: no pixels for tile of ids: {tile_ids:?} is present"
          ),
            VisErrorKind::WrongSizeTileLoad { width, height } => write!(f, "cannot load tile: expected tile pixel size (width: {pix_width}; height: {pix_height}), got: width: {width}; height: {height}", pix_width = self.pix_size.0, pix_height = self.pix_size.1),
      VisErrorKind::PixelRead {
          tile_pos,
          pixel_pos,
      } => write!(f, "cannot read tile pixels: image buffer is out of bounds for tile on position: {tile_pos:?}, with pixel: {pixel_pos:?}"),
      VisErrorKind::PixelWrite {
          tile_pos,
          pixel_pos,
      } => write!(f, "cannot draw tile: image buffer is out of bounds for tile on position: {tile_pos:?}, with pixel: {pixel_pos:?}"),
            VisErrorKind::WrongSizeGridSave { expected, actual } => write!(f, "actual image buffer size: {actual:?} differs from expected: {expected:?}"),
        }
    }
}

#[derive(Debug, Clone)]
enum VisErrorKind {
    NonExistingTile(GridPosition2D),
    NoPixelsForIdent(Vec<u64>),
    PixelRead {
        tile_pos: GridPosition2D,
        pixel_pos: (u32, u32),
    },
    PixelWrite {
        tile_pos: GridPosition2D,
        pixel_pos: (u32, u32),
    },
    WrongSizeGridLoad {
        x: u32,
        y: u32,
    },
    WrongSizeTileLoad {
        width: usize,
        height: usize,
    },
    WrongSizeGridSave {
        expected: (u32, u32),
        actual: (u32, u32),
    },
}
