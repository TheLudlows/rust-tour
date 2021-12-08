use criterion::{BatchSize, Bencher, BenchmarkId, Criterion};
use kv_store::{Allocator, Arena};
static CHUNK_SIZE: [usize; 4] = [256, 1024, 4096, 10000];

fn bench_offset_arena_allocate(c: &mut Criterion) {
    let mut group = c.benchmark_group("Arena::allocate");
    for size in CHUNK_SIZE.clone().iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b: &mut Bencher, size| {
                b.iter_batched(
                    || Arena::with_capacity(*size),
                    |arena| { },
                    BatchSize::PerIteration,
                );
            },
        );
    }
}


fn bench_vector(c: &mut Criterion) {
    let mut group = c.benchmark_group("Vec::new");
    for size in CHUNK_SIZE.clone().iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b: &mut Bencher, size| {
                b.iter_batched(
                    || {let v: Vec<u64> = Vec::with_capacity(*size >> 3); v},
                    |v |{  },
                    BatchSize::PerIteration,
                );
            },
        );
    }
}




pub fn bench_arena(c: &mut Criterion) {
    bench_offset_arena_allocate(c);
}

pub fn bench_vec(c: &mut Criterion) {
    bench_vector(c);
}