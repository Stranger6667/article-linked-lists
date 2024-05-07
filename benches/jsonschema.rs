use core::fmt;
use std::fs;

use criterion::{criterion_group, criterion_main, Bencher, BenchmarkId, Criterion};

use jsonschema::Validator;
use serde_json::Value;

#[derive(serde::Deserialize, Debug)]
struct Benchmark {
    schema: Value,
    instances: Vec<Instance>,
}

#[derive(serde::Deserialize, Debug)]
struct Instance {
    kind: InstanceKind,
    name: String,
    value: Value,
}

#[derive(serde::Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "lowercase")]
enum InstanceKind {
    Valid,
    Invalid,
}

impl fmt::Display for InstanceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstanceKind::Valid => f.write_str("valid"),
            InstanceKind::Invalid => f.write_str("invalid"),
        }
    }
}

impl From<InstanceKind> for String {
    fn from(value: InstanceKind) -> Self {
        value.to_string()
    }
}

const NUMBER_OF_ITERATIONS: usize = 10000;

fn benchmarks(c: &mut Criterion) {
    let file = fs::File::open("benches/data.json").expect("Failed to open benchmarks");
    let benchmark: Benchmark = serde_json::from_reader(file).expect("Failed to parse benchmarks");
    let validator = Validator::new(&benchmark.schema).expect("Failed to create validator");
    for instance in &benchmark.instances {
        // Sanity check
        let result = validator.validate(&instance.value);
        match (instance.kind, result) {
            (InstanceKind::Valid, Err(err)) => panic!("Should be valid - {err}"),
            (InstanceKind::Invalid, Ok(_)) => panic!("Should be invalid"),
            _ => {}
        };
        c.bench_with_input(
            BenchmarkId::new(instance.kind, &instance.name),
            &instance.value,
            |b: &mut Bencher, value| {
                b.iter(|| {
                    for _ in 0..NUMBER_OF_ITERATIONS {
                        let _ = validator.validate(value);
                    }
                });
            },
        );
    }
}

criterion_group!(default, benchmarks);
criterion_main!(default);
