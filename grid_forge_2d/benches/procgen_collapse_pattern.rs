use std::time::Duration;

use criterion::*;

use grid_forge_2d::core::GridSize2D;
use grid_forge_2d::image::ops::load_from_image_const_typed_auto;
use grid_forge_2d::image::TilePixConst;
use grid_forge_2d::procgen_collapse::pattern::{CollapsiblePatternGrid2D, OverlappingPattern2D, OverlappingPattern2DAnalyzer, OverlappingPattern2DResolver};
use grid_forge_2d::procgen_collapse::queue::PositionQueue2D;
use grid_forge_2d::procgen_collapse::tile::{
    CollapsibleTileGrid2D, FrequencyHints2D, SingularBorderAnalyzer2D, SingularIdentityAnalyzer2D,
    SingularResolver2D,
};
use grid_forge_core::id::{BasicTypedData, IdentTileDefaultBuilder, TypeIdMap};
use grid_forge_core::utils::dev::RngHelper;
use image::Rgb;
use rand_chacha::{ChaCha20Rng, ChaChaRng};

const MAP_10X10: &str = "../assets/samples/seas.png";
const MAP_20X20: &str = "../assets/samples/roads.png";
const MAP: &str = "../assets/samples/overlap.png";

fn analyze_adjacency_pattern_2x2(c: &mut Criterion) {
    let builder = IdentTileDefaultBuilder::<BasicTypedData>::default();

    let map_img = image::open(MAP).unwrap().into_rgb8();
    let mut id_pixel_map = TypeIdMap::<TilePixConst<4, 4, Rgb<u8>>>::default();

    let seas_grid =
        load_from_image_const_typed_auto(&map_img, &builder, &mut id_pixel_map).unwrap();

    c.bench_function("analyze_adjacency_pattern_2x2", |b| {
        b.iter(|| {
            let mut analyzer = OverlappingPattern2DAnalyzer::<2, 2, BasicTypedData>::default();
            analyzer.analyze(&seas_grid);
        });
    });
}

fn analyze_adjacency_pattern_3x3(c: &mut Criterion) {
    let builder = IdentTileDefaultBuilder::<BasicTypedData>::default();

    let map_img = image::open(MAP).unwrap().into_rgb8();
    let mut id_pixel_map = TypeIdMap::<TilePixConst<4, 4, Rgb<u8>>>::default();

    let seas_grid =
        load_from_image_const_typed_auto(&map_img, &builder, &mut id_pixel_map).unwrap();

    c.bench_function("analyze_adjacency_pattern_3x3", |b| {
        b.iter(|| {
            let mut analyzer = OverlappingPattern2DAnalyzer::<3, 3, BasicTypedData>::default();
            analyzer.analyze(&seas_grid);
        });
    });
}

fn analyze_build_collapsible_pattern_grid(c: &mut Criterion) {
    let builder = IdentTileDefaultBuilder::<BasicTypedData>::default();

    let map_img = image::open(MAP).unwrap().into_rgb8();
    let mut id_pixel_map = TypeIdMap::<TilePixConst<4, 4, Rgb<u8>>>::default();

    let map_grid =
        load_from_image_const_typed_auto(&map_img, &builder, &mut id_pixel_map).unwrap();

    let mut analyzer = OverlappingPattern2DAnalyzer::<3, 3, BasicTypedData>::default();
    analyzer.analyze(&map_grid);
    let adj_rules = analyzer.get_adjacency();
    let freq_hints = analyzer.get_frequency();
    let patterns = analyzer.get_collection();

    c.bench_function("analyze_build_collapsible_pattern_grid", |b| {
        b.iter(|| {
            let _grid =
                CollapsiblePatternGrid2D::new_empty(
                    GridSize2D::new(10, 10), 
                    analyzer.get_collection().clone(), 
                    analyzer.get_frequency(), 
                    analyzer.get_adjacency()
                )
                .unwrap();
        });
    });
}

