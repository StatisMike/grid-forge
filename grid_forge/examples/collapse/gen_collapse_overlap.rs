use std::fs::File;

use grid_forge::{
    gen::collapse::*,
    map::GridSize,
    vis::collection::VisCollection,
};
use overlap::{CollapsiblePatternGrid, DebugSubscriber};
use rand_chacha::ChaChaRng;
use utils::{ArgHelper, GifSingleSubscriber, RngHelper, VisGridLoaderHelper, VisRotate};

mod utils;

const MAP_10X10: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../assets/samples/seas.png");
const MAP_20X20: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../assets/samples/roads.png");

const OUTPUTS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/collapse/outputs/");

fn main() {
    let args = ArgHelper::gather();

    // VisCollection to handle Image <-> GridMap2D roundabouts
    let mut vis_collection = VisCollection::default();

    // Load two sample maps with 90 deegrees rotation to increase variety of rules.
    let maps = VisGridLoaderHelper::new(&mut vis_collection)
        .load_w_rotate(&[MAP_10X10, MAP_20X20], &[VisRotate::None]);

    // Create overlap analyzer.
    let mut analyzer = overlap::Analyzer::<overlap::OverlappingPattern2D<2, 2>, _>::default();

    // Analyze the loaded maps
    for map in maps {
        analyzer.analyze_map(&map);

    }

    let outputs_size = GridSize::new_xy(30, 30);

    // Resolver can be reused, as it is used for the same tile type.
    let mut resolver = overlap::Resolver::default();

    // Save the collapse process as a GIF.
    if args.gif() {
        let file =
            std::fs::File::create(format!("{}{}", OUTPUTS_DIR, "overlap_entrophy.gif")).unwrap();

        let subscriber =
            GifSingleSubscriber::new(file, &outputs_size, vis_collection.clone()).with_rescale(3);

        resolver = resolver.with_subscriber(Box::new(subscriber));
    } else if args.debug() {
        let subsciber = DebugSubscriber::new(Some(File::create(format!("{}{}", OUTPUTS_DIR, "overlap_debug.txt")).unwrap()));
        resolver = resolver.with_subscriber(Box::new(subsciber));
    }

    // Using propagating EntrophyQueue we can keep hight success rate, but is a little
    // slower than PositionQueue.
    let mut rng: ChaChaRng = RngHelper::init_str("overlap_entrophy", 1).into();
    let to_collapse = CollapsiblePatternGrid::new_empty(
        outputs_size,
        analyzer.get_collection().clone(),
        analyzer.get_frequency(),
        analyzer.get_adjacency(),
    )
    .unwrap();

    let after_collapse = resolver
        .generate(
            to_collapse,
            &mut rng,
            &outputs_size.get_all_possible_positions(),
            EntrophyQueue::default(),
        )
        .unwrap();

    let collapsed = after_collapse.retrieve_collapsed();
    let mut out_buffer = vis_collection.init_map_image_buffer(collapsed.as_ref().size());
    vis_collection
        .draw_map(collapsed.as_ref(), &mut out_buffer)
        .unwrap();
    out_buffer = image::imageops::resize(
        &out_buffer,
        outputs_size.x() * 4 * 3,
        outputs_size.y() * 4 * 3,
        image::imageops::FilterType::Nearest,
    );
    out_buffer
        .save(format!("{}{}", OUTPUTS_DIR, "overlap_entrophy.png"))
        .unwrap();
}
