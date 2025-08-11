use criterion::*;
use grid_forge_2d::core::{Grid2D as _, GridMap2D, GridMapShared2D, GridShared2D as _};
use grid_forge_2d::image::{TilePixConst, TilePixVar};
use grid_forge_core::id::{BasicTypedData, IdentTileDefaultBuilder, TypeIdMap};
use image::Rgb;

use grid_forge_2d::image::ops::*;

fn load_gridmap_const(c: &mut Criterion) {
    let image = image::open("../assets/samples/roads.png")
        .unwrap()
        .into_rgb8();

    c.bench_function("load_gridmap_const", |b| {
        b.iter(|| {
            let grid_map: GridMap2D<TilePixConst<4, 4, Rgb<u8>>> =
                load_from_image_const(&image).unwrap();
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
            let grid_map: GridMap2D<TilePixVar<Rgb<u8>>> =
                load_from_image_var(&image, (4, 4)).unwrap();
            black_box(grid_map);
        })
    });
}

fn load_gridmap_map_const_auto(c: &mut Criterion) {
    let builder = IdentTileDefaultBuilder::<BasicTypedData>::default();
    let image = image::open("../assets/samples/roads.png")
        .unwrap()
        .into_rgb8();

    c.bench_function("load_gridmap_map_const_auto", |b| {
        b.iter(|| {
            let mut pixel_map = TypeIdMap::<TilePixConst<4, 4, Rgb<u8>>>::default();
            let grid_map =
                load_from_image_const_typed_auto(&image, &builder, &mut pixel_map).unwrap();

            black_box(pixel_map);
            black_box(grid_map);
        })
    });
}

fn load_gridmap_map_var_auto(c: &mut Criterion) {
    let builder = IdentTileDefaultBuilder::<BasicTypedData>::default();
    let image = image::open("../assets/samples/roads.png")
        .unwrap()
        .into_rgb8();

    c.bench_function("load_gridmap_map_var_auto", |b| {
        b.iter(|| {
            let mut pixel_map = TypeIdMap::<TilePixVar<Rgb<u8>>>::default();
            let grid_map =
                load_from_image_var_typed_auto(&image, &builder, (4, 4), &mut pixel_map).unwrap();

            black_box(pixel_map);
            black_box(grid_map);
        })
    });
}

fn load_gridmap_map_const(c: &mut Criterion) {
    let builder = IdentTileDefaultBuilder::<BasicTypedData>::default();
    let image = image::open("../assets/samples/roads.png")
        .unwrap()
        .into_rgb8();

    // Load the grid map with automatic tile identification to create pixel map.
    let mut pixel_map = TypeIdMap::<TilePixConst<4, 4, Rgb<u8>>>::default();
    let _ = load_from_image_const_typed_auto(&image, &builder, &mut pixel_map).unwrap();

    c.bench_function("load_gridmap_map_const", |b| {
        b.iter(|| {
            let grid_map = load_from_image_const_typed(&image, &builder, &pixel_map).unwrap();

            black_box(grid_map);
        })
    });
}

fn load_gridmap_map_var(c: &mut Criterion) {
    let builder = IdentTileDefaultBuilder::<BasicTypedData>::default();
    let image = image::open("../assets/samples/roads.png")
        .unwrap()
        .into_rgb8();

    // Load the grid map with automatic tile identification to create pixel map.
    let mut pixel_map = TypeIdMap::<TilePixVar<Rgb<u8>>>::default();
    let _ = load_from_image_var_typed_auto(&image, &builder, (4, 4), &mut pixel_map).unwrap();

    c.bench_function("load_gridmap_map_var", |b| {
        b.iter(|| {
            let grid_map = load_from_image_var_typed(&image, &pixel_map, &builder, (4, 4)).unwrap();

            black_box(grid_map);
        })
    });
}

fn write_gridmap_const(c: &mut Criterion) {
    let image = image::open("../assets/samples/roads.png")
        .unwrap()
        .into_rgb8();

    let grid_map: GridMap2D<TilePixConst<4, 4, Rgb<u8>>> = load_from_image_const(&image).unwrap();

    c.bench_function("write_gridmap_const", |b| {
        b.iter(|| {
            let mut buffer = init_map_image_buffer(grid_map.size(), (4, 4));
            write_to_image_const(&grid_map, &mut buffer).unwrap();
        })
    });
}

