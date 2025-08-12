use std::time::Duration;

use criterion::*;

use grid_forge_2d::core::GridSize2D;
use grid_forge_2d::image::ops::load_from_image_const_typed_auto;
use grid_forge_2d::image::TilePixConst;
use grid_forge_2d::procgen_collapse::queue::PositionQueue2D;
use grid_forge_2d::procgen_collapse::tile::{
    CollapsibleTileGrid2D, FrequencyHints2D, TileIdentityAnalyzer2D, TileBorderAnalyzer2D,
    TileResolver2D,
};
use grid_forge_core::id::{BasicTypedData, IdentTileDefaultBuilder, TypeIdMap};
use grid_forge_core::utils::dev::RngHelper;
use image::Rgb;
use rand_chacha::{ChaCha20Rng, ChaChaRng};

const MAP_10X10: &str = "../assets/samples/seas.png";
const MAP_20X20: &str = "../assets/samples/roads.png";

fn analyze_adjacency_identity_10x10(c: &mut Criterion) {
    let builder = IdentTileDefaultBuilder::<BasicTypedData>::default();

    let seas_img = image::open(MAP_10X10).unwrap().into_rgb8();
    let mut id_pixel_map = TypeIdMap::<TilePixConst<4, 4, Rgb<u8>>>::default();

    let seas_grid =
        load_from_image_const_typed_auto(&seas_img, &builder, &mut id_pixel_map).unwrap();

    c.bench_function("analyze_adjacency_identity_10x10", |b| {
        b.iter(|| {
            let mut analyzer = TileIdentityAnalyzer2D::default();
            analyzer.analyze(&seas_grid);
        });
    });
}

fn analyze_adjacency_border_10x10(c: &mut Criterion) {
    let builder = IdentTileDefaultBuilder::<BasicTypedData>::default();

    let seas_img = image::open(MAP_10X10).unwrap().into_rgb8();
    let mut id_pixel_map = TypeIdMap::<TilePixConst<4, 4, Rgb<u8>>>::default();

    let seas_grid =
        load_from_image_const_typed_auto(&seas_img, &builder, &mut id_pixel_map).unwrap();

    c.bench_function("analyze_adjacency_border_10x10", |b| {
        b.iter(|| {
            let mut analyzer = TileBorderAnalyzer2D::default();
            analyzer.analyze(&seas_grid);
        });
    });
}

fn analyze_frequency_10x10(c: &mut Criterion) {
    let builder = IdentTileDefaultBuilder::<BasicTypedData>::default();

    let seas_img = image::open(MAP_10X10).unwrap().into_rgb8();
    let mut id_pixel_map = TypeIdMap::<TilePixConst<4, 4, Rgb<u8>>>::default();

    let seas_grid =
        load_from_image_const_typed_auto(&seas_img, &builder, &mut id_pixel_map).unwrap();

    c.bench_function("analyze_frequency_10x10", |b| {
        b.iter(|| {
            let mut freq_hints = FrequencyHints2D::default();
            freq_hints.analyze(&seas_grid);
        });
    });
}

fn analyze_build_collapsible_grid(c: &mut Criterion) {
    let builder = IdentTileDefaultBuilder::<BasicTypedData>::default();

    let seas_img = image::open(MAP_10X10).unwrap().into_rgb8();
    let mut id_pixel_map = TypeIdMap::<TilePixConst<4, 4, Rgb<u8>>>::default();

    let seas_grid =
        load_from_image_const_typed_auto(&seas_img, &builder, &mut id_pixel_map).unwrap();

    let mut analyzer = TileBorderAnalyzer2D::default();
    analyzer.analyze(&seas_grid);
    let adj_rules = analyzer.adjacency_rules();
    let mut freq_hints = FrequencyHints2D::default();
    freq_hints.analyze(&seas_grid);

    c.bench_function("analyze_build_collapsible_grid", |b| {
        b.iter(|| {
            let _grid =
                CollapsibleTileGrid2D::new_empty(GridSize2D::new(10, 10), &freq_hints, adj_rules);
        });
    });
}

fn gen_identity_position_10x10(c: &mut Criterion) {
    let builder = IdentTileDefaultBuilder::<BasicTypedData>::default();
    let mut id_pixel_map = TypeIdMap::<TilePixConst<4, 4, Rgb<u8>>>::default();

    let mut analyzer = TileIdentityAnalyzer2D::default();
    let mut frequency_hints = FrequencyHints2D::default();

    for path in &[MAP_10X10, MAP_20X20] {
        let img = image::open(path).unwrap().into_rgb8();

        let grid = load_from_image_const_typed_auto(&img, &builder, &mut id_pixel_map).unwrap();

        analyzer.analyze(&grid);
        frequency_hints.analyze(&grid);
    }

    let size = GridSize2D::new(10, 10);
    let grid = CollapsibleTileGrid2D::new_empty(size, &frequency_hints, analyzer.adjacency_rules());

    c.bench_function("gen_identity_position_10x10", |b| {
        b.iter(|| {
            // Seed for reproductability
            let mut rng: ChaChaRng = RngHelper::init_str("singular_identity", 0)
                .with_pos(1008)
                .into();

            let mut cloned_grid = grid.clone();

            let mut resolver = TileResolver2D::default();
            resolver
                .generate_position(
                    &mut cloned_grid,
                    &mut rng,
                    &size.get_all_possible_positions(),
                    PositionQueue2D::default(),
                )
                .unwrap();
        });
    });
}

