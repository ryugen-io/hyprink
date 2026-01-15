use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use hi_core::db::Store;
use hi_core::template::{Template, TemplateManifest};
use tempfile::NamedTempFile;

fn create_dummy_template(id: usize) -> Template {
    Template {
        manifest: TemplateManifest {
            name: format!("tpl_{}", id),
            description: "Benchmark template".to_string(),
            version: "1.0.0".to_string(),
            authors: vec![],
            repository: None,
            license: None,
            ignored: false,
        },
        targets: vec![],
        files: vec![],
        hooks: Default::default(),
    }
}

pub fn db_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("store_db");

    // Bench: Save DB (Serialization)
    group.bench_function("save_100_templates", |b| {
        b.iter_batched(
            || {
                let file = NamedTempFile::new().unwrap();
                let path = file.path().to_path_buf();
                let mut db = Store::load(&path).unwrap();
                for i in 0..100 {
                    db.add(create_dummy_template(i)).unwrap();
                }
                (db, file) // keep file alive
            },
            |(db, _file): (Store, NamedTempFile)| {
                db.save().unwrap();
            },
            BatchSize::SmallInput,
        );
    });

    // Bench: Load DB (Deserialization)
    group.bench_function("load_100_templates", |b| {
        // Setup: Create a pre-filled DB file
        let file = NamedTempFile::new().unwrap();
        let path = file.path().to_path_buf();
        {
            let mut db = Store::load(&path).unwrap();
            for i in 0..100 {
                db.add(create_dummy_template(i)).unwrap();
            }
            db.save().unwrap();
        }

        b.iter(|| {
            Store::load(&path).unwrap();
        });
    });

    // Bench: Add (Ingestion)
    group.bench_function("add_single", |b| {
        b.iter_batched(
            || {
                let file = NamedTempFile::new().unwrap();
                let path = file.path().to_path_buf();
                let db = Store::load(&path).unwrap();
                let tpl = create_dummy_template(999);
                (db, tpl, file)
            },
            |(mut db, tpl, _file): (Store, Template, NamedTempFile)| {
                db.add(tpl).unwrap();
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

criterion_group!(benches, db_benchmarks);
criterion_main!(benches);
