use criterion::{criterion_group, criterion_main, Criterion};

use topohedral_linalg::{SMatrix, SubViewable};

use nalgebra::SMatrix as NASMatrix;

use rand::prelude::*;

//{{{ collection: SMatrix benches
macro_rules! add_benches_smatrix {
    ($dim: expr, $name1: ident, $name2: ident, $name3: ident) => {
        pub fn $name1(crit: &mut Criterion)
        {
            let a = SMatrix::<f64, $dim, $dim>::from_uniform_random(0.0, 10.0);

            let b = SMatrix::<f64, $dim, $dim>::from_uniform_random(0.0, 10.0);

            let c = SMatrix::<f64, $dim, $dim>::from_uniform_random(0.0, 10.0);

            let d = SMatrix::<f64, $dim, $dim>::from_uniform_random(0.0, 10.0);

            let e = SMatrix::<f64, $dim, $dim>::from_uniform_random(0.0, 10.0);

            let f = SMatrix::<f64, $dim, $dim>::from_uniform_random(0.0, 10.0);

            let g = SMatrix::<f64, $dim, $dim>::from_uniform_random(0.0, 10.0);

            let h = SMatrix::<f64, $dim, $dim>::from_uniform_random(0.0, 10.0);

            let i = SMatrix::<f64, $dim, $dim>::from_uniform_random(0.0, 10.0);

            crit.bench_function(
                format!("topohedral-linalg_smatrix{}", $dim).as_str(),
                |be| {
                    be.iter(|| {
                        let j: SMatrix<f64, $dim, $dim> =
                            (&a + &b + &c + &d + &e + &f + &g + &h + &i).into();

                        std::hint::black_box(j);
                    })
                },
            );
        }

        pub fn $name2(crit: &mut Criterion)
        {
            let range = rand::distr::Uniform::<f64>::new(0.0, 10.0).unwrap();

            let mut rng = rand::rng();

            let a = NASMatrix::<f64, $dim, $dim>::from_distribution(&range, &mut rng);

            let b = NASMatrix::<f64, $dim, $dim>::from_distribution(&range, &mut rng);

            let c = NASMatrix::<f64, $dim, $dim>::from_distribution(&range, &mut rng);

            let d = NASMatrix::<f64, $dim, $dim>::from_distribution(&range, &mut rng);

            let e = NASMatrix::<f64, $dim, $dim>::from_distribution(&range, &mut rng);

            let f = NASMatrix::<f64, $dim, $dim>::from_distribution(&range, &mut rng);

            let g = NASMatrix::<f64, $dim, $dim>::from_distribution(&range, &mut rng);

            let h = NASMatrix::<f64, $dim, $dim>::from_distribution(&range, &mut rng);

            let i = NASMatrix::<f64, $dim, $dim>::from_distribution(&range, &mut rng);

            crit.bench_function(format!("nalgebra_smatrix_smatrix{}", $dim).as_str(), |be| {
                be.iter(|| {
                    let j = (&a + &b + &c + &d + &e + &f + &g + &h + &i);

                    std::hint::black_box(j);
                })
            });
        }

        pub fn $name3(crit: &mut Criterion)
        {
            let range = rand::distr::Uniform::<f64>::new(0.0, 10.0).unwrap();

            let mut rng = rand::rng();

            let mut a = [0.0f64; $dim * $dim];

            let mut b = [0.0f64; $dim * $dim];

            let mut c = [0.0f64; $dim * $dim];

            let mut d = [0.0f64; $dim * $dim];

            let mut e = [0.0f64; $dim * $dim];

            let mut f = [0.0f64; $dim * $dim];

            let mut g = [0.0f64; $dim * $dim];

            let mut h = [0.0f64; $dim * $dim];

            let mut i = [0.0f64; $dim * $dim];

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

            crit.bench_function(format!("array_{}", $dim).as_str(), |be| {
                be.iter(|| {
                    let mut j = [0.0f64; $dim * $dim];

                    for ii in 0..($dim * $dim)
                    {
                        j[ii] =
                            a[ii] + b[ii] + c[ii] + d[ii] + e[ii] + f[ii] + g[ii] + h[ii] + i[ii];
                    }

                    std::hint::black_box(j);
                })
            });
        }
    };
}

add_benches_smatrix!(
    10,
    topohedral_linalg_smatrix_10,
    nalgebra_smatrix_10,
    array_10
);

add_benches_smatrix!(
    20,
    topohedral_linalg_smatrix_20,
    nalgebra_smatrix_20,
    array_20
);

add_benches_smatrix!(
    30,
    topohedral_linalg_smatrix_30,
    nalgebra_smatrix_30,
    array_30
);

add_benches_smatrix!(
    40,
    topohedral_linalg_smatrix_40,
    nalgebra_smatrix_40,
    array_40
);