fn gen_identity_entrophy_10x10(c: &mut Criterion) {
    let builder = IdentTileDefaultBuilder::<BasicTypedData>::default();
    let mut id_pixel_map = TypeIdMap::<TilePixConst<4, 4, Rgb<u8>>>::default();

    let mut analyzer = TileIdentityAnalyzer2D::default();
    let mut frequency_hints = FrequencyHints2D::default();

    for path in &[MAP_10X10, MAP_20X20] {
        let img = image::open(path).unwrap().into_rgb8();

        let grid = load_from_image_const_typed_auto(&img, &builder, &mut id_pixel_map).unwrap();

        analyzer.analyze(&grid);
        frequency_hints.analyze(&grid);
    }

    let size = GridSize2D::new(10, 10);
    let grid = CollapsibleTileGrid2D::new_empty(size, &frequency_hints, analyzer.adjacency_rules());

    c.bench_function("gen_identity_entrophy_10x10", |b| {
        b.iter(|| {
            // Seed for reproductability
            let mut rng: ChaCha20Rng = RngHelper::init_str("i am benchmarking", 0).into();
            let mut cloned_grid = grid.clone();

            let mut resolver = TileResolver2D::default();
            resolver
                .generate_entrophy(
                    &mut cloned_grid,
                    &mut rng,
                    &size.get_all_possible_positions(),
                )
                .unwrap();
        });
    });
}

fn gen_border_position_10x10(c: &mut Criterion) {
    let builder = IdentTileDefaultBuilder::<BasicTypedData>::default();
    let mut id_pixel_map = TypeIdMap::<TilePixConst<4, 4, Rgb<u8>>>::default();

    let mut analyzer = TileBorderAnalyzer2D::default();
    let mut frequency_hints = FrequencyHints2D::default();

    for path in &[MAP_10X10, MAP_20X20] {
        let img = image::open(path).unwrap().into_rgb8();

        let grid = load_from_image_const_typed_auto(&img, &builder, &mut id_pixel_map).unwrap();

        analyzer.analyze(&grid);
        frequency_hints.analyze(&grid);
    }

    let size = GridSize2D::new(10, 10);
    let grid = CollapsibleTileGrid2D::new_empty(size, &frequency_hints, analyzer.adjacency_rules());

    c.bench_function("gen_border_position_10x10", |b| {
        b.iter(|| {
            // Seed for reproductability
            let mut rng: ChaChaRng = RngHelper::init_str("singular_border", 15).into();
            let mut cloned_grid = grid.clone();

            let mut resolver = TileResolver2D::default();
            resolver
                .generate_position(
                    &mut cloned_grid,
                    &mut rng,
                    &size.get_all_possible_positions(),
                    PositionQueue2D::default(),
                )
                .unwrap();
        });
    });
}

fn gen_border_entrophy_10x10(c: &mut Criterion) {
    let builder = IdentTileDefaultBuilder::<BasicTypedData>::default();
    let mut id_pixel_map = TypeIdMap::<TilePixConst<4, 4, Rgb<u8>>>::default();

    let mut analyzer = TileBorderAnalyzer2D::default();
    let mut frequency_hints = FrequencyHints2D::default();

    for path in &[MAP_10X10, MAP_20X20] {
        let img = image::open(path).unwrap().into_rgb8();

        let grid = load_from_image_const_typed_auto(&img, &builder, &mut id_pixel_map).unwrap();

        analyzer.analyze(&grid);
        frequency_hints.analyze(&grid);
    }

    let size = GridSize2D::new(10, 10);
    let grid = CollapsibleTileGrid2D::new_empty(size, &frequency_hints, analyzer.adjacency_rules());

    c.bench_function("gen_border_entrophy_10x10", |b| {
        b.iter(|| {
            // Seed for reproductability
            let mut rng: ChaCha20Rng = RngHelper::init_str("collapse_gen_example", 0).into();
            let mut cloned_grid = grid.clone();

            let mut resolver = TileResolver2D::default();
            resolver
                .generate_entrophy(
                    &mut cloned_grid,
                    &mut rng,
                    &size.get_all_possible_positions(),
                )
                .unwrap();
        });
    });
}

criterion_group!(
    analyze,
    analyze_adjacency_identity_10x10,
    analyze_adjacency_border_10x10,
    analyze_frequency_10x10,
    analyze_build_collapsible_grid
);
criterion_group! {
  name = generate;
  config = Criterion::default().measurement_time(Duration::from_secs(10));
  targets =
    gen_identity_position_10x10,
    gen_border_position_10x10,
    gen_identity_entrophy_10x10,
    gen_border_entrophy_10x10
}
criterion_main!(analyze, generate);
