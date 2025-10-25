use arrow_array::{Array, BooleanArray, Int32Array};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use arrow_arith::boolean::not;
use arrow_select::filter::{FilterBuilder, SplitBuilder, SplitPredicate};

fn criterion_benchmark(c: &mut Criterion) {
    let array = Int32Array::from_iter_values(0..8192);
    let selection = BooleanArray::from([vec![true;4096],vec![false;4096]].concat());


    c.bench_function("split_two_pass", |b| {
        b.iter(|| {
            let filter = FilterBuilder::new(&selection).build();
            let left = black_box(filter.filter(&array).unwrap());
            let inverted = not(&selection).unwrap();
            let filter = FilterBuilder::new(&inverted).build();
            let right = black_box(filter.filter(&inverted).unwrap());
            let length = left.len() + right.len();
            assert_eq!(length, array.len());
        });
    });

    c.bench_function("split_one_pass", |b| {
        b.iter(|| {
            let split = SplitBuilder::new(&selection).optimize().build();
            let (left, right) = black_box(split.split(&array).unwrap());
            let length = left.len() + right.len();
            assert_eq!(length, array.len());
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);