fn generate_10x10_pattern_2x2_entrophy(c: &mut Criterion) {
    let builder = IdentTileDefaultBuilder::<BasicTypedData>::default();

    let mut id_pixel_map = TypeIdMap::<TilePixConst<4, 4, Rgb<u8>>>::default();

    let mut analyzer = OverlappingPattern2DAnalyzer::<2, 2, BasicTypedData>::default();

    let img = image::open(MAP).unwrap().into_rgb8();

    let grid = load_from_image_const_typed_auto(&img, &builder, &mut id_pixel_map).unwrap();

    analyzer.analyze(&grid);

    let pattern_collection = analyzer.get_collection().clone();
    let pattern_rules = analyzer.get_adjacency();
    let pattern_freq = analyzer.get_frequency();

    let size = GridSize2D::new(10, 10);
    let positions = size.get_all_possible_positions();

    let grid =
        CollapsiblePatternGrid2D::new_empty(size, pattern_collection, pattern_freq, pattern_rules)
            .unwrap();

    c.bench_function("generate_10x10_pattern_2x2_entrophy", |b| {
        b.iter(|| {
            let mut rng: ChaChaRng = RngHelper::init_str("overlap_bench", 1).into();

            let mut resolver = OverlappingPattern2DResolver::default();
            let res = resolver.generate_entrophy(grid.clone(), &mut rng, &positions);

            assert!(res.is_ok());
        });
    });
}

fn generate_10x10_pattern_3x3_entrophy(c: &mut Criterion) {
    let builder = IdentTileDefaultBuilder::<BasicTypedData>::default();

    let mut id_pixel_map = TypeIdMap::<TilePixConst<4, 4, Rgb<u8>>>::default();

    let mut analyzer = OverlappingPattern2DAnalyzer::<3, 3, BasicTypedData>::default();

    let img = image::open(MAP).unwrap().into_rgb8();

    let grid = load_from_image_const_typed_auto(&img, &builder, &mut id_pixel_map).unwrap();

    analyzer.analyze(&grid);

    let pattern_collection = analyzer.get_collection().clone();
    let pattern_rules = analyzer.get_adjacency();
    let pattern_freq = analyzer.get_frequency();

    let size = GridSize2D::new(10, 10);
    let positions = size.get_all_possible_positions();

    let grid =
        CollapsiblePatternGrid2D::new_empty(size, pattern_collection, pattern_freq, pattern_rules)
            .unwrap();

    c.bench_function("generate_10x10_pattern_3x3_entrophy", |b| {
        b.iter(|| {
            let mut rng: ChaChaRng = RngHelper::init_str("overlap_bench", 1).into();

            let mut resolver = OverlappingPattern2DResolver::default();
            let res = resolver.generate_entrophy(grid.clone(), &mut rng, &positions);

            assert!(res.is_ok());
        });
    });
}

fn generate_10x10_pattern_2x2_position(c: &mut Criterion) {
    let builder = IdentTileDefaultBuilder::<BasicTypedData>::default();

    let mut id_pixel_map = TypeIdMap::<TilePixConst<4, 4, Rgb<u8>>>::default();

    let mut analyzer = OverlappingPattern2DAnalyzer::<2, 2, BasicTypedData>::default();

    let img = image::open(MAP).unwrap().into_rgb8();

    let grid = load_from_image_const_typed_auto(&img, &builder, &mut id_pixel_map).unwrap();

    analyzer.analyze(&grid);

    let pattern_collection = analyzer.get_collection().clone();
    let pattern_rules = analyzer.get_adjacency();
    let pattern_freq = analyzer.get_frequency();

    let size = GridSize2D::new(10, 10);
    let positions = size.get_all_possible_positions();

    let grid =
        CollapsiblePatternGrid2D::new_empty(size, pattern_collection, pattern_freq, pattern_rules)
            .unwrap();

    c.bench_function("generate_10x10_pattern_2x2_position", |b| {
        b.iter(|| {
            let mut rng: ChaChaRng = RngHelper::init_str("overlap_position", 0).into();

            let mut resolver = OverlappingPattern2DResolver::default();
            let res = resolver.generate_position(grid.clone(), &mut rng, &positions, PositionQueue2D::default());

            assert!(res.is_ok());
        });
    });
}

