use criterion::{black_box, criterion_group, criterion_main, Criterion};

use parametrizer::Parametrizer;

fn polynomial(t: f32) -> f32
{

    return 1.0 + (5.0*t) + (25.0*t*t);

}

fn polynomial_64(t: f64) -> f64
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

    if t >= 6.0
    {

        return 9.0-t;

    }
    else if t >= 2.0
    {

        return f32::sin(t)

    }
    else
    {

        return 2.0*t;

    }

}

fn piecewise_bench(c: &mut Criterion)
{

    c.bench_function("piecewise - First branch", |b| b.iter(|| piecewise(black_box(1.0))));
    c.bench_function("piecewise - Second branch", |b| b.iter(|| piecewise(black_box(4.5))));
    c.bench_function("piecewise - Third branch", |b| b.iter(|| piecewise(black_box(7.1))));

}

fn parametrizer_piecewise_bench(c: &mut Criterion)
{

    let parametrizer = Parametrizer::new("p2*t>0|sin(t)>2|9-t>6").unwrap();

    c.bench_function("piecewise (parametrizer) - First branch", |b| b.iter(|| parametrizer.evaluate(black_box(1.0))));
    c.bench_function("piecewise (parametrizer) - Second branch", |b| b.iter(|| parametrizer.evaluate(black_box(4.5))));
    c.bench_function("piecewise (parametrizer) - Third branch", |b| b.iter(|| parametrizer.evaluate(black_box(7.1))));

}

fn function_benchmark(c: &mut Criterion)
{

    let parametrizer = Parametrizer::new_functions("poly(t)", vec![parametrizer::ParametrizerFunction::new("poly".to_string(), polynomial_64)]).unwrap();

    c.bench_function("polynomial (64)", |b| b.iter(|| polynomial_64(black_box(1.011))));
    c.bench_function("polynomial (64, wrapped)", |b| b.iter(|| parametrizer.evaluate(black_box(1.011))));

}

criterion_group!(benches, polynomial_bench, parametrizer_polynomial_bench, trig_bench, parametrizer_trig_bench, piecewise_bench, parametrizer_piecewise_bench, function_benchmark);
criterion_main!(benches);
