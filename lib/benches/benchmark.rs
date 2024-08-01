use criterion::{black_box, criterion_group, criterion_main, Criterion};
use human_ids::{HumanId, Options};

fn benchmark_generate_default(c: &mut Criterion) {
    c.bench_function("generate_default", |b| {
        b.iter(|| {
            let id = HumanId::new(None).generate();
            black_box(id);
        })
    });
}

fn benchmark_generate_custom_separator(c: &mut Criterion) {
    let options = Options::new().separator("_");

    let human_id = HumanId::new(Some(options));
    c.bench_function("generate_custom_separator", |b| {
        b.iter(|| {
            let id = human_id.generate();
            black_box(id);
        })
    });
}

fn benchmark_generate_capitalize(c: &mut Criterion) {
    let options = Options::new().capitalize(true);

    let human_id = HumanId::new(Some(options));
    c.bench_function("generate_capitalize", |b| {
        b.iter(|| {
            let id = human_id.generate();
            black_box(id);
        })
    });
}

fn benchmark_generate_add_adverb(c: &mut Criterion) {
    let options = Options::new().add_adverb(true);

    let human_id = HumanId::new(Some(options));
    c.bench_function("generate_add_adverb", |b| {
        b.iter(|| {
            let id = human_id.generate();
            black_box(id);
        })
    });
}

fn benchmark_generate_multiple_adjectives(c: &mut Criterion) {
    let options = Options::new().adjective_count(3);

    let human_id = HumanId::new(Some(options));
    c.bench_function("generate_multiple_adjectives", |b| {
        b.iter(|| {
            let id = human_id.generate();
            black_box(id);
        })
    });
}

fn benchmark_generate_separator_capitalize(c: &mut Criterion) {
    let options = Options::new().separator("_").capitalize(true);

    let human_id = HumanId::new(Some(options));
    c.bench_function("generate_separator_capitalize", |b| {
        b.iter(|| {
            let id = human_id.generate();
            black_box(id);
        })
    });
}

fn benchmark_generate_capitalize_add_adverb(c: &mut Criterion) {
    let options = Options::new().capitalize(true).add_adverb(true);

    let human_id = HumanId::new(Some(options));
    c.bench_function("generate_capitalize_add_adverb", |b| {
        b.iter(|| {
            let id = human_id.generate();
            black_box(id);
        })
    });
}

fn benchmark_generate_all_options(c: &mut Criterion) {
    let options = Options::new()
        .separator("_")
        .capitalize(true)
        .add_adverb(true)
        .adjective_count(3);

    let human_id = HumanId::new(Some(options));

    c.bench_function("generate_all_options", |b| {
        b.iter(|| {
            let id = human_id.generate();
            black_box(id);
        })
    });
}

criterion_group!(
    benches,
    benchmark_generate_default,
    benchmark_generate_custom_separator,
    benchmark_generate_capitalize,
    benchmark_generate_add_adverb,
    benchmark_generate_multiple_adjectives,
    benchmark_generate_separator_capitalize,
    benchmark_generate_capitalize_add_adverb,
    benchmark_generate_all_options,
);
criterion_main!(benches);
