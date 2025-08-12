use std::fs::File;

use grid_forge::prelude::*;
use grid_forge::procgen_collapse::DebugSubscriber2D;
use grid_forge::{
    id::{BasicTypedData, TypeIdMap},
    image::{
        ops::{init_map_image_buffer, write_to_image_const_typed},
        TilePixConst,
    },
    procgen_collapse::PositionQueue2D,
};
use grid_forge::procgen_collapse::pattern::{
    CollapsiblePatternGrid2D, Pattern2DAnalyzer, Pattern2DResolver,
};
use image::Rgb;
use rand_chacha::ChaChaRng;

use crate::utils::{
    collapse::{try_n_times_2d_tile, try_n_times_2d_pattern, ArgHelper, GifSubscriber},
    image::{VisGridLoaderHelper, VisRotate},
    RngHelper,
};

#[path = "../../example_utils/mod.rs"]
mod utils;

const MAP: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../assets/samples/overlap.png");

const OUTPUTS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/2d/procgen/output/");

fn main() {
    let args = ArgHelper::gather();

    // Map for TileId and Pixel ID to handle Image <-> GridMap2D roundabouts
    // Use `TilePixConst` to handle pixel data if its size is known at compile time.
    let mut id_pixel_map = TypeIdMap::<TilePixConst<4, 4, Rgb<u8>>>::default();

    let maps = VisGridLoaderHelper::new(&mut id_pixel_map)
        .load_w_rotate(&[MAP], &[VisRotate::None, VisRotate::R90]);

    let outputs_size = GridSize2D::new(30, 30);

    if !args.skip_entrophy() {
        // Create overlap analyzer.
        let mut analyzer = Pattern2DAnalyzer::<3, 3, BasicTypedData>::default();

        for map in maps.iter() {
            analyzer.analyze(map);
        }

        // Resolver can be reused, as it is used for the same tile type.
        let mut resolver = Pattern2DResolver::default();

        // Save the collapse process as a GIF.
        if args.gif() {
            let file = std::fs::File::create(format!("{}{}", OUTPUTS_DIR, "overlap_entrophy.gif"))
                .unwrap();

            let subscriber =
                GifSubscriber::new(file, &outputs_size, id_pixel_map.clone()).with_rescale(3);

            resolver = resolver.with_subscriber(Box::new(subscriber));
        } else if args.debug() {
            let subsciber = DebugSubscriber2D::new(Some(
                File::create(format!("{}{}", OUTPUTS_DIR, "overlap_entrophy_debug.txt")).unwrap(),
            ));
            resolver = resolver.with_subscriber(Box::new(subsciber));
        }

        // Using propagating EntrophyQueue, we will use more restrictive `identity`
        // AdjacencyRules. It will help to keep high success rate, but is a little
        // slower than PositionQueue.
        let mut rng: ChaChaRng = RngHelper::init_str("overlap entrophy", 1)
            .with_pos(45138)
            .into();

        let to_collapse = CollapsiblePatternGrid2D::new_empty(
            outputs_size,
            analyzer.get_collection().clone(),
            &analyzer.get_frequency().clone().with_max_weight(30),
            analyzer.get_adjacency(),
        )
        .unwrap();

        let after_collapse = resolver
            .generate_entrophy(
                to_collapse,
                &mut rng,
                &outputs_size.get_all_possible_positions(),
            )
            .unwrap();

        let collapsed = after_collapse.retrieve_collapsed();

        // let collapsed = try_n_times_2d_overlap(200, || {

        //     RngHelper::print_state(&rng);
        //     resolver.generate_entrophy(to_collapse.clone(), &mut rng, &outputs_size.get_all_possible_positions())

        // }).unwrap();

        // We will generate output image using the same `VisCollection`.
        let mut out_buffer = init_map_image_buffer(collapsed.grid().size(), (4, 4));
        write_to_image_const_typed(&mut out_buffer, collapsed.grid(), &id_pixel_map).unwrap();

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

    if !args.skip_position() {
        // Using non-propagating PositionQueue, we will use less restrictive `border`
        // AdjacencyRules. The success rate will be still moderately high - and
        // errors can be mitigated by just retrying, as non-propagating queue is faster.

        let mut analyzer = Pattern2DAnalyzer::<2, 2, BasicTypedData>::default();
        for map in maps.iter() {
            analyzer.analyze(map);
        }
        let mut resolver = Pattern2DResolver::default();

        // Save the collapse process as a GIF.
        if args.gif() {
            let file = std::fs::File::create(format!("{}{}", OUTPUTS_DIR, "overlap_position.gif"))
                .unwrap();

            let subscriber =
                GifSubscriber::new(file, &outputs_size, id_pixel_map.clone()).with_rescale(3);

            resolver = resolver.with_subscriber(Box::new(subscriber));
        }

        let to_collapse = CollapsiblePatternGrid2D::new_empty(
            outputs_size,
            analyzer.get_collection().clone(),
            analyzer.get_frequency(),
            analyzer.get_adjacency(),
        )
        .unwrap();

        let mut rng: ChaChaRng = RngHelper::init_str("overlap_position", 0)
            .with_pos(13934)
            .into();

        let after_collapse = resolver.generate_position(
            to_collapse.clone(),
            &mut rng,
            &outputs_size.get_all_possible_positions(),
            PositionQueue2D::default(),
        );

        let collapsed = after_collapse.unwrap().retrieve_collapsed();
        // We will generate output image using the same `VisCollection`.
        let mut out_buffer = init_map_image_buffer(collapsed.grid().size(), (4, 4));
        write_to_image_const_typed(&mut out_buffer, collapsed.grid(), &id_pixel_map).unwrap();

        out_buffer = image::imageops::resize(
            &out_buffer,
            outputs_size.x() * 4 * 3,
            outputs_size.y() * 4 * 3,
            image::imageops::FilterType::Nearest,
        );
        out_buffer
            .save(format!("{}{}", OUTPUTS_DIR, "overlap_position.png"))
            .unwrap();
    }
}
