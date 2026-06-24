use criterion::{criterion_group, criterion_main, Criterion};
use nalgebra::DMatrix as NADMatrix;
use rand::prelude::*;
use topohedral_linalg::{DMatrix, SubViewable};

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
            let a = DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim, $dim);

            let b = DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim, $dim);

            let c = DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim, $dim);

            let d = DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim, $dim);

            let e = DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim, $dim);

            let f = DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim, $dim);

            let g = DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim, $dim);

            let h = DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim, $dim);

            let i = DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim, $dim);

            crit.bench_function(
                format!("topohedral-linalg_dmatrix{}", $dim).as_str(),
                |be| {
                    be.iter(|| {
                        let j: DMatrix<f64> = (&a + &b + &c + &d + &e + &f + &g + &h + &i).into();
                        std::hint::black_box(j);
                    })
                },
            );
        }

        pub fn $name2(crit: &mut Criterion)
        {
            let range = rand::distr::Uniform::<f64>::new(0.0, 10.0).unwrap();

            let mut rng = rand::rng();

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
                    std::hint::black_box(tmp);
                    std::hint::black_box(j);
                })
            });
        }

        pub fn $name3(crit: &mut Criterion)
        {
            let range = rand::distr::Uniform::<f64>::new(0.0, 10.0).unwrap();

            let mut rng = rand::rng();

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

                    std::hint::black_box(j);
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

macro_rules! add_view_benches_dmatrix {
    ($dim: expr, $name_cols: ident, $name_block: ident) => {
        pub fn $name_cols(crit: &mut Criterion)
        {
            let a = DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim, $dim + 2);
            let b = DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim, $dim + 2);
            let c = DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim, $dim + 2);
            let d = DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim, $dim + 2);
            let e = DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim, $dim + 2);
            let f = DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim, $dim + 2);
            let g = DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim, $dim + 2);
            let h = DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim, $dim + 2);
            let i = DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim, $dim + 2);

            let av = a.cols_range(1, $dim);
            let bv = b.cols_range(1, $dim);
            let cv = c.cols_range(1, $dim);
            let dv = d.cols_range(1, $dim);
            let ev = e.cols_range(1, $dim);
            let fv = f.cols_range(1, $dim);
            let gv = g.cols_range(1, $dim);
            let hv = h.cols_range(1, $dim);
            let iv = i.cols_range(1, $dim);

            crit.bench_function(
                format!("topohedral-linalg_dmatrix_view_cols{}", $dim).as_str(),
                |be| {
                    be.iter(|| {
                        let j: DMatrix<f64> =
                            (&av + &bv + &cv + &dv + &ev + &fv + &gv + &hv + &iv).into();
                        std::hint::black_box(j);
                    })
                },
            );
        }

        pub fn $name_block(crit: &mut Criterion)
        {
            let a = DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim + 2, $dim + 2);
            let b = DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim + 2, $dim + 2);
            let c = DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim + 2, $dim + 2);
            let d = DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim + 2, $dim + 2);
            let e = DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim + 2, $dim + 2);
            let f = DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim + 2, $dim + 2);
            let g = DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim + 2, $dim + 2);
            let h = DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim + 2, $dim + 2);
            let i = DMatrix::<f64>::from_uniform_random(0.0, 10.0, $dim + 2, $dim + 2);

            let av = a.subview_range(1, $dim, 1, $dim);
            let bv = b.subview_range(1, $dim, 1, $dim);
            let cv = c.subview_range(1, $dim, 1, $dim);
            let dv = d.subview_range(1, $dim, 1, $dim);
            let ev = e.subview_range(1, $dim, 1, $dim);
            let fv = f.subview_range(1, $dim, 1, $dim);
            let gv = g.subview_range(1, $dim, 1, $dim);
            let hv = h.subview_range(1, $dim, 1, $dim);
            let iv = i.subview_range(1, $dim, 1, $dim);

            crit.bench_function(
                format!("topohedral-linalg_dmatrix_view_block{}", $dim).as_str(),
                |be| {
                    be.iter(|| {
                        let j: DMatrix<f64> =
                            (&av + &bv + &cv + &dv + &ev + &fv + &gv + &hv + &iv).into();
                        std::hint::black_box(j);
                    })
                },
            );
        }
    };
}

add_view_benches_dmatrix!(
    10,
    topohedral_linalg_dmatrix_view_cols_10,
    topohedral_linalg_dmatrix_view_block_10
);

add_view_benches_dmatrix!(
    40,
    topohedral_linalg_dmatrix_view_cols_40,
    topohedral_linalg_dmatrix_view_block_40
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
    vec_40,
    topohedral_linalg_dmatrix_view_cols_10,
    topohedral_linalg_dmatrix_view_block_10,
    topohedral_linalg_dmatrix_view_cols_40,
    topohedral_linalg_dmatrix_view_block_40
);
//}}}

criterion_main!(benches_dmatrix);
