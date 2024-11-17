#![feature(generic_const_exprs)]
#![feature(impl_trait_in_assoc_type)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use topohedral_linalg::{Evaluate, SMatrix};

use nalgebra::SMatrix as NASMatrix;

use rand::prelude::*;



macro_rules! add_benches {
    ($dim: expr, $name1: ident, $name2: ident, $name3: ident) => {
        
        pub fn $name1(crit: &mut Criterion) {

            let a = SMatrix::<f64, $dim, $dim>::from_uniform_random(0.0, 10.0);
            let b = SMatrix::<f64, $dim, $dim>::from_uniform_random(0.0, 10.0);
            let c = SMatrix::<f64, $dim, $dim>::from_uniform_random(0.0, 10.0);
            let d = SMatrix::<f64, $dim, $dim>::from_uniform_random(0.0, 10.0);
            let e = SMatrix::<f64, $dim, $dim>::from_uniform_random(0.0, 10.0);
            let f = SMatrix::<f64, $dim, $dim>::from_uniform_random(0.0, 10.0);
            let g = SMatrix::<f64, $dim, $dim>::from_uniform_random(0.0, 10.0);
            let h = SMatrix::<f64, $dim, $dim>::from_uniform_random(0.0, 10.0);
            let i = SMatrix::<f64, $dim, $dim>::from_uniform_random(0.0, 10.0);

            crit.bench_function(format!("topohedral-linalg_{}", $dim).as_str(), |be| be.iter(|| {
                let j: SMatrix<f64, $dim, $dim> = (&a + &b + &c + &d + &e + &f + &g + &h + &i).eval();
                black_box(j);
            }));
            
        }


        pub fn $name2(crit: &mut Criterion) {

            let range = rand::distributions::Uniform::<f64>::new(0.0, 10.0);
            let mut rng = rand::thread_rng();
            let a = NASMatrix::<f64, $dim, $dim>::from_distribution(&range, &mut rng);
            let b = NASMatrix::<f64, $dim, $dim>::from_distribution(&range, &mut rng);
            let c = NASMatrix::<f64, $dim, $dim>::from_distribution(&range, &mut rng);
            let d = NASMatrix::<f64, $dim, $dim>::from_distribution(&range, &mut rng);
            let e = NASMatrix::<f64, $dim, $dim>::from_distribution(&range, &mut rng);
            let f = NASMatrix::<f64, $dim, $dim>::from_distribution(&range, &mut rng);
            let g = NASMatrix::<f64, $dim, $dim>::from_distribution(&range, &mut rng);
            let h = NASMatrix::<f64, $dim, $dim>::from_distribution(&range, &mut rng);
            let i = NASMatrix::<f64, $dim, $dim>::from_distribution(&range, &mut rng);

            crit.bench_function(format!("nalgebra_{}", $dim).as_str(), |be| be.iter(|| {

                let j = (&a + &b + &c + &d + &e + &f + &g + &h + &i);
                black_box(j);
            }));
        }
        

        pub fn $name3(crit: &mut Criterion) {

            let range = rand::distributions::Uniform::<f64>::new(0.0, 10.0);
            let mut rng = rand::thread_rng();

            let mut a = [0.0f64; $dim * $dim];
            let mut b = [0.0f64; $dim * $dim];
            let mut c = [0.0f64; $dim * $dim];
            let mut d = [0.0f64; $dim * $dim];
            let mut e = [0.0f64; $dim * $dim];
            let mut f = [0.0f64; $dim * $dim];
            let mut g = [0.0f64; $dim * $dim];
            let mut h = [0.0f64; $dim * $dim];
            let mut i = [0.0f64; $dim * $dim];

            for ii in 0..$dim * $dim{
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

            crit.bench_function(format!("array_{}", $dim).as_str(), |be| be.iter(|| {

                let mut j = [0.0f64; $dim * $dim];

                for ii in 0..($dim * $dim){
                    j[ii] = a[ii] + b[ii] + c[ii] + d[ii] + e[ii] + f[ii] + g[ii] + h[ii] + i[ii];
                }

                black_box(j);
            }));

        }

    };
}


add_benches!(10, topohedral_linalg_10, nalgebra_10, array_10);
add_benches!(20, topohedral_linalg_20, nalgebra_20, array_20);
add_benches!(30, topohedral_linalg_30, nalgebra_30, array_30);
criterion_group!(benches, 
    topohedral_linalg_10, nalgebra_10, array_10,
    topohedral_linalg_20, nalgebra_20, array_20,
    topohedral_linalg_30, nalgebra_30, array_30
);
criterion_main!(benches);


// pub fn bench1(crit: &mut Criterion) {

//     let a = SMatrix::<f64, 10, 10>::from_uniform_random(0.0, 10.0);
//     let b = SMatrix::<f64, 10, 10>::from_uniform_random(0.0, 10.0);
//     let c = SMatrix::<f64, 10, 10>::from_uniform_random(0.0, 10.0);
//     let d = SMatrix::<f64, 10, 10>::from_uniform_random(0.0, 10.0);
//     let e = SMatrix::<f64, 10, 10>::from_uniform_random(0.0, 10.0);
//     let f = SMatrix::<f64, 10, 10>::from_uniform_random(0.0, 10.0);

//     crit.bench_function("topohedral-linalg", |be| be.iter(|| {

//         let g: SMatrix<f64, 10, 10> = (&f * (&a + &b) - (&c / &d) + &e).eval();
//         black_box(g);
//     }));
    
// }

// pub fn bench2(crit: &mut Criterion) {

//     let range = rand::distributions::Uniform::<f64>::new(0.0, 10.0);
//     let mut rng = rand::thread_rng();
//     let a = NASMatrix::<f64, 10, 10>::from_distribution(&range, &mut rng);
//     let b = NASMatrix::<f64, 10, 10>::from_distribution(&range, &mut rng);
//     let c = NASMatrix::<f64, 10, 10>::from_distribution(&range, &mut rng);
//     let d = NASMatrix::<f64, 10, 10>::from_distribution(&range, &mut rng);
//     let e = NASMatrix::<f64, 10, 10>::from_distribution(&range, &mut rng);
//     let f = NASMatrix::<f64, 10, 10>::from_distribution(&range, &mut rng);

//     crit.bench_function("nalgebra", |be| be.iter(|| {

//         let g: NASMatrix<f64, 10, 10> = f.component_mul(&(&a + &b)) - (c.component_div(&d)) + &e;
//         black_box(g);
//     }));
// }

// pub fn bench3(crit: &mut Criterion) {

//     let range = rand::distributions::Uniform::<f64>::new(0.0, 10.0);
//     let mut rng = rand::thread_rng();

//     let mut a = [0.0f64; 100];
//     let mut b = [0.0f64; 100];
//     let mut c = [0.0f64; 100];
//     let mut d = [0.0f64; 100];
//     let mut e = [0.0f64; 100];
//     let mut f = [0.0f64; 100];
//     let mut g = [0.0f64; 100];

//     for i in 0..100 {
//         a[i] = range.sample(&mut rng);
//         b[i] = range.sample(&mut rng);
//         c[i] = range.sample(&mut rng);
//         d[i] = range.sample(&mut rng);
//         e[i] = range.sample(&mut rng);
//         f[i] = range.sample(&mut rng);
//         g[i] = range.sample(&mut rng);
//     }

//     crit.bench_function("arrays", |be| be.iter(|| {

//         for i in 0..100 {
//             g[i] = f[i] * (a[i] + b[i]) - (c[i] / d[i]) + e[i];
//         }

//         black_box(g);
//     }));

// }

// criterion_group!(benches, bench1, bench2, bench3);
