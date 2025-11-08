use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use std::arch::x86_64::_rdtsc;
use std::fs::File;
use std::io::Cursor;
use std::path::PathBuf;
use std::sync::{Arc, OnceLock, RwLock};
use std::time::Duration;
use gvas::game_version::GameVersion;
use gvas::GvasFile;
use gammaeditor::save::pokemon::pokemon_info::PokemonInfo;
use gammaeditor::{try_gvas_read, try_gvas_write};

fn bench_gvas_read(c: &mut Criterion) {
    let path = PathBuf::from("benches/resource/Slot1.sav");
    let bytes = std::fs::read(&path).expect("Failed to read save file");

    let mut group = c.benchmark_group("gvas_read");
    group.measurement_time(Duration::from_secs(5));

    group.bench_function("read (time)", |b| {
                b.iter(|| {
            let mut cursor = Cursor::new(black_box(&bytes));
            let gvas = GvasFile::read(&mut cursor, GameVersion::Default).unwrap();
            black_box(gvas);
        });
    });

    group.bench_function("read (cycles)", |b| {
        b.iter_custom(|iters| {
            let mut total_cycles: u64 = 0;
            for _ in 0..iters {
                unsafe {
                    let start = _rdtsc();
                    let mut cursor = Cursor::new(&bytes);
                    let gvas = GvasFile::read(&mut cursor, GameVersion::Default).unwrap();
                    let end = _rdtsc();
                    total_cycles += end - start;
                    std::hint::black_box(gvas);
                }
            }

            let avg_cycles = total_cycles as f64 / iters as f64;
            println!("Average cycles per iteration: {:.0}", avg_cycles);
            Duration::from_secs(1)
        });
    });

    let mut gvas: GvasFile = GvasFile::read(&mut Cursor::new(&bytes), GameVersion::Default).unwrap();
    group.bench_function("get a deep struct (time)", |b| {
       b.iter(|| {
           let g = black_box(&gvas);

            let res = PokemonInfo::new_party(g)
                .unwrap()
                .get_stats(2)
                .unwrap();

           black_box(res);
       })
    });

    group.bench_function("get a deep struct (cycles)", |b| {
        b.iter_custom(|iters| {
            let mut total_cycles: u64 = 0;
            for _ in 0..iters {
                unsafe {
                    let start = _rdtsc();

                    let g = black_box(&gvas);

                    let res = PokemonInfo::new_party(g)
                        .unwrap()
                        .get_stats(2)
                        .unwrap();

                    let end = _rdtsc();
                    total_cycles += end - start;

                    black_box(res);
                }
            }

            let avg_cycles = total_cycles as f64 / iters as f64;
            println!("Average cycles per iteration: {:.0}", avg_cycles);
            Duration::from_secs(1)
        });
    });

    let  gvas: GvasFile = GvasFile::read(&mut Cursor::new(&bytes), GameVersion::Default).unwrap();
    pub static FAKE_STATIC: OnceLock<Arc<RwLock<GvasFile>>> = OnceLock::<Arc<RwLock<GvasFile>>>::new();
    FAKE_STATIC.set(Arc::new(RwLock::new(gvas)));
    group.bench_function("try gvas read (time)", |b| {
        b.iter(|| {
            let res = try_gvas_read!(FAKE_STATIC);

            black_box(res);
        })
    });

    group.bench_function("try gvas write (time)", |b| {
        b.iter(|| {
            let res = try_gvas_write!(FAKE_STATIC);

            black_box(res);
        })
    });

    group.bench_function("try gvas read (cycles)", |b| {
        b.iter_custom(|iters| {
            let mut total_cycles: u64 = 0;
            for _ in 0..iters {
                unsafe {
                    let start = _rdtsc();

                    let res = try_gvas_read!(FAKE_STATIC);

                    let end = _rdtsc();
                    total_cycles += end - start;

                    black_box(res);
                }
            }

            let avg_cycles = total_cycles as f64 / iters as f64;
            println!("Average cycles per iteration: {:.0}", avg_cycles);
            Duration::from_secs(1)
        });
    });

    group.bench_function("try gvas write (cycles)", |b| {
        b.iter_custom(|iters| {
            let mut total_cycles: u64 = 0;
            for _ in 0..iters {
                unsafe {
                    let start = _rdtsc();

                    let res = try_gvas_write!(FAKE_STATIC);

                    let end = _rdtsc();
                    total_cycles += end - start;

                    black_box(res);
                }
            }

            let avg_cycles = total_cycles as f64 / iters as f64;
            println!("Average cycles per iteration: {:.0}", avg_cycles);
            Duration::from_secs(1)
        });
    });

    group.finish();
}

criterion_group!(benches, bench_gvas_read);
criterion_main!(benches);