macro_rules! add_view_benches_smatrix {
    ($inner: expr, $outer: expr, $name_cols: ident, $name_block: ident) => {
        pub fn $name_cols(crit: &mut Criterion)
        {
            let a = SMatrix::<f64, $inner, $outer>::from_uniform_random(0.0, 10.0);
            let b = SMatrix::<f64, $inner, $outer>::from_uniform_random(0.0, 10.0);
            let c = SMatrix::<f64, $inner, $outer>::from_uniform_random(0.0, 10.0);
            let d = SMatrix::<f64, $inner, $outer>::from_uniform_random(0.0, 10.0);
            let e = SMatrix::<f64, $inner, $outer>::from_uniform_random(0.0, 10.0);
            let f = SMatrix::<f64, $inner, $outer>::from_uniform_random(0.0, 10.0);
            let g = SMatrix::<f64, $inner, $outer>::from_uniform_random(0.0, 10.0);
            let h = SMatrix::<f64, $inner, $outer>::from_uniform_random(0.0, 10.0);
            let i = SMatrix::<f64, $inner, $outer>::from_uniform_random(0.0, 10.0);

            let av = a.cols_range(1, $inner);
            let bv = b.cols_range(1, $inner);
            let cv = c.cols_range(1, $inner);
            let dv = d.cols_range(1, $inner);
            let ev = e.cols_range(1, $inner);
            let fv = f.cols_range(1, $inner);
            let gv = g.cols_range(1, $inner);
            let hv = h.cols_range(1, $inner);
            let iv = i.cols_range(1, $inner);

            crit.bench_function(
                format!("topohedral-linalg_smatrix_view_cols{}", $inner).as_str(),
                |be| {
                    be.iter(|| {
                        let j: SMatrix<f64, $inner, $inner> =
                            (&av + &bv + &cv + &dv + &ev + &fv + &gv + &hv + &iv).into();

                        std::hint::black_box(j);
                    })
                },
            );
        }

        pub fn $name_block(crit: &mut Criterion)
        {
            let a = SMatrix::<f64, $outer, $outer>::from_uniform_random(0.0, 10.0);
            let b = SMatrix::<f64, $outer, $outer>::from_uniform_random(0.0, 10.0);
            let c = SMatrix::<f64, $outer, $outer>::from_uniform_random(0.0, 10.0);
            let d = SMatrix::<f64, $outer, $outer>::from_uniform_random(0.0, 10.0);
            let e = SMatrix::<f64, $outer, $outer>::from_uniform_random(0.0, 10.0);
            let f = SMatrix::<f64, $outer, $outer>::from_uniform_random(0.0, 10.0);
            let g = SMatrix::<f64, $outer, $outer>::from_uniform_random(0.0, 10.0);
            let h = SMatrix::<f64, $outer, $outer>::from_uniform_random(0.0, 10.0);
            let i = SMatrix::<f64, $outer, $outer>::from_uniform_random(0.0, 10.0);

            let av = a.subview_range(1, $inner, 1, $inner);
            let bv = b.subview_range(1, $inner, 1, $inner);
            let cv = c.subview_range(1, $inner, 1, $inner);
            let dv = d.subview_range(1, $inner, 1, $inner);
            let ev = e.subview_range(1, $inner, 1, $inner);
            let fv = f.subview_range(1, $inner, 1, $inner);
            let gv = g.subview_range(1, $inner, 1, $inner);
            let hv = h.subview_range(1, $inner, 1, $inner);
            let iv = i.subview_range(1, $inner, 1, $inner);

            crit.bench_function(
                format!("topohedral-linalg_smatrix_view_block{}", $inner).as_str(),
                |be| {
                    be.iter(|| {
                        let j: SMatrix<f64, $inner, $inner> =
                            (&av + &bv + &cv + &dv + &ev + &fv + &gv + &hv + &iv).into();

                        std::hint::black_box(j);
                    })
                },
            );
        }
    };
}

add_view_benches_smatrix!(
    10,
    12,
    topohedral_linalg_smatrix_view_cols_10,
    topohedral_linalg_smatrix_view_block_10
);

add_view_benches_smatrix!(
    40,
    42,
    topohedral_linalg_smatrix_view_cols_40,
    topohedral_linalg_smatrix_view_block_40
);

criterion_group!(
    benches_smatrix,
    topohedral_linalg_smatrix_10,
    nalgebra_smatrix_10,
    array_10,
    topohedral_linalg_smatrix_20,
    nalgebra_smatrix_20,
    array_20,
    topohedral_linalg_smatrix_30,
    nalgebra_smatrix_30,
    array_30,
    topohedral_linalg_smatrix_40,
    nalgebra_smatrix_40,
    array_40,
    topohedral_linalg_smatrix_view_cols_10,
    topohedral_linalg_smatrix_view_block_10,
    topohedral_linalg_smatrix_view_cols_40,
    topohedral_linalg_smatrix_view_block_40
);
//}}}

criterion_main!(benches_smatrix);
