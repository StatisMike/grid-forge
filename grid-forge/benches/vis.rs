
use criterion::*;
use godot::classes::GridMap;
use grid_forge::{id::{BasicTypedData, IdentTileDefaultBuilder}, two_d::{GridMap2D, GridMapShared2D}, vis::{grid::init_map_image_buffer, tile::{TilePixConst, TilePixVar}}};
use image::Rgb;

fn load_gridmap_const(c: &mut Criterion) {
    let image = image::open("../assets/samples/roads.png")
    .unwrap()
    .into_rgb8();

    c.bench_function("load_gridmap_const", |b| {
        b.iter(|| {
            let grid_map = GridMap2D::<TilePixConst<4, 4, Rgb<u8>>>::load_from_image((4, 4), &image).unwrap();
            black_box(grid_map);
        })
    });
}

fn load_gridmap_var(c: &mut Criterion) {
    let image = image::open("../assets/samples/roads.png")
    .unwrap()
    .into_rgb8();

    c.bench_function("load_gridmap_var", |b| {
        b.iter(|| {
            let grid_map = GridMap2D::<TilePixVar<Rgb<u8>>>::load_from_image((4, 4), &image).unwrap();
            black_box(grid_map);
        })
    });
}

fn load_gridmap_auto_const(c: &mut Criterion) {
    let builder = IdentTileDefaultBuilder::<BasicTypedData>::default();
    let image = image::open("../assets/samples/roads.png")
        .unwrap()
        .into_rgb8();

    c.bench_function("load_gridmap_auto_const", |b| {
        b.iter(|| {
            let grid_map = GridMap2D::load_from_image_typed_auto::<TilePixConst<4, 4, _>, _, _>((4, 4), &image, &builder).unwrap();
            black_box(grid_map);
        })
    });
}

fn load_gridmap_auto_var(c: &mut Criterion) {
    let builder = IdentTileDefaultBuilder::<BasicTypedData>::default();
    let image = image::open("../assets/samples/roads.png")
        .unwrap()
        .into_rgb8();

    c.bench_function("load_gridmap_auto_var", |b| {
        b.iter(|| {
            let grid_map = GridMap2D::load_from_image_typed_auto::<TilePixVar<_>, _,  _>((4, 4), &image, &builder).unwrap();
            black_box(grid_map);
        })
    });
}

fn load_gridmap_map_const(c: &mut Criterion) {
    let builder = IdentTileDefaultBuilder::<BasicTypedData>::default();
    let image = image::open("../assets/samples/roads.png")
        .unwrap()
        .into_rgb8();

    let (_, map) = GridMap2D::load_from_image_typed_auto::<TilePixConst<4, 4, _>, _, _>((4, 4), &image, &builder).unwrap();

    c.bench_function("load_gridmap_map_const", |b| {
        b.iter(|| {
            let grid_map = GridMap2D::load_from_image_typed((4,4), &image, &builder, &map).unwrap();
            black_box(grid_map);
        })
    });
}

fn load_gridmap_map_var(c: &mut Criterion) {
    let builder = IdentTileDefaultBuilder::<BasicTypedData>::default();
    let image = image::open("../assets/samples/roads.png")
        .unwrap()
        .into_rgb8();

    let (_, map) = GridMap2D::load_from_image_typed_auto::<TilePixVar<_>, _, _>((4, 4), &image, &builder).unwrap();

    c.bench_function("load_gridmap_map_var", |b| {
        b.iter(|| {
            let grid_map = GridMap2D::load_from_image_typed((4,4), &image, &builder, &map).unwrap();
            black_box(grid_map);
        })
    });
}

fn write_gridmap_const(c: &mut Criterion) {
    let image = image::open("../assets/samples/roads.png")
    .unwrap()
    .into_rgb8();

    let grid_map = GridMap2D::<TilePixConst<4, 4, Rgb<u8>>>::load_from_image((4, 4), &image).unwrap();

    c.bench_function("write_gridmap_const", |b| {
        b.iter(|| {
            let mut buffer = init_map_image_buffer(grid_map.size(), (4, 4));
            grid_map.write_to_image(&mut buffer, (4, 4)).unwrap();
        })
    });
}

fn write_gridmap_var(c: &mut Criterion) {
    let image = image::open("../assets/samples/roads.png")
    .unwrap()
    .into_rgb8();

    let grid_map = GridMap2D::<TilePixVar<Rgb<u8>>>::load_from_image((4, 4), &image).unwrap();

    c.bench_function("write_gridmap_var", |b| {
        b.iter(|| {
            let mut buffer = init_map_image_buffer(grid_map.size(), (4, 4));
            grid_map.write_to_image(&mut buffer, (4, 4)).unwrap();
        })
    });
}

