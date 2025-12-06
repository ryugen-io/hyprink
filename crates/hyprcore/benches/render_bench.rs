use criterion::{criterion_group, criterion_main, Criterion};
use tera::{Context, Tera};

fn bench_render(c: &mut Criterion) {
    let mut tera = Tera::default();
    tera.add_raw_template("bench", "Hello {{ name }}! Color is {{ colors.primary }}").unwrap();
    
    let mut ctx = Context::new();
    ctx.insert("name", "User");
    ctx.insert("colors", &serde_json::json!({"primary": "#ff0000"}));

    c.bench_function("render simple template", |b| {
        b.iter(|| tera.render("bench", &ctx).unwrap())
    });
}

criterion_group!(benches, bench_render);
criterion_main!(benches);
