use std::cell::RefCell;
use std::collections::HashMap;
use bevy::render::render_resource::encase::private::RuntimeSizedArray;
use criterion::{criterion_group, criterion_main, Criterion};
use fnv::{FnvBuildHasher, FnvHashMap};
use sparse_set::SparseSet;
use rc_shared::atlas::{TEXTURE_ATLAS, TextureAtlas};
use rc_shared::block::BlockStates;
use rc_shared::block::types::VisualBlock;
use crate::chunk::get_chunk;

mod chunk;

thread_local! {
    pub static CACHE: RefCell<Vec<Option<Box<VisualBlock>>>> = RefCell::new(Vec::new());
}

// This tests different visualblock caching strategies
fn bench_get_block(c: &mut Criterion) {

    TEXTURE_ATLAS.set(TextureAtlas::blank());
    let mut block_states = BlockStates::new();

    block_states.calculate_states();

    CACHE.set(vec![None; 100]);

    let chunk = get_chunk();

    let mut group = c.benchmark_group("draw_blocks");

    // Store them in a huge vec
    group.bench_function("vec", |b| {
        b.iter(|| {
            std::hint::black_box({
                CACHE.with_borrow_mut(|cache| {
                    for x in 0..16 {
                        for y in 0..16 {
                            for z in 0..16 {
                                let index = chunk[x][y][z];

                                let block = match cache.get(index as usize).unwrap() {
                                    Some(v) => &**v,
                                    None => {
                                        cache[index as usize] =
                                            Some(Box::new(block_states.get_block_from_id(index).draw()));

                                        cache[index as usize].as_ref().unwrap()
                                    }
                                };
                            }
                        }
                    }
                })
            });
        });
    });

    // Store them in a hashmap
    let mut cache: HashMap<u32, VisualBlock, FnvBuildHasher> = FnvHashMap::default();
    group.bench_function("fnvhashmap", |b| {
        b.iter(|| {
            std::hint::black_box({
                for x in 0..16 {
                    for y in 0..16 {
                        for z in 0..16 {
                            let index = chunk[x][y][z];

                            let block = match cache.get(&index) {
                                Some(v) => v,
                                None => {
                                    cache.insert(index, block_states.get_block_from_id(index).draw());

                                    cache.get(&index).unwrap()
                                }
                            };
                        }
                    }
                }
            });
        });
    });

    // Store them in a sparseset
    let mut cache: SparseSet<usize, VisualBlock> = SparseSet::default();
    group.bench_function("sparseset", |b| {
        b.iter(|| {
            std::hint::black_box({
                for x in 0..16 {
                    for y in 0..16 {
                        for z in 0..16 {
                            let index = chunk[x][y][z];

                            let block = match cache.get(index as usize) {
                                Some(v) => v,
                                None => {
                                    cache.insert(index as usize, block_states.get_block_from_id(index).draw());

                                    cache.get(index as usize).unwrap()
                                }
                            };
                        }
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