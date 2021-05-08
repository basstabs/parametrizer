use criterion::{black_box, criterion_group, criterion_main, Criterion};

use parametrizer::Parametrizer;

fn polynomial(t: f32) -> f32
{

    return 1.0 + (5.0*t) + (25.0*t*t);

}

fn polynomial_bench(c: &mut Criterion)
{

    c.bench_function("polynomial", |b| b.iter(|| polynomial(black_box(101.34))));

}

fn parametrizer_polynomial_bench(c: &mut Criterion)
{

    let parametrizer = Parametrizer::new("1+5*t+25*t*t").unwrap();

    c.bench_function("polynomial (parametrizer)", |b| b.iter(|| parametrizer.evaluate(black_box(101.34))));

}

fn trig(t: f32) -> f32
{

    return 2.0*f32::sin(t+5.0);

}

fn trig_bench(c: &mut Criterion)
{

    c.bench_function("trig", |b| b.iter(|| trig(black_box(1.0098))));

}

fn parametrizer_trig_bench(c: &mut Criterion)
{

    let parametrizer = Parametrizer::new("2*sin(t+5)").unwrap();

    c.bench_function("trig (parametrizer)", |b| b.iter(|| parametrizer.evaluate(black_box(1.0098))));

}

fn piecewise(t: f32) -> f32
{

    

}

fn piecewise_bench(c: &mut Criterion)
{
}

fn parametrizer_piecewise_bench(c: &mut Criterion)
{

    let parametrizer = Parametrizer::new("p

}

criterion_group!(benches, polynomial_bench, parametrizer_polynomial_bench, trig_bench, parametrizer_trig_bench);
criterion_main!(benches);
