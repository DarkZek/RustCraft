use criterion::{criterion_group, criterion_main, Criterion};
use rc_shared::atlas::{TEXTURE_ATLAS, TextureAtlas};
use rc_shared::block::BlockStates;
use rc_shared::block::types::VisualBlock;
use crate::chunk::get_chunk;

mod chunk;

fn bench_get_block(c: &mut Criterion) {

    TEXTURE_ATLAS.set(TextureAtlas::blank());
    let mut block_states = BlockStates::new();

    block_states.calculate_states();

    let chunk = get_chunk();

    let mut group = c.benchmark_group("draw_blocks");

    group.bench_function("draw_block_cached", |b| {
        b.iter(|| {
            std::hint::black_box(for x in 0..16 {
                for y in 0..16 {
                    for z in 0..16 {
                        let index = chunk[x][y][z];
                        let visual_block = block_states
                            .visual_block_cache
                            [index as usize]
                            .as_ref()
                            .map(|t| (**t).clone())
                            .unwrap_or_else(|| {
                                block_states.visual_block_cache[index as usize] =
                                    Some(Box::new(block_states.get_block_from_id(index).draw()));
                                *block_states.visual_block_cache[index as usize].as_ref().unwrap().clone()
                            });
                    }
                }
            });
        });
    });

    group.bench_function("draw_block", |b| {
        b.iter(|| {
            std::hint::black_box(for x in 0..16 {
                for y in 0..16 {
                    for z in 0..16 {
                        let index = chunk[x][y][z];
                        let block = block_states.get_block_from_id(index).draw();
                    }
                }
            });
        });
    });
}

criterion_group!(
    benches,
    bench_get_block,
);
criterion_main!(benches);