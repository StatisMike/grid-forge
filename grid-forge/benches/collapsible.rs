// use std::time::Duration;

// use criterion::*;
// use grid_forge::{
//     gen::collapse::{singular::*, *},
//     identifiable::{builders::IdentTileTraitBuilder, BasicIdentTileData},
//     vis::{collection::VisCollection, ops::load_gridmap_identifiable_auto, DefaultVisPixel},
//     GridSize,
// };
// use rand::SeedableRng;
// use rand_chacha::ChaChaRng;
// use utils::RngHelper;

// mod utils;

// fn analyze_adjacency_identity_10x10(c: &mut Criterion) {
//     let builder = IdentTileTraitBuilder::<BasicIdentTileData>::default();
//     let mut collection = VisCollection::<DefaultVisPixel, 4, 4>::default();

//     let seas_img = image::open("../assets/samples/seas.png")
//         .unwrap()
//         .into_rgb8();

//     let seas_grid = load_gridmap_identifiable_auto(&seas_img, &mut collection, &builder).unwrap();

//     c.bench_function("analyze_adjacency_identity_10x10", |b| {
//         b.iter(|| {
//             let mut analyzer = IdentityAnalyzer::default();
//             analyzer.analyze(&seas_grid);
//         })
//     });
// }

// fn analyze_adjacency_border_10x10(c: &mut Criterion) {
//     let builder = IdentTileTraitBuilder::<BasicIdentTileData>::default();
//     let mut collection = VisCollection::<DefaultVisPixel, 4, 4>::default();

//     let seas_img = image::open("../assets/samples/seas.png")
//         .unwrap()
//         .into_rgb8();

//     let seas_grid = load_gridmap_identifiable_auto(&seas_img, &mut collection, &builder).unwrap();

//     c.bench_function("analyze_adjacency_border_10x10", |b| {
//         b.iter(|| {
//             let mut analyzer = BorderAnalyzer::default();
//             analyzer.analyze(&seas_grid);
//         })
//     });
// }

// fn analyze_frequency_10x10(c: &mut Criterion) {
//     let builder = IdentTileTraitBuilder::<BasicIdentTileData>::default();
//     let mut collection = VisCollection::<DefaultVisPixel, 4, 4>::default();

//     let seas_img = image::open("../assets/samples/seas.png")
//         .unwrap()
//         .into_rgb8();

//     let seas_grid = load_gridmap_identifiable_auto(&seas_img, &mut collection, &builder).unwrap();

//     c.bench_function("analyze_frequency_10x10", |b| {
//         b.iter(|| {
//             let mut freq_hints = FrequencyHints::default();
//             freq_hints.analyze(&seas_grid);
//         })
//     });
// }

// fn analyze_build_collapsible_grid(c: &mut Criterion) {
//     let builder = IdentTileTraitBuilder::<BasicIdentTileData>::default();
//     let mut collection = VisCollection::<DefaultVisPixel, 4, 4>::default();

//     let seas_img = image::open("../assets/samples/seas.png")
//         .unwrap()
//         .into_rgb8();

//     let seas_grid = load_gridmap_identifiable_auto(&seas_img, &mut collection, &builder).unwrap();

//     let mut analyzer = BorderAnalyzer::default();
//     analyzer.analyze(&seas_grid);
//     let adj_rules = analyzer.adjacency();
//     let mut freq_hints = FrequencyHints::default();
//     freq_hints.analyze(&seas_grid);

//     c.bench_function("analyze_build_collapsible_grid", |b| {
//         b.iter(|| {
//             let _grid =
//                 CollapsibleTileGrid::new_empty(GridSize::new_xy(10, 10), &freq_hints, adj_rules);
//         })
//     });
// }

// fn gen_identity_position_10x10(c: &mut Criterion) {
//     let builder = IdentTileTraitBuilder::<BasicIdentTileData>::default();
//     let mut collection = VisCollection::<DefaultVisPixel, 4, 4>::default();

//     let seas_img = image::open("../assets/samples/seas.png")
//         .unwrap()
//         .into_rgb8();

//     let seas_grid = load_gridmap_identifiable_auto(&seas_img, &mut collection, &builder).unwrap();

//     let roads_img = image::open("../assets/samples/roads.png")
//         .unwrap()
//         .into_rgb8();
//     let roads_grid = load_gridmap_identifiable_auto(&roads_img, &mut collection, &builder).unwrap();

//     let mut analyzer = IdentityAnalyzer::default();
//     analyzer.analyze(&seas_grid);
//     analyzer.analyze(&roads_grid);

//     let mut frequency_hints = FrequencyHints::default();
//     frequency_hints.analyze(&seas_grid);
//     frequency_hints.analyze(&roads_grid);

//     let size = GridSize::new_xy(10, 10);
//     let mut grid = CollapsibleTileGrid::new_empty(size, &frequency_hints, analyzer.adjacency());

//     c.bench_function("gen_identity_position_10x10", |b| {
//         b.iter(|| {
//             // Seed for reproductability
//             let mut rng: ChaChaRng = RngHelper::init_str("singular_identity", 0)
//                 .with_pos(1008)
//                 .into();

//             let mut resolver = Resolver::default();
//             resolver
//                 .generate_position(
//                     &mut grid,
//                     &mut rng,
//                     &size.get_all_possible_positions(),
//                     PositionQueue::default(),
//                 )
//                 .unwrap();
//         })
//     });
// }

// fn gen_identity_entrophy_10x10(c: &mut Criterion) {
//     let builder = IdentTileTraitBuilder::<BasicIdentTileData>::default();
//     let mut collection = VisCollection::<DefaultVisPixel, 4, 4>::default();

