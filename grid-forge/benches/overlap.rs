use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use rand_chacha::ChaChaRng;

use grid_forge::{
    gen::collapse::overlap::*,
    identifiable::{builders::IdentTileTraitBuilder, BasicIdentTileData},
    vis::{collection::VisCollection, ops::load_gridmap_identifiable_auto, DefaultVisPixel},
    GridSize,
};
use utils::RngHelper;

mod utils;

const MAP_10X10: &str = "../assets/samples/seas.png";
const MAP_20X20: &str = "../assets/samples/roads.png";

fn analyze_10x10_pattern_2x2(c: &mut Criterion) {
    let builder = IdentTileTraitBuilder::<BasicIdentTileData>::default();
    let mut collection = VisCollection::<DefaultVisPixel, 4, 4>::default();

    let img = image::open(MAP_10X10).unwrap().into_rgb8();

    let grid = load_gridmap_identifiable_auto(&img, &mut collection, &builder).unwrap();

    c.bench_function("analyze_10x10_pattern_2x2", |b| {
        b.iter(|| {
            let mut analyzer =
                Analyzer::<OverlappingPattern2D<2, 2>, BasicIdentTileData>::default();
            analyzer.analyze(&grid);
        })
    });
}

fn analyze_10x10_pattern_3x3(c: &mut Criterion) {
    let builder = IdentTileTraitBuilder::<BasicIdentTileData>::default();
    let mut collection = VisCollection::<DefaultVisPixel, 4, 4>::default();

    let img = image::open(MAP_10X10).unwrap().into_rgb8();

    let grid = load_gridmap_identifiable_auto(&img, &mut collection, &builder).unwrap();

    c.bench_function("analyze_10x10_pattern_3x3", |b| {
        b.iter(|| {
            let mut analyzer =
                Analyzer::<OverlappingPattern2D<3, 3>, BasicIdentTileData>::default();
            analyzer.analyze(&grid);
        })
    });
}

fn generate_10x10_pattern_2x2_entrophy(c: &mut Criterion) {
    let builder = IdentTileTraitBuilder::<BasicIdentTileData>::default();

    let mut vis_collection = VisCollection::<DefaultVisPixel, 4, 4>::default();

    let mut analyzer = Analyzer::<OverlappingPattern2D<2, 2>, BasicIdentTileData>::default();

    for path in &[MAP_10X10, MAP_20X20] {
        let img = image::open(path).unwrap().into_rgb8();

        let grid = load_gridmap_identifiable_auto(&img, &mut vis_collection, &builder).unwrap();

        analyzer.analyze(&grid);
    }

    let pattern_collection = analyzer.get_collection().clone();
    let pattern_rules = analyzer.get_adjacency();
    let pattern_freq = analyzer.get_frequency();

    let size = GridSize::new_xy(10, 10);
    let positions = size.get_all_possible_positions();

    let grid =
        CollapsiblePatternGrid::new_empty(size, pattern_collection, pattern_freq, pattern_rules)
            .unwrap();

    c.bench_function("generate_10x10_pattern_2x2_entrophy", |b| {
        b.iter(|| {
            let mut rng: ChaChaRng = RngHelper::init_str("overlap_bench", 1).into();

            let mut resolver = Resolver::default();
            let res = resolver.generate_entrophy(grid.clone(), &mut rng, &positions);

            assert!(res.is_ok());
        })
    });
}

fn generate_10x10_pattern_3x3_entrophy(c: &mut Criterion) {
    let builder = IdentTileTraitBuilder::<BasicIdentTileData>::default();

    let mut vis_collection = VisCollection::<DefaultVisPixel, 4, 4>::default();

    let mut analyzer = Analyzer::<OverlappingPattern2D<3, 3>, BasicIdentTileData>::default();

    for path in &[MAP_10X10, MAP_20X20] {
        let img = image::open(path).unwrap().into_rgb8();

        let grid = load_gridmap_identifiable_auto(&img, &mut vis_collection, &builder).unwrap();

        analyzer.analyze(&grid);
    }

    let pattern_collection = analyzer.get_collection().clone();
    let pattern_rules = analyzer.get_adjacency();
    let pattern_freq = analyzer.get_frequency();

    let size = GridSize::new_xy(10, 10);
    let positions = size.get_all_possible_positions();

    let grid =
        CollapsiblePatternGrid::new_empty(size, pattern_collection, pattern_freq, pattern_rules)
            .unwrap();

    c.bench_function("generate_10x10_pattern_3x3_entrophy", |b| {
        b.iter(|| {
            let mut rng: ChaChaRng = RngHelper::init_str("overlap_bench", 1).into();

            let mut resolver = Resolver::default();
            let res = resolver.generate_entrophy(grid.clone(), &mut rng, &positions);

            assert!(res.is_ok())
        })
    });
}

criterion_group!(
    analyze_bench,
    analyze_10x10_pattern_2x2,
    analyze_10x10_pattern_3x3,
);
criterion_group!(
    name = generate_bench;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets = generate_10x10_pattern_2x2_entrophy, generate_10x10_pattern_3x3_entrophy
);
criterion_main!(analyze_bench, generate_bench);
