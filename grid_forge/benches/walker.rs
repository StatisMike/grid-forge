use criterion::*;
use grid_forge::{
    gen::walker::GridWalker2DBuilder,
    map::GridSize,
    tile::{GridPosition, GridTile, TileData},
};
use rand::thread_rng;

struct EmptyTileData;
impl TileData for EmptyTileData {}

fn walker_walk_4500(c: &mut Criterion) {
    let grid_size = GridSize::new_xy(255, 255);

    let mut walker = GridWalker2DBuilder::default()
        .with_size(grid_size)
        .with_rng(thread_rng())
        .with_min_step_size(2)
        .with_max_step_size(5)
        .build()
        .unwrap();

    c.bench_function("walker_walk_4500", |b| {
        b.iter(|| {
            while walker.current_iters() <= 4500 {
                walker.walk();
            }

            walker.reset();
            walker.set_current_pos(GridPosition::new_xy(
                grid_size.center().0,
                grid_size.center().1,
            ));
        })
    });
}

fn walker_walk_45000(c: &mut Criterion) {
    let grid_size = GridSize::new_xy(255, 255);

    let mut walker = GridWalker2DBuilder::default()
        .with_size(grid_size)
        .with_rng(thread_rng())
        .with_min_step_size(2)
        .with_max_step_size(5)
        .build()
        .unwrap();

    c.bench_function("walker_walk_45000", |b| {
        b.iter(|| {
            while walker.current_iters() <= 45000 {
                walker.walk();
            }
            walker.reset();
            walker.set_current_pos(GridPosition::new_xy(
                grid_size.center().0,
                grid_size.center().1,
            ));
        })
    });
}

fn walker_grid_4500(c: &mut Criterion) {
    let grid_size = GridSize::new_xy(255, 255);

    let mut walker = GridWalker2DBuilder::default()
        .with_size(grid_size)
        .with_rng(thread_rng())
        .with_min_step_size(2)
        .with_max_step_size(5)
        .build()
        .unwrap();

    while walker.current_iters() <= 4500 {
        walker.walk();
    }

    c.bench_function("walker_grid_4500", |b| {
        b.iter(|| {
            walker.gen_grid_map(|pos| GridTile::new(pos, EmptyTileData));
        })
    });
}

fn walker_grid_45000(c: &mut Criterion) {
    let grid_size = GridSize::new_xy(255, 255);

    let mut walker = GridWalker2DBuilder::default()
        .with_size(grid_size)
        .with_rng(thread_rng())
        .with_min_step_size(2)
        .with_max_step_size(5)
        .build()
        .unwrap();

    while walker.current_iters() <= 45000 {
        walker.walk();
    }

    c.bench_function("walker_grid_45000", |b| {
        b.iter(|| {
            walker.gen_grid_map(|pos| GridTile::new(pos, EmptyTileData));
        })
    });
}

criterion_group!(
    benches,
    walker_walk_4500,
    walker_walk_45000,
    walker_grid_4500,
    walker_grid_45000,
);
criterion_main!(benches);
