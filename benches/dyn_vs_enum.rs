use diol::prelude::*;

use polymorph::*;

fn main() -> std::io::Result<()> {
    let mut bench = Bench::new(BenchConfig::from_args()?);
    bench.register(bench_dynamic_impl_with_existing, [20_000, 200_000, 2_000_000]);
    bench.register(bench_dynamic_impl_with_new, [20_000, 200_000, 2_000_000]);
    bench.register(bench_enum_impl, [20_000, 200_000, 2_000_000]);
    bench.run()?;
    Ok(())
}

// динамическая реализация с только библиотечными типами
fn bench_dynamic_impl_with_existing(bencher: Bencher, num_of_each: usize) {
    // AllComponents из модуля dynamic_impl
    let mut components = dynamic_impl::AllComponents::new();

    for i in 0..num_of_each {
        components.add_rect(Rectangle { 
            pos: Point { x: 10. + i as f32, y: 11. + i as f32 },
            w: 10. + i as f32,
            h: 18. + i as f32,
            color: Color::RED,
        });
        components.add_star(Star { 
            pos: Point { x: 1. + i as f32, y: 18. + i as f32 },
            radius: 15. + i as f32,
            color: Color::GREEN,
        });
        components.add_triangle(Triangle { 
            p1: Point { x: 10. + i as f32, y: 18. + i as f32 },
            p2: Point { x: 20. + i as f32, y: 30. + i as f32 },
            p3: Point { x: 15. + i as f32, y: 40. + i as f32 },
            color: Color::BLUE,
        });
    }

    bencher.bench(|| {
        components.draw(&mut Screen)
    });

    black_box(components);
}

#[derive(Clone, Copy)]
pub struct Cube {
    pub rect: Rectangle,
    pub depth: f32,
}

impl Draw for Cube {
    fn draw(&self, screen: &mut Screen) {
        screen.fill_whatever(*self);
    }
}

// динамическая реализация с новым типом, созданным пользователем.
fn bench_dynamic_impl_with_new(bencher: Bencher, num_of_each: usize) {
    // AllComponents из модуля dynamic_impl
    let mut components = dynamic_impl::AllComponents::new();

    for i in 0..num_of_each {
        components.add_rect(Rectangle { 
            pos: Point { x: 10. + i as f32, y: 11. + i as f32 },
            w: 10. + i as f32,
            h: 18. + i as f32,
            color: Color::RED,
        });
        // тип Cube создан пользователем
        components.add_user_component(Cube {
            rect: Rectangle { 
                pos: Point { x: 10. + i as f32, y: 11. + i as f32 },
                w: 10. + i as f32,
                h: 18. + i as f32,
                color: Color::RED,
            },
            depth: 15. + i as f32,
        });
        components.add_triangle(Triangle { 
            p1: Point { x: 10. + i as f32, y: 18. + i as f32 },
            p2: Point { x: 20. + i as f32, y: 30. + i as f32 },
            p3: Point { x: 15. + i as f32, y: 40. + i as f32 },
            color: Color::BLUE,
        });
    }

    bencher.bench(|| {
        components.draw(&mut Screen)
    });

    black_box(components);
}

// enum реализация
fn bench_enum_impl(bencher: Bencher, num_of_each: usize) {
    // AllComponents из модуля enum_impl
    let mut components = enum_impl::AllComponents::new();

    for i in 0..num_of_each {
        components.add_rect(Rectangle { 
            pos: Point { x: 10. + i as f32, y: 11. + i as f32 },
            w: 10. + i as f32,
            h: 18. + i as f32,
            color: Color::RED,
        });
        components.add_star(Star { 
            pos: Point { x: 1. + i as f32, y: 18. + i as f32 },
            radius: 15. + i as f32,
            color: Color::GREEN,
        });
        components.add_triangle(Triangle { 
            p1: Point { x: 10. + i as f32, y: 18. + i as f32 },
            p2: Point { x: 20. + i as f32, y: 30. + i as f32 },
            p3: Point { x: 15. + i as f32, y: 40. + i as f32 },
            color: Color::BLUE,
        });
    }

    bencher.bench(|| {
        components.draw(&mut Screen)
    });

    black_box(components);
}
