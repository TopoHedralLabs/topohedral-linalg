#![feature(generic_const_exprs)]
#![feature(impl_trait_in_assoc_type)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nalgebra::DMatrix as NADMatrix;
use rand::prelude::*;
use topohedral_linalg::dmatrix;
use topohedral_linalg::dmatrix::EvaluateDMatrix;

// fn somethin() {

//     let range = rand::distributions::Uniform::<f64>::new(0.0, 10.0);

//     let mut rng = rand::thread_rng();

//     let a = NADMatrix::<f64>::from_distribution(10, 10, &range, &mut rng);

//     let b = NADMatrix::<f64>::from_distribution(10, 10, &range, &mut rng);

//     let c = a + b;

// }

//{{{ collection: DMatrix benches
macro_rules! add_benches_dmatrix {
    ($dim: expr, $name1: ident, $name2: ident, $name3: ident) => {
        pub fn $name1(crit: &mut Criterion)
        {
            let a = dmatrix::DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim, $dim);

            let b = dmatrix::DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim, $dim);

            let c = dmatrix::DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim, $dim);

            let d = dmatrix::DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim, $dim);

            let e = dmatrix::DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim, $dim);

            let f = dmatrix::DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim, $dim);

            let g = dmatrix::DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim, $dim);

            let h = dmatrix::DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim, $dim);

            let i = dmatrix::DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim, $dim);

            crit.bench_function(
                format!("topohedral-linalg_dmatrix{}", $dim).as_str(),
                |be| {
                    be.iter(|| {
                        let j: dmatrix::DMatrix<f64> =
                            (&a + &b + &c + &d + &e + &f + &g + &h + &i).evald();

                        let tmp = j[(0, 0)];
                        black_box(j);
                    })
                },
            );
        }

        pub fn $name2(crit: &mut Criterion)
        {
            let range = rand::distributions::Uniform::<f64>::new(0.0, 10.0);

            let mut rng = rand::thread_rng();

            let a = NADMatrix::<f64>::from_distribution($dim, $dim, &range, &mut rng);

            let b = NADMatrix::<f64>::from_distribution($dim, $dim, &range, &mut rng);

            let c = NADMatrix::<f64>::from_distribution($dim, $dim, &range, &mut rng);

            let d = NADMatrix::<f64>::from_distribution($dim, $dim, &range, &mut rng);

            let e = NADMatrix::<f64>::from_distribution($dim, $dim, &range, &mut rng);

            let f = NADMatrix::<f64>::from_distribution($dim, $dim, &range, &mut rng);

            let g = NADMatrix::<f64>::from_distribution($dim, $dim, &range, &mut rng);

            let h = NADMatrix::<f64>::from_distribution($dim, $dim, &range, &mut rng);

            let i = NADMatrix::<f64>::from_distribution($dim, $dim, &range, &mut rng);

            crit.bench_function(format!("nalgebra_dmatrix{}", $dim).as_str(), |be| {
                be.iter(|| {
                    let j = (&a + &b + &c + &d + &e + &f + &g + &h + &i);
                    let tmp = j[(0, 0)];
                    black_box(tmp);
                    black_box(j);
                })
            });
        }

        pub fn $name3(crit: &mut Criterion)
        {
            let range = rand::distributions::Uniform::<f64>::new(0.0, 10.0);

            let mut rng = rand::thread_rng();

            let mut a = vec![0.0f64; $dim * $dim];

            let mut b = vec![0.0f64; $dim * $dim];

            let mut c = vec![0.0f64; $dim * $dim];

            let mut d = vec![0.0f64; $dim * $dim];

            let mut e = vec![0.0f64; $dim * $dim];

            let mut f = vec![0.0f64; $dim * $dim];

            let mut g = vec![0.0f64; $dim * $dim];

            let mut h = vec![0.0f64; $dim * $dim];

            let mut i = vec![0.0f64; $dim * $dim];

            for ii in 0..$dim * $dim
            {
                a[ii] = range.sample(&mut rng);

                b[ii] = range.sample(&mut rng);

                c[ii] = range.sample(&mut rng);

                d[ii] = range.sample(&mut rng);

                e[ii] = range.sample(&mut rng);

                f[ii] = range.sample(&mut rng);

                g[ii] = range.sample(&mut rng);

                h[ii] = range.sample(&mut rng);

                i[ii] = range.sample(&mut rng);
            }

            crit.bench_function(format!("vec_{}", $dim).as_str(), |be| {
                be.iter(|| {
                    let mut j = vec![0.0f64; $dim * $dim];

                    for ii in 0..($dim * $dim)
                    {
                        j[ii] =
                            a[ii] + b[ii] + c[ii] + d[ii] + e[ii] + f[ii] + g[ii] + h[ii] + i[ii];
                    }

                    black_box(j);
                })
            });
        }
    };
}

add_benches_dmatrix!(
    10,
    topohedral_linalg_dmatrix_10,
    nalgebra_dmatrix_10,
    vec_10
);

add_benches_dmatrix!(
    20,
    topohedral_linalg_dmatrix_20,
    nalgebra_dmatrix_20,
    vec_20
);

add_benches_dmatrix!(
    30,
    topohedral_linalg_dmatrix_30,
    nalgebra_dmatrix_30,
    vec_30
);

add_benches_dmatrix!(
    40,
    topohedral_linalg_dmatrix_40,
    nalgebra_dmatrix_40,
    vec_40
);

criterion_group!(
    benches_dmatrix,
    topohedral_linalg_dmatrix_10,
    nalgebra_dmatrix_10,
    vec_10,
    topohedral_linalg_dmatrix_20,
    nalgebra_dmatrix_20,
    vec_20,
    topohedral_linalg_dmatrix_30,
    nalgebra_dmatrix_30,
    vec_30,
    topohedral_linalg_dmatrix_40,
    nalgebra_dmatrix_40,
    vec_40
);
//}}}

criterion_main!(benches_dmatrix);
