use std::hint::black_box;
use std::time::Duration;

use criterion::*;
use grid_forge_2d::prelude::*;
use grid_forge_core::id::{SharedData, TypeIdMap, TypedData};
use grid_forge_core::TileData;

pub struct DefaultTile {
    offset: usize,
    tile_id: u64,
}

impl TileData for DefaultTile {}

impl DefaultTile {
    const TYPE_COUNT: u64 = 10;

    pub fn new(offset: usize) -> Self {
        Self { offset, tile_id: tile_id_from_offset(offset) }
    }

    pub fn offset_matches(&self, offset: usize) -> bool {
        self.offset == offset
    }
}

impl TypedData for DefaultTile {
    fn tile_type_id(&self) -> u64 {
        self.tile_id
    }
}

pub struct DefaultTileShared {
    foo: bool,
}

impl DefaultTileShared {
    #[allow(dead_code)]
    pub fn new(tile_type_id: u64) -> Self {
        Self {
            foo: tile_type_id % 2 == 0,
        }
    }
}

impl SharedData for DefaultTileShared {}

fn create_default_2d_grid(size: GridSize2D) -> GridMap2D<DefaultTile> {
    let mut grid = GridMap2D::new(size);
    let possible_positions = size.get_all_possible_positions();
    let positions_with_offsets = possible_positions
        .iter()
        .map(|pos| (pos, size.offset(pos)))
        .collect::<Vec<_>>();
    for (pos, offset) in positions_with_offsets.iter() {
        grid.insert_data(pos, DefaultTile::new(*offset));
    }
    grid
}

fn tile_id_from_offset(offset: usize) -> u64 {
    offset as u64 % DefaultTile::TYPE_COUNT
}

pub fn create_grid_100x100(c: &mut Criterion) {
    let size = GridSize2D::new(100, 100);
    c.bench_function("create_2d_grid_100x100", |b| {
        b.iter(|| create_default_2d_grid(size))
    });
}

pub fn grid_access_100x100(c: &mut Criterion) {
    let size = GridSize2D::new(100, 100);
    let grid = create_default_2d_grid(size.clone());
    let possible_positions = size.get_all_possible_positions();

    c.bench_function("grid_access_100x100", |b| {
        b.iter(|| {
            for pos in possible_positions.iter() {
                let tile = grid.tile_at(pos).unwrap();
                black_box(tile);
            }
        })
    });
}

pub fn grid_access_100x100_mut(c: &mut Criterion) {
    let size = GridSize2D::new(100, 100);
    let mut grid = create_default_2d_grid(size.clone());
    let possible_positions = size.get_all_possible_positions();

    c.bench_function("grid_access_100x100_mut", |b| {
        b.iter(|| {
            for pos in possible_positions.iter() {
                let mut tile: TileMut2D<DefaultTile> =
                    grid.tile_at_mut(pos).unwrap().into();
                tile.data().offset = 1;
            }
        })
    });
}

pub fn grid_access_100x100_mut_track(c: &mut Criterion) {
    let size = GridSize2D::new(100, 100);
    let mut grid = create_default_2d_grid(size.clone());
    let possible_positions = size.get_all_possible_positions();
    grid.mut_access_tracking(true);

    c.bench_function("grid_access_100x100_mut_track", |b| {
        b.iter(|| {
            for pos in possible_positions.iter() {
                let mut tile = grid.tile_at_mut(pos).unwrap();
                tile.data().offset = 1;
            }
        })
    });
}

pub fn grid_access_100x100_shared(c: &mut Criterion) {
    let size = GridSize2D::new(100, 100);
    let grid = create_default_2d_grid(size.clone());
    let mut grid = GridMapShared2D::from_regular(grid);
    let mut shared_container = TypeIdMap::<DefaultTileShared>::default();

    for id in 0..DefaultTile::TYPE_COUNT {
        shared_container.insert(id, DefaultTileShared::new(id));
    }
    grid.import_shared(shared_container);

    let possible_positions = size.get_all_possible_positions();

    c.bench_function("grid_access_100x100_shared", |b| {
        b.iter(|| {
            for pos in possible_positions.iter() {
                let tile = grid.tile_with_shared_at(pos).unwrap();
                black_box(tile);
            }
        })
    });
}

pub fn grid_access_100x100_shared_data(c: &mut Criterion) {
    let size = GridSize2D::new(100, 100);
    let grid = create_default_2d_grid(size.clone());
    let mut grid = GridMapShared2D::from_regular(grid);
    let mut shared_container = TypeIdMap::<DefaultTileShared>::default();

    for id in 0..DefaultTile::TYPE_COUNT {
        shared_container.insert(id, DefaultTileShared::new(id));
    }
    grid.import_shared(shared_container);

    let possible_positions = size.get_all_possible_positions();

    c.bench_function("grid_access_100x100_shared_data", |b| {
        b.iter(|| {
            for pos in possible_positions.iter() {
                let data = grid.shared_at(pos).unwrap();
                black_box(data);
            }
        })
    });
}

pub fn grid_access_100x100_neighbour(c: &mut Criterion) {
    let size = GridSize2D::new(100, 100);
    let grid = create_default_2d_grid(size.clone());
    let possible_positions = size.get_all_possible_positions();

    c.bench_function("grid_access_100x100_neighbour_up", |b| {
        b.iter(|| {
            for pos in possible_positions.iter() {
                let tile = grid.neighbor_at(pos, &Direction2D::Up);
                black_box(tile);
            }
        })
    });

    c.bench_function("grid_access_100x100_neighbour_down", |b| {
        b.iter(|| {
            for pos in possible_positions.iter() {
                let tile = grid.neighbor_at(pos, &Direction2D::Down);
                black_box(tile);
            }
        })
    });

    c.bench_function("grid_access_100x100_neighbour_left", |b| {
        b.iter(|| {
            for pos in possible_positions.iter() {
                let tile = grid.neighbor_at(pos, &Direction2D::Left);
                black_box(tile);
            }
        })
    });

    c.bench_function("grid_access_100x100_neighbour_right", |b| {
        b.iter(|| {
            for pos in possible_positions.iter() {
                let tile = grid.neighbor_at(pos, &Direction2D::Right);
                black_box(tile);
            }
        })
    });
}

pub fn grid_access_100x100_all_neighbours(c: &mut Criterion) {
    let size = GridSize2D::new(100, 100);
    let grid = create_default_2d_grid(size.clone());
    let possible_positions = size.get_all_possible_positions();

    c.bench_function("grid_access_100x100_all_neighbours", |b| {
        b.iter(|| {
            for pos in possible_positions.iter() {
                let tiles = grid.neighbors(pos);
                black_box(tiles);
            }
        })
    });
}

criterion_group!(
    name = grid_1000;
    config = Criterion::default().measurement_time(Duration::from_secs(10)).warm_up_time(Duration::from_secs(5));
    targets =   grid_access_100x100,
                grid_access_100x100_mut,
                grid_access_100x100_mut_track,
                grid_access_100x100_shared,
                grid_access_100x100_shared_data,
                create_grid_100x100,
);

criterion_group!(
    name = grid_1000_neighbour;
    config = Criterion::default().measurement_time(Duration::from_secs(5)).warm_up_time(Duration::from_secs(3));
    targets =   grid_access_100x100_neighbour,
                grid_access_100x100_all_neighbours,
);

criterion_main!(grid_1000, grid_1000_neighbour);