fn write_gridmap_var(c: &mut Criterion) {
    let image = image::open("../assets/samples/roads.png")
        .unwrap()
        .into_rgb8();

    let grid_map: GridMap2D<TilePixVar<Rgb<u8>>> = load_from_image_var(&image, (4, 4)).unwrap();

    c.bench_function("write_gridmap_var", |b| {
        b.iter(|| {
            let mut buffer = init_map_image_buffer(grid_map.size(), (4, 4));
            write_to_image_var(&grid_map, &mut buffer, (4, 4)).unwrap();
        })
    });
}

fn write_gridmap_const_typed(c: &mut Criterion) {
    let image = image::open("../assets/samples/roads.png")
        .unwrap()
        .into_rgb8();

    let builder = IdentTileDefaultBuilder::<BasicTypedData>::default();
    let mut pixel_map = TypeIdMap::<TilePixConst<4, 4, Rgb<u8>>>::default();
    let grid_map = load_from_image_const_typed_auto(&image, &builder, &mut pixel_map).unwrap();

    c.bench_function("write_gridmap_const_typed", |b| {
        b.iter(|| {
            let mut buffer = init_map_image_buffer(grid_map.size(), (4, 4));
            write_to_image_const_typed(&mut buffer, &grid_map, &pixel_map).unwrap();
        })
    });
}

fn write_gridmap_var_typed(c: &mut Criterion) {
    let image = image::open("../assets/samples/roads.png")
        .unwrap()
        .into_rgb8();

    let builder = IdentTileDefaultBuilder::<BasicTypedData>::default();
    let mut pixel_map = TypeIdMap::<TilePixVar<Rgb<u8>>>::default();
    let grid_map =
        load_from_image_var_typed_auto(&image, &builder, (4, 4), &mut pixel_map).unwrap();

    c.bench_function("write_gridmap_var_typed", |b| {
        b.iter(|| {
            let mut buffer = init_map_image_buffer(grid_map.size(), (4, 4));
            write_to_image_var_typed(&mut buffer, &grid_map, &pixel_map, (4, 4)).unwrap();
        })
    });
}

fn write_gridmap_const_shared(c: &mut Criterion) {
    let image = image::open("../assets/samples/roads.png")
        .unwrap()
        .into_rgb8();

    let builder = IdentTileDefaultBuilder::<BasicTypedData>::default();
    let mut pixel_map = TypeIdMap::<TilePixConst<4, 4, Rgb<u8>>>::default();
    let grid_map = load_from_image_const_typed_auto(&image, &builder, &mut pixel_map).unwrap();

    let mut grid_map = GridMapShared2D::from_regular(grid_map);
    grid_map.import_shared_data(pixel_map);

    c.bench_function("write_gridmap_const_shared", |b| {
        b.iter(|| {
            let mut buffer = init_map_image_buffer(grid_map.size(), (4, 4));
            write_to_image_const_shared(&mut buffer, &grid_map).unwrap();
        })
    });
}

fn write_gridmap_var_shared(c: &mut Criterion) {
    let image = image::open("../assets/samples/roads.png")
        .unwrap()
        .into_rgb8();

    let builder = IdentTileDefaultBuilder::<BasicTypedData>::default();
    let mut pixel_map = TypeIdMap::<TilePixVar<Rgb<u8>>>::default();
    let grid_map =
        load_from_image_var_typed_auto(&image, &builder, (4, 4), &mut pixel_map).unwrap();

    let mut grid_map = GridMapShared2D::from_regular(grid_map);
    grid_map.import_shared_data(pixel_map);

    c.bench_function("write_gridmap_var_shared", |b| {
        b.iter(|| {
            let mut buffer = init_map_image_buffer(grid_map.size(), (4, 4));
            write_to_image_var_shared(&mut buffer, &grid_map, (4, 4)).unwrap();
        })
    });
}

criterion_group!(
    name = read;
    config = Criterion::default();
    targets =   load_gridmap_const, load_gridmap_var,
                load_gridmap_map_const_auto, load_gridmap_map_var_auto,
                load_gridmap_map_const, load_gridmap_map_var,
);

criterion_group!(
    name = write;
    config = Criterion::default();
    targets =   write_gridmap_const, write_gridmap_var,
                write_gridmap_const_typed, write_gridmap_var_typed,
                write_gridmap_const_shared, write_gridmap_var_shared,
);

criterion_main!(read, write);