//     let seas_img = image::open("../assets/samples/seas.png")
//         .unwrap()
//         .into_rgb8();

//     let seas_grid = load_gridmap_identifiable_auto(&seas_img, &mut collection, &builder).unwrap();

//     let roads_img = image::open("../assets/samples/roads.png")
//         .unwrap()
//         .into_rgb8();
//     let roads_grid = load_gridmap_identifiable_auto(&roads_img, &mut collection, &builder).unwrap();

//     let mut analyzer = IdentityAnalyzer::default();
//     analyzer.analyze(&seas_grid);
//     analyzer.analyze(&roads_grid);

//     let mut frequency_hints = FrequencyHints::default();
//     frequency_hints.analyze(&seas_grid);
//     frequency_hints.analyze(&roads_grid);

//     let size = GridSize::new_xy(10, 10);
//     let mut grid = CollapsibleTileGrid::new_empty(size, &frequency_hints, analyzer.adjacency());

//     c.bench_function("gen_identity_entrophy_10x10", |b| {
//         b.iter(|| {
//             // Seed for reproductability
//             let mut seed = [0u8; 32];
//             let bytes = "i am benchmarking".as_bytes();
//             seed[..bytes.len().min(32)].copy_from_slice(bytes);

//             let mut rng = ChaChaRng::from_seed(seed);

//             let mut resolver = Resolver::default();
//             resolver
//                 .generate_entrophy(&mut grid, &mut rng, &size.get_all_possible_positions())
//                 .unwrap();
//         })
//     });
// }

// fn gen_border_position_10x10(c: &mut Criterion) {
//     let builder = IdentTileTraitBuilder::<BasicIdentTileData>::default();
//     let mut collection = VisCollection::<DefaultVisPixel, 4, 4>::default();

//     let seas_img = image::open("../assets/samples/seas.png")
//         .unwrap()
//         .into_rgb8();

//     let seas_grid = load_gridmap_identifiable_auto(&seas_img, &mut collection, &builder).unwrap();

//     let roads_img = image::open("../assets/samples/roads.png")
//         .unwrap()
//         .into_rgb8();
//     let roads_grid = load_gridmap_identifiable_auto(&roads_img, &mut collection, &builder).unwrap();

//     let mut analyzer = BorderAnalyzer::default();
//     analyzer.analyze(&seas_grid);
//     analyzer.analyze(&roads_grid);

//     let mut frequency_hints = FrequencyHints::default();
//     frequency_hints.analyze(&seas_grid);
//     frequency_hints.analyze(&roads_grid);

//     let size = GridSize::new_xy(10, 10);
//     let mut grid = CollapsibleTileGrid::new_empty(size, &frequency_hints, analyzer.adjacency());

//     c.bench_function("gen_border_position_10x10", |b| {
//         b.iter(|| {
//             // Seed for reproductability
//             let mut rng: ChaChaRng = RngHelper::init_str("singular_border", 0)
//                 .with_pos(354)
//                 .into();

//             let mut resolver = Resolver::default();
//             resolver
//                 .generate_position(
//                     &mut grid,
//                     &mut rng,
//                     &size.get_all_possible_positions(),
//                     PositionQueue::default(),
//                 )
//                 .unwrap();
//         })
//     });
// }

// fn gen_border_entrophy_10x10(c: &mut Criterion) {
//     let builder = IdentTileTraitBuilder::<BasicIdentTileData>::default();
//     let mut collection = VisCollection::<DefaultVisPixel, 4, 4>::default();

//     let seas_img = image::open("../assets/samples/seas.png")
//         .unwrap()
//         .into_rgb8();

//     let seas_grid = load_gridmap_identifiable_auto(&seas_img, &mut collection, &builder).unwrap();

//     let roads_img = image::open("../assets/samples/roads.png")
//         .unwrap()
//         .into_rgb8();
//     let roads_grid = load_gridmap_identifiable_auto(&roads_img, &mut collection, &builder).unwrap();

//     let mut analyzer = BorderAnalyzer::default();
//     analyzer.analyze(&seas_grid);
//     analyzer.analyze(&roads_grid);

//     let mut frequency_hints = FrequencyHints::default();
//     frequency_hints.analyze(&seas_grid);
//     frequency_hints.analyze(&roads_grid);

//     let size = GridSize::new_xy(10, 10);
//     let mut grid = CollapsibleTileGrid::new_empty(size, &frequency_hints, analyzer.adjacency());

//     c.bench_function("gen_border_entrophy_10x10", |b| {
//         b.iter(|| {
//             // Seed for reproductability
//             let mut seed = [0u8; 32];
//             let bytes = "collapse_gen_example".as_bytes();
//             seed[..bytes.len().min(32)].copy_from_slice(bytes);

//             let mut rng = ChaChaRng::from_seed(seed);

//             let mut resolver = Resolver::default();
//             resolver
//                 .generate_entrophy(&mut grid, &mut rng, &size.get_all_possible_positions())
//                 .unwrap();
//         })
//     });
// }

// criterion_group!(
//     analyze_bench,
//     analyze_adjacency_identity_10x10,
//     analyze_adjacency_border_10x10,
//     analyze_frequency_10x10,
//     analyze_build_collapsible_grid,
// );
// criterion_group!(
//     name = generate_bench;
//     config = Criterion::default().measurement_time(Duration::from_secs(10));
//     targets = gen_identity_position_10x10, gen_identity_entrophy_10x10, gen_border_position_10x10, gen_border_entrophy_10x10
// );
// criterion_main!(analyze_bench, generate_bench);
