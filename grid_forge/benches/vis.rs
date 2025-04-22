<<<<<<<< HEAD:grid-forge/benches/vis_io.rs
use criterion::{criterion_group, criterion_main, Criterion};

========
>>>>>>>> feature/gd_update:grid_forge/benches/vis.rs
use grid_forge::{
    identifiable::{builders::IdentTileTraitBuilder, BasicIdentTileData},
    vis::{
        collection::VisCollection,
        ops::{
            init_map_image_buffer, load_gridmap_identifiable_auto,
            load_gridmap_identifiable_manual, write_gridmap_identifiable,
        },
        DefaultVisPixel,
    },
};

<<<<<<<< HEAD:grid-forge/benches/vis_io.rs
========
use criterion::*;

>>>>>>>> feature/gd_update:grid_forge/benches/vis.rs
fn load_gridmap_auto(c: &mut Criterion) {
    let builder = IdentTileTraitBuilder::<BasicIdentTileData>::default();
    let image = image::open("../assets/samples/roads.png")
        .unwrap()
        .into_rgb8();

    c.bench_function("load_gridmap_auto", |b| {
        b.iter(|| {
            let mut collection = VisCollection::<DefaultVisPixel, 4, 4>::default();
            load_gridmap_identifiable_auto(&image, &mut collection, &builder).unwrap();
<<<<<<<< HEAD:grid-forge/benches/vis_io.rs
        });
========
        })
>>>>>>>> feature/gd_update:grid_forge/benches/vis.rs
    });
}

fn load_gridmap_manual(c: &mut Criterion) {
    let builder = IdentTileTraitBuilder::<BasicIdentTileData>::default();
    let image = image::open("../assets/samples/roads.png")
        .unwrap()
        .into_rgb8();

    let mut collection = VisCollection::<DefaultVisPixel, 4, 4>::default();
    load_gridmap_identifiable_auto(&image, &mut collection, &builder).unwrap();

    c.bench_function("load_gridmap_manual", |b| {
        b.iter(|| {
            load_gridmap_identifiable_manual(&image, &collection, &builder).unwrap();
<<<<<<<< HEAD:grid-forge/benches/vis_io.rs
        });
    });
}

fn write_gridmap_ident(c: &mut Criterion) {
========
        })
    });
}

fn write_grimap_ident(c: &mut Criterion) {
>>>>>>>> feature/gd_update:grid_forge/benches/vis.rs
    let builder = IdentTileTraitBuilder::<BasicIdentTileData>::default();
    let image = image::open("../assets/samples/roads.png")
        .unwrap()
        .into_rgb8();
    let mut collection = VisCollection::<DefaultVisPixel, 4, 4>::default();
    let gridmap = load_gridmap_identifiable_auto(&image, &mut collection, &builder).unwrap();

<<<<<<<< HEAD:grid-forge/benches/vis_io.rs
    c.bench_function("write_gridmap_ident", |b| {
        b.iter(|| {
            let mut buffer = init_map_image_buffer::<DefaultVisPixel, 4, 4>(gridmap.size());
            write_gridmap_identifiable(&mut buffer, &gridmap, &collection).unwrap();
        });
========
    c.bench_function("write_grimap_ident", |b| {
        b.iter(|| {
            let mut buffer = init_map_image_buffer::<DefaultVisPixel, 4, 4>(gridmap.size());
            write_gridmap_identifiable(&mut buffer, &gridmap, &collection).unwrap();
        })
>>>>>>>> feature/gd_update:grid_forge/benches/vis.rs
    });
}

criterion_group!(
    benches,
    load_gridmap_auto,
    load_gridmap_manual,
<<<<<<<< HEAD:grid-forge/benches/vis_io.rs
    write_gridmap_ident
========
    write_grimap_ident,
>>>>>>>> feature/gd_update:grid_forge/benches/vis.rs
);
criterion_main!(benches);
