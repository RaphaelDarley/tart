use art::SparseArray;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::mem;

fn bench_vec_array_push(c: &mut Criterion) {
    let mut vec_array: SparseArray<u64, 1000> = SparseArray::new();
    c.bench_function("vec_array_push", |b| {
        b.iter(|| {
            for i in 0..1000 {
                vec_array.set(i, i as u64);
            }
        })
    });

    let memory_usage = mem::size_of_val(&vec_array);
    println!("vec_array_memory: {}", memory_usage);
}

fn bench_vec_array_get(c: &mut Criterion) {
    let mut vec_array: SparseArray<u32, 1000> = SparseArray::new();
    for i in 0..1000 {
        vec_array.push(i);
    }
    c.bench_function("vec_array_get", |b| {
        b.iter(|| {
            for i in 0..1000 {
                black_box(vec_array.get(i));
            }
        })
    });
}

criterion_group!(benches, bench_vec_array_push, bench_vec_array_get);
criterion_main!(benches);