fn generate_10x10_pattern_3x3_position(c: &mut Criterion) {
    let builder = IdentTileDefaultBuilder::<BasicTypedData>::default();

    let mut id_pixel_map = TypeIdMap::<TilePixConst<4, 4, Rgb<u8>>>::default();

    let mut analyzer = OverlappingPattern2DAnalyzer::<3, 3, BasicTypedData>::default();

    let img = image::open(MAP).unwrap().into_rgb8();

    let grid = load_from_image_const_typed_auto(&img, &builder, &mut id_pixel_map).unwrap();

    analyzer.analyze(&grid);

    let pattern_collection = analyzer.get_collection().clone();
    let pattern_rules = analyzer.get_adjacency();
    let pattern_freq = analyzer.get_frequency();

    let size = GridSize2D::new(10, 10);
    let positions = size.get_all_possible_positions();

    let grid =
        CollapsiblePatternGrid2D::new_empty(size, pattern_collection, pattern_freq, pattern_rules)
            .unwrap();

    c.bench_function("generate_10x10_pattern_3x3_position", |b| {
        b.iter(|| {
                let mut rng: ChaChaRng = RngHelper::init_str("overlap_position", 0)
                .with_pos(3767)
                .into();

            let mut resolver = OverlappingPattern2DResolver::default();
            let res = resolver.generate_position(grid.clone(), &mut rng, &positions, PositionQueue2D::default());

            assert!(res.is_ok());
        });
    });
}

// fn generate_10x10_pattern_3x3_entrophy(c: &mut Criterion) {
//     let builder = IdentTileTraitBuilder::<BasicIdentTileData>::default();

//     let mut vis_collection = VisCollection::<DefaultVisPixel, 4, 4>::default();

//     let mut analyzer = Analyzer::<OverlappingPattern2D<3, 3>, BasicIdentTileData>::default();

//     let img = image::open(MAP).unwrap().into_rgb8();

//     let grid = load_gridmap_identifiable_auto(&img, &mut vis_collection, &builder).unwrap();

//     analyzer.analyze(&grid);

//     let pattern_collection = analyzer.get_collection().clone();
//     let pattern_rules = analyzer.get_adjacency();
//     let pattern_freq = analyzer.get_frequency();

//     let size = GridSize::new_xy(10, 10);
//     let positions = size.get_all_possible_positions();

//     let grid =
//         CollapsiblePatternGrid::new_empty(size, pattern_collection, pattern_freq, pattern_rules)
//             .unwrap();

//     c.bench_function("generate_10x10_pattern_3x3_entrophy", |b| {
//         b.iter(|| {
//             let mut rng: ChaChaRng = RngHelper::init_str("overlap_bench", 1).into();

//             let mut resolver = Resolver::default();
//             let res = resolver.generate_entrophy(grid.clone(), &mut rng, &positions);

//             assert!(res.is_ok())
//         });
//     });
// }

// fn generate_10x10_pattern_2x2_position(c: &mut Criterion) {
//     let builder = IdentTileTraitBuilder::<BasicIdentTileData>::default();

//     let mut vis_collection = VisCollection::<DefaultVisPixel, 4, 4>::default();

//     let mut analyzer = Analyzer::<OverlappingPattern2D<2, 2>, BasicIdentTileData>::default();

//     let img = image::open(MAP).unwrap().into_rgb8();

//     let grid = load_gridmap_identifiable_auto(&img, &mut vis_collection, &builder).unwrap();

//     analyzer.analyze(&grid);

//     let pattern_collection = analyzer.get_collection().clone();
//     let pattern_rules = analyzer.get_adjacency();
//     let pattern_freq = analyzer.get_frequency();

//     let size = GridSize::new_xy(10, 10);
//     let positions = size.get_all_possible_positions();

//     let grid =
//         CollapsiblePatternGrid::new_empty(size, pattern_collection, pattern_freq, pattern_rules)
//             .unwrap();

//     c.bench_function("generate_10x10_pattern_2x2_position", |b| {
//         b.iter(|| {
//             let mut rng: ChaChaRng = RngHelper::init_str("overlap_position", 0).into();

//             let mut resolver = Resolver::default();
//             let res = resolver.generate_position(
//                 grid.clone(),
//                 &mut rng,
//                 &positions,
//                 PositionQueue::default(),
//             );

