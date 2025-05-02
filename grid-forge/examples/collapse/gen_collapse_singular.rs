use std::fs::File;

use grid_forge::gen::collapse::singular::subscriber::DebugSubscriber;
use grid_forge::r#gen::collapse::two_d::CollapsibleTileGrid2D;
use grid_forge::r#gen::collapse::CollapsedGrid;
use grid_forge::two_d::*;
use grid_forge::vis::collection::VisCollection; 
use grid_forge::gen::collapse::{singular, CollapsibleGrid};
use rand_chacha::ChaChaRng;
use utils::{ArgHelper, GifSingleSubscriber, RngHelper, VisGridLoaderHelper, VisRotate};

mod utils;

const MAP_10X10: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../assets/samples/seas.png");
const MAP_20X20: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../assets/samples/roads.png");

const OUTPUTS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/collapse/outputs/");

fn main() {
    let args = ArgHelper::gather();

    // -------------------------------------------- GENERAL SETUP ------------------------------------------------//

    // VisCollection to handle Image <-> GridMap2D roundabouts.
    let mut vis_collection = VisCollection::default();

    // I'm loading two sample maps with 90 deegrees rotation to increase variety of rules which will be generated
    // after analyzing the maps.
    //
    // Custom helper is used there to keep the example less verbose.
    let maps = VisGridLoaderHelper::new(&mut vis_collection).load_w_rotate(
        &[MAP_10X10, MAP_20X20],
        &[VisRotate::None, VisRotate::R90, VisRotate::R180],
    );

    // Create Identity (for `identity_entrophy`) and Border (for `border_position`) analyzers and FrequencyRules.
    let mut identity_analyzer = singular::IdentityAnalyzer::default();
    // let mut border_analyzer = singular::BorderAnalyzer::default();
    let mut frequency_hints = singular::FrequencyHints::default();

    use grid_forge::r#gen::collapse::singular::Analyzer as _;

    // Analyze the loaded maps, recording the `AdjacencyRules` in analyzers.
    for map in maps {
        identity_analyzer.analyze(&map);
        // border_analyzer.analyze(&map);
        frequency_hints.analyze(&map);
    }

    let outputs_size = GridSize2D::new(30, 30);

    // Resolver can be reused, as it is used for the same tile type.
    let mut resolver = singular::Resolver::default();

    // ----- Singular with Entrophy Queue ----- //
    //
    // EntrophyQueue will keep higher success rate, but is slower than PositionQueue.
    if !args.skip_entrophy() {
        {
            // Save the collapse process as a GIF.
            if args.gif() {
                let file =
                    std::fs::File::create(format!("{}{}", OUTPUTS_DIR, "identity_entrophy.gif"))
                        .unwrap();
                let subscriber =
                    GifSingleSubscriber::new(file, &outputs_size, vis_collection.clone())
                        .with_rescale(3);

                resolver = resolver.with_subscriber(Box::new(subscriber));
            } else if args.debug() {
                let subsciber = DebugSubscriber::new(Some(
                    File::create(format!("{}{}", OUTPUTS_DIR, "identity_entrophy_debug.txt"))
                        .unwrap(),
                ));
                resolver = resolver.with_subscriber(Box::new(subsciber));
            }

            // Using propagating EntrophyQueue, we will use more restrictive `identity`
            // AdjacencyRules. It will help to keep high success rate, but is a little
            // slower than PositionQueue.
            let mut rng: ChaChaRng = RngHelper::init_str("singular_identity", 1).into();
            let mut to_collapse = CollapsibleTileGrid2D::new_empty(
                outputs_size,
                &frequency_hints,
                identity_analyzer.adjacency(),
            );
            resolver
                .generate_entrophy(
                    &mut to_collapse,
                    &mut rng,
                    &outputs_size.get_all_possible_positions(),
                )
                .unwrap();

            // `CollapsibleTileGrid` can be now transformed into `CollapsedGrid`. If you have custom `IdentifiableTileData`,
            // you can use `.retrieve_ident()` method.
            let collapsed = to_collapse.retrieve_collapsed();

            // We will generate output image using the same `VisCollection`.
            let mut out_buffer = vis_collection.init_map_image_buffer(collapsed.grid().size());
            vis_collection
                .draw_map(&collapsed.grid(), &mut out_buffer)
                .unwrap();

            // A little resize as tiles are 4x4 pixels themselves.
            out_buffer = image::imageops::resize(
                &out_buffer,
                outputs_size.x() * 4 * 3,
                outputs_size.y() * 4 * 3,
                image::imageops::FilterType::Nearest,
            );
            out_buffer
                .save(format!("{}{}", OUTPUTS_DIR, "identity_entrophy.png"))
                .unwrap();
        }

        // if !args.skip_position() {
        //     // ------------------------------ Border rules + Position Queue generation ----------------------------------//

        //     // Using non-propagating PositionQueue, we will use less restrictive `border`
        //     // AdjacencyRules. The success rate will be still moderately high - and
        //     // errors can be mitigated by just retrying, as non-propagating queue is faster.

        //     // Save the collapse process as a GIF
        //     if args.gif() {
        //         let file =
        //             std::fs::File::create(format!("{}{}", OUTPUTS_DIR, "border_position.gif"))
        //                 .unwrap();
        //         let subscriber =
        //             GifSingleSubscriber::new(file, &outputs_size, vis_collection.clone())
        //                 .with_rescale(3);

        //         resolver = resolver.with_subscriber(Box::new(subscriber));
        //     } else if args.debug() {
        //         let subsciber = DebugSubscriber::new(Some(
        //             File::create(format!("{}{}", OUTPUTS_DIR, "border_position_debug.txt"))
        //                 .unwrap(),
        //         ));
        //         resolver = resolver.with_subscriber(Box::new(subsciber));
        //     }

        //     // Using non-propagating PositionQueue, we will use less restrictive `border`
        //     // AdjacencyRules. The success rate will be still moderately high - and
        //     // errors can be mitigated by just retrying, as non-propagating queue is faster.
        //     let mut rng: ChaChaRng = RngHelper::init_str("singular_border", 5)
        //         .with_pos(833)
        //         .into();

        //     let mut to_collapse = CollapsibleTileGrid::new_empty(
        //         outputs_size,
        //         &frequency_hints,
        //         border_analyzer.adjacency(),
        //     );

        //     resolver
        //         .generate_position(
        //             &mut to_collapse,
        //             &mut rng,
        //             &outputs_size.get_all_possible_positions(),
        //             PositionQueue::default(),
        //         )
        //         .unwrap();

        //     // let collapsed = collapsed.unwrap().retrieve_collapsed();
        //     let collapsed = to_collapse.retrieve_collapsed();
        //     let mut out_buffer = vis_collection.init_map_image_buffer(collapsed.as_ref().size());
        //     vis_collection
        //         .draw_map(collapsed.as_ref(), &mut out_buffer)
        //         .unwrap();
        //     out_buffer = image::imageops::resize(
        //         &out_buffer,
        //         outputs_size.x() * 4 * 3,
        //         outputs_size.y() * 4 * 3,
        //         image::imageops::FilterType::Nearest,
        //     );
        //     out_buffer
        //         .save(format!("{}{}", OUTPUTS_DIR, "border_position.png"))
        //         .unwrap();
        // }
    }
}