fn write_gridmap_const_typed(c: &mut Criterion) {
    let image = image::open("../assets/samples/roads.png")
    .unwrap()
    .into_rgb8();

    let builder = IdentTileDefaultBuilder::<BasicTypedData>::default();
    let (grid_map, pix_map) = GridMap2D::load_from_image_typed_auto::<TilePixConst<4, 4, _>,_, _>((4, 4), &image, &builder).unwrap();

    c.bench_function("write_gridmap_const_typed", |b| {
        b.iter(|| {
            let mut buffer = init_map_image_buffer(grid_map.size(), (4, 4));
            grid_map.write_to_image_typed(&mut buffer, &pix_map, (4, 4)).unwrap();
        })
    });
}

fn write_gridmap_var_typed(c: &mut Criterion) {
    let image = image::open("../assets/samples/roads.png")
    .unwrap()
    .into_rgb8();

    let builder = IdentTileDefaultBuilder::<BasicTypedData>::default();
    let (grid_map, pix_map) = GridMap2D::load_from_image_typed_auto::<TilePixVar<_>, _, _>((4, 4), &image, &builder).unwrap();

    c.bench_function("write_gridmap_var_typed", |b| {
        b.iter(|| {
            let mut buffer = init_map_image_buffer(grid_map.size(), (4, 4));
            grid_map.write_to_image_typed(&mut buffer, &pix_map, (4, 4)).unwrap();
        })
    });
}

fn write_gridmap_const_shared(c: &mut Criterion) {
    let image = image::open("../assets/samples/roads.png")
    .unwrap()
    .into_rgb8();

    let builder = IdentTileDefaultBuilder::<BasicTypedData>::default();
    let (grid_map, pix_map) = GridMap2D::load_from_image_typed_auto::<TilePixConst<4, 4, _>, _,  _>((4, 4), &image, &builder).unwrap();

    let mut grid_map = GridMapShared2D::from_regular(grid_map);
    grid_map.import_shared_data(pix_map);

    c.bench_function("write_gridmap_const_shared", |b| {
        b.iter(|| {
            let mut buffer = init_map_image_buffer(grid_map.size(), (4, 4));
            grid_map.write_to_image(&mut buffer, (4, 4)).unwrap();
        })
    });
}

fn write_gridmap_var_shared(c: &mut Criterion) {
    let image = image::open("../assets/samples/roads.png")
    .unwrap()
    .into_rgb8();

    let builder = IdentTileDefaultBuilder::<BasicTypedData>::default();
    let (grid_map, pix_map) = GridMap2D::load_from_image_typed_auto::<TilePixVar<_>, _, _>((4, 4), &image, &builder).unwrap();

    let mut grid_map = GridMapShared2D::from_regular(grid_map);
    grid_map.import_shared_data(pix_map);

    c.bench_function("write_gridmap_var_shared", |b| {
        b.iter(|| {
            let mut buffer = init_map_image_buffer(grid_map.size(), (4, 4));
            grid_map.write_to_image(&mut buffer, (4, 4)).unwrap();
        })
    });
}


// fn load_gridmap_manual(c: &mut Criterion) {
//     let builder = IdentTileTraitBuilder::<BasicIdentTileData>::default();
//     let image = image::open("../assets/samples/roads.png")
//         .unwrap()
//         .into_rgb8();

//     let mut collection = VisCollection::<DefaultVisPixel, 4, 4>::default();
//     load_gridmap_identifiable_auto(&image, &mut collection, &builder).unwrap();

//     c.bench_function("load_gridmap_manual", |b| {
//         b.iter(|| {
//             load_gridmap_identifiable_manual(&image, &collection, &builder).unwrap();
//         })
//     });
// }

// fn write_grimap_ident(c: &mut Criterion) {
//     let builder = IdentTileTraitBuilder::<BasicIdentTileData>::default();
//     let image = image::open("../assets/samples/roads.png")
//         .unwrap()
//         .into_rgb8();
//     let mut collection = VisCollection::<DefaultVisPixel, 4, 4>::default();
//     let gridmap = load_gridmap_identifiable_auto(&image, &mut collection, &builder).unwrap();

//     c.bench_function("write_grimap_ident", |b| {
//         b.iter(|| {
//             let mut buffer = init_map_image_buffer::<DefaultVisPixel, 4, 4>(gridmap.size());
//             write_gridmap_identifiable(&mut buffer, &gridmap, &collection).unwrap();
//         })
//     });
// }

criterion_group!(
    benches,
    // load_gridmap_const,
    // load_gridmap_var,
    // load_gridmap_auto_const,
    // load_gridmap_auto_var,
    // load_gridmap_map_const,
    // load_gridmap_map_var,
    // write_gridmap_const,
    // write_gridmap_var,
    write_gridmap_const_typed,
    write_gridmap_var_typed,
    write_gridmap_const_shared,
    write_gridmap_var_shared,
);
criterion_main!(benches);