//             assert!(res.is_ok())
//         });
//     });
// }

// fn generate_10x10_pattern_3x3_position(c: &mut Criterion) {
//     let builder = IdentTileTraitBuilder::<BasicIdentTileData>::default();

//     let mut vis_collection = VisCollection::<DefaultVisPixel, 4, 4>::default();

//     let mut analyzer = Analyzer::<OverlappingPattern2D<3, 3>, BasicIdentTileData>::default();

//     let img = image::open(MAP).unwrap().into_rgb8();

//     let grid = load_gridmap_identifiable_auto(&img, &mut vis_collection, &builder).unwrap();

//     analyzer.analyze(&grid);

//     let pattern_collection = analyzer.get_collection().clone();
//     let pattern_rules = analyzer.get_adjacency();
//     let pattern_freq = analyzer.get_frequency();

//     let size = GridSize::new_xy(10, 10);
//     let positions = size.get_all_possible_positions();

//     let grid =
//         CollapsiblePatternGrid::new_empty(size, pattern_collection, pattern_freq, pattern_rules)
//             .unwrap();

//     c.bench_function("generate_10x10_pattern_3x3_position", |b| {
//         b.iter(|| {
//             let mut rng: ChaChaRng = RngHelper::init_str("overlap_position", 0)
//                 .with_pos(3767)
//                 .into();

//             let mut resolver = Resolver::default();
//             let res = resolver.generate_position(
//                 grid.clone(),
//                 &mut rng,
//                 &positions,
//                 PositionQueue::default(),
//             );

//             assert!(res.is_ok())
//         });
//     });
// }

// fn gen_identity_position_10x10(c: &mut Criterion) {
//     let builder = IdentTileDefaultBuilder::<BasicTypedData>::default();
//     let mut id_pixel_map = TypeIdMap::<TilePixConst<4, 4, Rgb<u8>>>::default();

//     let mut analyzer = SingularIdentityAnalyzer2D::default();
//     let mut frequency_hints = FrequencyHints2D::default();

//     for path in &[MAP_10X10, MAP_20X20] {
//         let img = image::open(path).unwrap().into_rgb8();

//         let grid = load_from_image_const_typed_auto(&img, &builder, &mut id_pixel_map).unwrap();

//         analyzer.analyze(&grid);
//         frequency_hints.analyze(&grid);
//     }

//     let size = GridSize2D::new(10, 10);
//     let grid = CollapsibleTileGrid2D::new_empty(size, &frequency_hints, analyzer.adjacency_rules());

//     c.bench_function("gen_identity_position_10x10", |b| {
//         b.iter(|| {
//             // Seed for reproductability
//             let mut rng: ChaChaRng = RngHelper::init_str("singular_identity", 0)
//                 .with_pos(1008)
//                 .into();

//             let mut cloned_grid = grid.clone();

//             let mut resolver = SingularResolver2D::default();
//             resolver
//                 .generate_position(
//                     &mut cloned_grid,
//                     &mut rng,
//                     &size.get_all_possible_positions(),
//                     PositionQueue2D::default(),
//                 )
//                 .unwrap();
//         });
//     });
// }

// fn gen_identity_entrophy_10x10(c: &mut Criterion) {
//     let builder = IdentTileDefaultBuilder::<BasicTypedData>::default();
//     let mut id_pixel_map = TypeIdMap::<TilePixConst<4, 4, Rgb<u8>>>::default();

//     let mut analyzer = SingularIdentityAnalyzer2D::default();
//     let mut frequency_hints = FrequencyHints2D::default();

//     for path in &[MAP_10X10, MAP_20X20] {
//         let img = image::open(path).unwrap().into_rgb8();

//         let grid = load_from_image_const_typed_auto(&img, &builder, &mut id_pixel_map).unwrap();

//         analyzer.analyze(&grid);
//         frequency_hints.analyze(&grid);
//     }

//     let size = GridSize2D::new(10, 10);
//     let grid = CollapsibleTileGrid2D::new_empty(size, &frequency_hints, analyzer.adjacency_rules());

