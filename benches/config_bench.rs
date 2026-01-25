use criterion::{Criterion, black_box, criterion_group, criterion_main};
use hyprink::config::Config;
use std::fs;
use tempfile::tempdir;

fn benchmark_config_load(c: &mut Criterion) {
    // Setup a dummy config directory
    let dir = tempdir().unwrap();
    let config_dir = dir.path();
    let conf_path = config_dir.join("hyprink.conf");

    fs::write(
        &conf_path,
        r##"
[theme]
name = "bench_theme"
active_icons = "ascii"
[theme.colors]
bg = "#282a36"
fg = "#f8f8f2"
primary = "#ff79c6"
[theme.fonts]
ui = "Sans"

[icons]
[icons.nerdfont]
[icons.ascii]

[layout]
[layout.tag]
prefix = "["
suffix = "]"
transform = "uppercase"
min_width = 10
alignment = "left"
[layout.labels]
[layout.structure]
terminal = "{tag} {msg}"
file = "{tag} {msg}"
[layout.logging]
base_dir = "logs"
path_structure = "{app}.log"
filename_structure = "log"
timestamp_format = "%Y"
write_by_default = false
app_name = "benchmark"
"##,
    )
    .unwrap();

    c.bench_function("config_load_from_path", |b| {
        b.iter(|| {
            let _ = Config::load_from_path(black_box(&conf_path));
        })
    });
}

fn benchmark_config_serialization(c: &mut Criterion) {
    let dir = tempdir().unwrap();
    let config_dir = dir.path();
    let conf_path = config_dir.join("hyprink.conf");

    // Mock config file
    fs::write(
        &conf_path,
        r##"
[theme]
name = "bench_theme"
active_icons = "ascii"
[theme.colors]
bg = "#282a36"
fg = "#f8f8f2"
primary = "#ff79c6"
[theme.fonts]
ui = "Sans"

[icons]
[icons.nerdfont]
[icons.ascii]

[layout]
[layout.tag]
prefix = "["
suffix = "]"
transform = "uppercase"
min_width = 10
alignment = "left"
[layout.labels]
[layout.structure]
terminal = "{tag} {msg}"
file = "{tag} {msg}"
[layout.logging]
base_dir = "logs"
path_structure = "{app}.log"
filename_structure = "log"
timestamp_format = "%Y"
write_by_default = false
app_name = "benchmark"

[presets.bench_ok]
level = "success"
msg = "benchmark passed"
"##,
    )
    .unwrap();

    let config = Config::load_from_path(&conf_path).expect("Failed to load config");

    let bin_path = dir.path().join("bench.bin");

    c.bench_function("config_save_cache", |b| {
        b.iter(|| {
            let _ = config.save_cache(black_box(&bin_path));
        })
    });
}

criterion_group!(
    benches,
    benchmark_config_load,
    benchmark_config_serialization
);
criterion_main!(benches);
