use std::hint::black_box;
use std::time::Duration;

use criterion::*;
use grid_forge::common::*;
use grid_forge::three_d::*;
use grid_forge::two_d::*;

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

pub fn create_2d_grid_bench_100x100(c: &mut Criterion) {
    let size = GridSize2D::new(100, 100);
    c.bench_function("create_2d_grid_100x100", |b| {
        b.iter(|| create_default_2d_grid(size))
    });
}

pub fn create_3d_grid_bench_100x10x10(c: &mut Criterion) {
    let size = GridSize3D::new(100, 10, 10);
    c.bench_function("create_3d_grid_100x10x10", |b| {
        b.iter(|| create_default_3d_grid(size))
    });
}

pub fn grid_access_2d_100x100(c: &mut Criterion) {
    let size = GridSize2D::new(100, 100);
    let grid = create_default_2d_grid(size.clone());
    let possible_positions = size.get_all_possible_positions();

    c.bench_function("grid_access_2d_100x100", |b| {
        b.iter(|| {
            for pos in possible_positions.iter() {
                let tile = grid.get_tile_at_position(pos).unwrap();
                black_box(tile);
            }
        })
    });
}

pub fn grid_access_2d_100x100_mut(c: &mut Criterion) {
    let size = GridSize2D::new(100, 100);
    let mut grid = create_default_2d_grid(size.clone());
    let possible_positions = size.get_all_possible_positions();

    c.bench_function("grid_access_2d_100x100_mut", |b| {
        b.iter(|| {
            for pos in possible_positions.iter() {
                let mut tile: TileMut2D<DefaultTile> =
                    grid.get_mut_tile_at_position(pos).unwrap().into();
                tile.as_mut().offset = 1;
            }
        })
    });
}

pub fn grid_access_2d_100x100_neighbour(c: &mut Criterion) {
    let size = GridSize2D::new(100, 100);
    let grid = create_default_2d_grid(size.clone());
    let possible_positions = size.get_all_possible_positions();

    c.bench_function("grid_access_2d_100x100_neighbour_up", |b| {
        b.iter(|| {
            for pos in possible_positions.iter() {
                let tile = grid.get_neighbour_at(pos, &Direction2D::Up);
                if pos.y() == 0 {
                    black_box(tile);
                    continue;
                }
                black_box(tile.unwrap());
            }
        })
    });

    c.bench_function("grid_access_2d_100x100_neighbour_down", |b| {
        b.iter(|| {
            for pos in possible_positions.iter() {
                let tile = grid.get_neighbour_at(pos, &Direction2D::Down);
                if pos.y() + 1 == size.y() {
                    black_box(tile);
                    continue;
                }
                black_box(tile.unwrap());
            }
        })
    });

    c.bench_function("grid_access_2d_100x100_neighbour_left", |b| {
        b.iter(|| {
            for pos in possible_positions.iter() {
                let tile = grid.get_neighbour_at(pos, &Direction2D::Left);
                if pos.x() == 0 {
                    black_box(tile);
                    continue;
                }
                black_box(tile.unwrap());
            }
        })
    });

    c.bench_function("grid_access_2d_100x100_neighbour_right", |b| {
        b.iter(|| {
            for pos in possible_positions.iter() {
                let tile = grid.get_neighbour_at(pos, &Direction2D::Right);
                if pos.x() + 1 == size.x() {
                    black_box(tile);
                    continue;
                }
                black_box(tile.unwrap());
            }
        })
    });
}

pub fn grid_access_2d_100x100_all_neighbours(c: &mut Criterion) {
    let size = GridSize2D::new(100, 100);
    let grid = create_default_2d_grid(size.clone());
    let possible_positions = size.get_all_possible_positions();

    c.bench_function("grid_access_2d_100x10x10_all_neighbours", |b| {
        b.iter(|| {
            for pos in possible_positions.iter() {
                let tiles = grid.get_neighbours(pos);
                black_box(tiles);
            }
        })
    });
}

pub fn grid_access_3d_100x10x10(c: &mut Criterion) {
    let size = GridSize3D::new(100, 10, 10);
    let grid = create_default_3d_grid(size.clone());
    let possible_positions = size.get_all_possible_positions();

    c.bench_function("grid_access_3d_100x10x10", |b| {
        b.iter(|| {
            for pos in possible_positions.iter() {
                let tile = grid.get_tile_at_position(pos).unwrap();
                black_box(tile);
            }
        })
    });
}

pub fn grid_access_3d_100x10x10_mut(c: &mut Criterion) {
    let size = GridSize3D::new(100, 10, 10);
    let mut grid = create_default_3d_grid(size.clone());
    let possible_positions = size.get_all_possible_positions();

    c.bench_function("grid_access_3d_100x10x10_mut", |b| {
        b.iter(|| {
            for pos in possible_positions.iter() {
                let mut tile =
                    grid.get_mut_tile_at_position(pos).unwrap();
                tile.as_mut().offset = 1;
            }
        })
    });
}

criterion_group!(
    name = grid_1000;
    config = Criterion::default().measurement_time(Duration::from_secs(10)).warm_up_time(Duration::from_secs(5));
    targets =   create_2d_grid_bench_100x100, create_3d_grid_bench_100x10x10,
                grid_access_2d_100x100, grid_access_2d_100x100_mut,
                grid_access_3d_100x10x10, grid_access_3d_100x10x10_mut
);

criterion_group!(
    name = grid_1000_neighbour;
    config = Criterion::default().measurement_time(Duration::from_secs(5)).warm_up_time(Duration::from_secs(3));
    targets =   grid_access_2d_100x100_neighbour,
                grid_access_2d_100x100_all_neighbours
);

criterion_main!(
    grid_1000,
    grid_1000_neighbour
);
