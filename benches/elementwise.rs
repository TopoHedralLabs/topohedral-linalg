#![feature(generic_const_exprs)]
#![feature(impl_trait_in_assoc_type)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use topohedral_linalg::{Evaluate, SMatrix};

use nalgebra::SMatrix as NASMatrix;

use rand::prelude::*;


pub fn bench1(crit: &mut Criterion) {

    let a = SMatrix::<f64, 10, 10>::from_uniform_random(0.0, 10.0);
    let b = SMatrix::<f64, 10, 10>::from_uniform_random(0.0, 10.0);
    let c = SMatrix::<f64, 10, 10>::from_uniform_random(0.0, 10.0);
    let d = SMatrix::<f64, 10, 10>::from_uniform_random(0.0, 10.0);
    let e = SMatrix::<f64, 10, 10>::from_uniform_random(0.0, 10.0);
    let f = SMatrix::<f64, 10, 10>::from_uniform_random(0.0, 10.0);

    crit.bench_function("topohedral-linalg", |be| be.iter(|| {

        let g: SMatrix<f64, 10, 10> = (&f * (&a + &b) - (&c / &d) + &e).eval();
        black_box(g);
    }));
    
}

pub fn bench2(crit: &mut Criterion) {

    let range = rand::distributions::Uniform::<f64>::new(0.0, 10.0);
    let mut rng = rand::thread_rng();
    let a = NASMatrix::<f64, 10, 10>::from_distribution(&range, &mut rng);
    let b = NASMatrix::<f64, 10, 10>::from_distribution(&range, &mut rng);
    let c = NASMatrix::<f64, 10, 10>::from_distribution(&range, &mut rng);
    let d = NASMatrix::<f64, 10, 10>::from_distribution(&range, &mut rng);
    let e = NASMatrix::<f64, 10, 10>::from_distribution(&range, &mut rng);
    let f = NASMatrix::<f64, 10, 10>::from_distribution(&range, &mut rng);

    crit.bench_function("nalgebra", |be| be.iter(|| {

        let g: NASMatrix<f64, 10, 10> = &f * (&a + &b) - (c.component_div(&d)) + &e;
        black_box(g);
    }));
}

pub fn bench3(crit: &mut Criterion) {

    let range = rand::distributions::Uniform::<f64>::new(0.0, 10.0);
    let mut rng = rand::thread_rng();

    let mut a = [0.0f64; 100];
    let mut b = [0.0f64; 100];
    let mut c = [0.0f64; 100];
    let mut d = [0.0f64; 100];
    let mut e = [0.0f64; 100];
    let mut f = [0.0f64; 100];
    let mut g = [0.0f64; 100];

    for i in 0..100 {
        a[i] = range.sample(&mut rng);
        b[i] = range.sample(&mut rng);
        c[i] = range.sample(&mut rng);
        d[i] = range.sample(&mut rng);
        e[i] = range.sample(&mut rng);
        f[i] = range.sample(&mut rng);
        g[i] = range.sample(&mut rng);
    }

    crit.bench_function("arrays", |be| be.iter(|| {

        for i in 0..100 {
            g[i] = f[i] * (a[i] + b[i]) - (c[i] / d[i]) + e[i];
        }

        black_box(g);
    }));

}

criterion_group!(benches, bench1, bench2, bench3);
criterion_main!(benches);