//     c.bench_function("gen_identity_entrophy_10x10", |b| {
//         b.iter(|| {
//             // Seed for reproductability
//             let mut rng: ChaCha20Rng = RngHelper::init_str("i am benchmarking", 0).into();
//             let mut cloned_grid = grid.clone();

//             let mut resolver = SingularResolver2D::default();
//             resolver
//                 .generate_entrophy(
//                     &mut cloned_grid,
//                     &mut rng,
//                     &size.get_all_possible_positions(),
//                 )
//                 .unwrap();
//         });
//     });
// }

// fn gen_border_position_10x10(c: &mut Criterion) {
//     let builder = IdentTileDefaultBuilder::<BasicTypedData>::default();
//     let mut id_pixel_map = TypeIdMap::<TilePixConst<4, 4, Rgb<u8>>>::default();

//     let mut analyzer = SingularBorderAnalyzer2D::default();
//     let mut frequency_hints = FrequencyHints2D::default();

//     for path in &[MAP_10X10, MAP_20X20] {
//         let img = image::open(path).unwrap().into_rgb8();

//         let grid = load_from_image_const_typed_auto(&img, &builder, &mut id_pixel_map).unwrap();

//         analyzer.analyze(&grid);
//         frequency_hints.analyze(&grid);
//     }

//     let size = GridSize2D::new(10, 10);
//     let grid = CollapsibleTileGrid2D::new_empty(size, &frequency_hints, analyzer.adjacency_rules());

//     c.bench_function("gen_border_position_10x10", |b| {
//         b.iter(|| {
//             // Seed for reproductability
//             let mut rng: ChaChaRng = RngHelper::init_str("singular_border", 15).into();
//             let mut cloned_grid = grid.clone();

//             let mut resolver = SingularResolver2D::default();
//             resolver
//                 .generate_position(
//                     &mut cloned_grid,
//                     &mut rng,
//                     &size.get_all_possible_positions(),
//                     PositionQueue2D::default(),
//                 )
//                 .unwrap();
//         });
//     });
// }

// fn gen_border_entrophy_10x10(c: &mut Criterion) {
//     let builder = IdentTileDefaultBuilder::<BasicTypedData>::default();
//     let mut id_pixel_map = TypeIdMap::<TilePixConst<4, 4, Rgb<u8>>>::default();

//     let mut analyzer = SingularBorderAnalyzer2D::default();
//     let mut frequency_hints = FrequencyHints2D::default();

//     for path in &[MAP_10X10, MAP_20X20] {
//         let img = image::open(path).unwrap().into_rgb8();

//         let grid = load_from_image_const_typed_auto(&img, &builder, &mut id_pixel_map).unwrap();

//         analyzer.analyze(&grid);
//         frequency_hints.analyze(&grid);
//     }

//     let size = GridSize2D::new(10, 10);
//     let grid = CollapsibleTileGrid2D::new_empty(size, &frequency_hints, analyzer.adjacency_rules());

//     c.bench_function("gen_border_entrophy_10x10", |b| {
//         b.iter(|| {
//             // Seed for reproductability
//             let mut rng: ChaCha20Rng = RngHelper::init_str("collapse_gen_example", 0).into();
//             let mut cloned_grid = grid.clone();

//             let mut resolver = SingularResolver2D::default();
//             resolver
//                 .generate_entrophy(
//                     &mut cloned_grid,
//                     &mut rng,
//                     &size.get_all_possible_positions(),
//                 )
//                 .unwrap();
//         });
//     });
// }

criterion_group!(
    analyze,
    analyze_adjacency_pattern_2x2,
    analyze_adjacency_pattern_3x3,
    analyze_build_collapsible_pattern_grid
);
criterion_group! {
  name = generate;
  config = Criterion::default().measurement_time(Duration::from_secs(10));
  targets =
    // generate_10x10_pattern_2x2_entrophy,
    // generate_10x10_pattern_3x3_entrophy,
    // generate_10x10_pattern_2x2_position,
    generate_10x10_pattern_3x3_position,
    // gen_border_position_10x10,
    // gen_identity_entrophy_10x10,
    // gen_border_entrophy_10x10
}
criterion_main!(
    // analyze, 
    generate
);
