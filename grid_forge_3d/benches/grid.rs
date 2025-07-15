use std::hint::black_box;
use std::time::Duration;

use criterion::*;
use grid_forge_3d::prelude::*;
use grid_forge_core::TileData;

pub struct DefaultTile {
    offset: usize,
}

impl TileData for DefaultTile {}

impl DefaultTile {
    pub fn new(offset: usize) -> Self {
        Self { offset }
    }

    pub fn offset_matches(&self, offset: usize) -> bool {
        self.offset == offset
    }
}

fn create_default_3d_grid(size: GridSize3D) -> GridMap3D<DefaultTile> { 
    let mut grid = GridMap3D::new(size);
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

pub fn create_grid_10000(c: &mut Criterion) {
    let size = GridSize3D::new(10, 10, 100);
    c.bench_function("create_grid_10x10x100", |b| {
        b.iter(|| create_default_3d_grid(size))
    });
}

pub fn grid_access_10000(c: &mut Criterion) {
    let size = GridSize3D::new(10, 10, 100);
    let grid = create_default_3d_grid(size.clone());
    let possible_positions = size.get_all_possible_positions();

    c.bench_function("grid_access_10000", |b| {
        b.iter(|| {
            for pos in possible_positions.iter() {
                let tile = grid.get_tile_at_position(pos).unwrap();
                black_box(tile);
            }
        })
    });
}

pub fn grid_access_10000_mut(c: &mut Criterion) {
    let size = GridSize3D::new(10, 10, 100);
    let mut grid = create_default_3d_grid(size.clone());
    let possible_positions = size.get_all_possible_positions();

    c.bench_function("grid_access_10000_mut", |b| {
        b.iter(|| {
            for pos in possible_positions.iter() {
                let mut tile: TileMut3D<DefaultTile> =
                    grid.get_mut_tile_at_position(pos).unwrap().into();
                tile.data().offset = 1;
            }
        })
    });
}

pub fn grid_access_10000_mut_track(c: &mut Criterion) {
    let size = GridSize3D::new(10, 10, 100);
    let mut grid = create_default_3d_grid(size.clone());
    let possible_positions = size.get_all_possible_positions();
    grid.mut_access_tracking(true);

    c.bench_function("grid_access_10000_mut_track", |b| {
        b.iter(|| {
            for pos in possible_positions.iter() {
                let mut tile: TileMut3D<DefaultTile> =
                    grid.get_mut_tile_at_position(pos).unwrap().into();
                tile.data().offset = 1;
            }
        })
    });
}

pub fn grid_access_10000_neighbour(c: &mut Criterion) {
    let size = GridSize3D::new(10, 10, 100);
    let grid = create_default_3d_grid(size.clone());
    let possible_positions = size.get_all_possible_positions();

    c.bench_function("grid_access_10000_neighbour_up", |b| {
        b.iter(|| {
            for pos in possible_positions.iter() {
                let tile = grid.get_neighbour_at(pos, &Direction3D::Up);
                black_box(tile);
            }
        })
    });

    c.bench_function("grid_access_10000_neighbour_down", |b| {
        b.iter(|| {
            for pos in possible_positions.iter() {
                let tile = grid.get_neighbour_at(pos, &Direction3D::Down);
                black_box(tile);
            }
        })
    });

    c.bench_function("grid_access_10000_neighbour_left", |b| {
        b.iter(|| {
            for pos in possible_positions.iter() {
                let tile = grid.get_neighbour_at(pos, &Direction3D::Left);
                black_box(tile);
            }
        })
    });

    c.bench_function("grid_access_10000_neighbour_right", |b| {
        b.iter(|| {
            for pos in possible_positions.iter() {
                let tile = grid.get_neighbour_at(pos, &Direction3D::Right);
                black_box(tile);
            }
        })
    });


    c.bench_function("grid_access_10000_neighbour_lower", |b| {
        b.iter(|| {
            for pos in possible_positions.iter() {
                let tile = grid.get_neighbour_at(pos, &Direction3D::Lower);
                black_box(tile);
            }
        })
    });

    c.bench_function("grid_access_10000_neighbour_higher", |b| {
        b.iter(|| {
            for pos in possible_positions.iter() {
                let tile = grid.get_neighbour_at(pos, &Direction3D::Higher);
                black_box(tile);
            }
        })
    });
}

pub fn grid_access_10000_all_neighbours(c: &mut Criterion) {
    let size = GridSize3D::new(10, 10, 100);
    let grid = create_default_3d_grid(size.clone());
    let possible_positions = size.get_all_possible_positions();

    c.bench_function("grid_access_10000_all_neighbours", |b| {
        b.iter(|| {
            for pos in possible_positions.iter() {
                let tiles = grid.get_neighbours(pos);
                black_box(tiles);
            }
        })
    });
}



criterion_group!(
    name = grid_10000;
    config = Criterion::default().measurement_time(Duration::from_secs(10)).warm_up_time(Duration::from_secs(5));
    targets =   grid_access_10000,
                grid_access_10000_mut,
                grid_access_10000_mut_track,
                create_grid_10000,
);

criterion_group!(
    name = grid_10000_neighbour;
    config = Criterion::default().measurement_time(Duration::from_secs(5)).warm_up_time(Duration::from_secs(3));
    targets =   grid_access_10000_neighbour,
                grid_access_10000_all_neighbours,
);

criterion_main!(grid_10000, grid_10000_neighbour);
