use approx::assert_relative_eq;
use std::cmp::Ordering;
use std::num::FpCategory;
use topohedral_linalg::Float;

macro_rules! float_trait_smoke_test {
    ($name:ident, $type:ty, $epsilon:expr) => {
        #[test]
        fn $name()
        {
            let x = 1.25 as $type;
            let y = -0.75 as $type;
            let z = 0.5 as $type;
            let theta = std::f64::consts::FRAC_PI_6 as $type;
            let e = std::f64::consts::E as $type;

            assert_eq!(<$type as Float>::abs(y), 0.75 as $type);
            #[allow(deprecated)]
            {
                assert_eq!(<$type as Float>::abs_sub(x, z), 0.75 as $type);
            }
            assert_relative_eq!(
                <$type as Float>::acos(0.5 as $type),
                std::f64::consts::FRAC_PI_3 as $type,
                epsilon = $epsilon
            );
            assert_relative_eq!(
                <$type as Float>::acosh(1.5 as $type).cosh(),
                1.5 as $type,
                epsilon = $epsilon
            );
            assert_relative_eq!(
                <$type as Float>::asin(0.5 as $type),
                theta,
                epsilon = $epsilon
            );
            assert_relative_eq!(
                <$type as Float>::asinh(1.5 as $type).sinh(),
                1.5 as $type,
                epsilon = $epsilon
            );
            assert_relative_eq!(
                <$type as Float>::atan(1.0 as $type),
                std::f64::consts::FRAC_PI_4 as $type,
                epsilon = $epsilon
            );
            assert_relative_eq!(
                <$type as Float>::atan2(1.0 as $type, 1.0 as $type),
                std::f64::consts::FRAC_PI_4 as $type,
                epsilon = $epsilon
            );
            assert_relative_eq!(
                <$type as Float>::atanh(0.25 as $type).tanh(),
                0.25 as $type,
                epsilon = $epsilon
            );
            assert_eq!(<$type as Float>::cbrt(8.0 as $type), 2.0 as $type);
            assert_eq!(<$type as Float>::ceil(1.25 as $type), 2.0 as $type);
            assert_eq!(
                <$type as Float>::clamp(3.0 as $type, -1.0 as $type, 2.0 as $type),
                2.0 as $type
            );
            assert_eq!(
                <$type as Float>::clamp_magnitude(-5.0 as $type, 3.0 as $type),
                -3.0 as $type
            );
            assert_eq!(<$type as Float>::classify(x), FpCategory::Normal);
            assert_eq!(<$type as Float>::copysign(x, y), -x);
            assert_relative_eq!(
                <$type as Float>::cos(theta),
                0.8660254037844386 as $type,
                epsilon = 10.0 as $type * $epsilon
            );
            assert_relative_eq!(
                <$type as Float>::cosh(0.5 as $type),
                1.1276259652063807 as $type,
                epsilon = 10.0 as $type * $epsilon
            );
            assert_eq!(
                <$type as Float>::div_euclid(7.0 as $type, 4.0 as $type),
                1.0 as $type
            );
            assert_relative_eq!(
                <$type as Float>::erf(0.0 as $type),
                0.0 as $type,
                epsilon = $epsilon
            );
            assert_relative_eq!(
                <$type as Float>::erfc(0.0 as $type),
                1.0 as $type,
                epsilon = $epsilon
            );
            assert_relative_eq!(
                <$type as Float>::exp(1.0 as $type),
                e,
                epsilon = 10.0 as $type * $epsilon
            );
            assert_eq!(<$type as Float>::exp2(3.0 as $type), 8.0 as $type);
            assert_relative_eq!(
                <$type as Float>::exp_m1(1.0 as $type),
                e - 1.0 as $type,
                epsilon = 10.0 as $type * $epsilon
            );
            assert_eq!(<$type as Float>::floor(1.75 as $type), 1.0 as $type);
            assert_relative_eq!(
                <$type as Float>::fract(1.75 as $type),
                0.75 as $type,
                epsilon = $epsilon
            );

            let bits = <$type as Float>::to_bits(x);
            assert_eq!(<$type as Float>::from_bits(bits), x);

            let be_bytes = <$type as Float>::to_be_bytes(x);
            let le_bytes = <$type as Float>::to_le_bytes(x);
            let ne_bytes = <$type as Float>::to_ne_bytes(x);
            assert_eq!(<$type as Float>::from_be_bytes(be_bytes), x);
            assert_eq!(<$type as Float>::from_le_bytes(le_bytes), x);
            assert_eq!(<$type as Float>::from_ne_bytes(ne_bytes), x);

            assert_relative_eq!(
                <$type as Float>::gamma(5.0 as $type),
                24.0 as $type,
                epsilon = 100.0 as $type * $epsilon
            );
            assert_relative_eq!(
                <$type as Float>::hypot(3.0 as $type, 4.0 as $type),
                5.0 as $type,
                epsilon = $epsilon
            );
            assert!(<$type as Float>::is_finite(x));
            assert!(<$type as Float>::is_infinite(<$type>::INFINITY));
            assert!(<$type as Float>::is_nan(<$type>::NAN));
            assert!(<$type as Float>::is_normal(x));
            assert!(<$type as Float>::is_sign_negative(-1.0 as $type));
            assert!(<$type as Float>::is_sign_positive(1.0 as $type));
            assert!(<$type as Float>::is_subnormal(
                <$type>::MIN_POSITIVE / 2.0 as $type
            ));
            assert_relative_eq!(
                <$type as Float>::ln(e),
                1.0 as $type,
                epsilon = 10.0 as $type * $epsilon
            );
            assert_relative_eq!(
                <$type as Float>::ln_1p(0.5 as $type),
                (1.5 as $type).ln(),
                epsilon = 10.0 as $type * $epsilon
            );

            let (ln_gamma, sign) = <$type as Float>::ln_gamma(2.0 as $type);
            assert_relative_eq!(ln_gamma, 0.0 as $type, epsilon = 10.0 as $type * $epsilon);
            assert_eq!(sign, 1);

            assert_relative_eq!(
                <$type as Float>::log(8.0 as $type, 2.0 as $type),
                3.0 as $type,
                epsilon = 10.0 as $type * $epsilon
            );
            assert_eq!(<$type as Float>::log10(100.0 as $type), 2.0 as $type);
            assert_eq!(<$type as Float>::log2(8.0 as $type), 3.0 as $type);
            assert_eq!(<$type as Float>::max(x, y), x);
            assert_eq!(<$type as Float>::maximum(x, y), x);
            assert_eq!(<$type as Float>::midpoint(x, y), 0.25 as $type);
            assert_eq!(<$type as Float>::min(x, y), y);
            assert_eq!(<$type as Float>::minimum(x, y), y);
            assert_eq!(
                <$type as Float>::mul_add(2.0 as $type, 3.0 as $type, 4.0 as $type),
                10.0 as $type
            );
            assert!(<$type as Float>::next_down(x) < x);
            assert!(<$type as Float>::next_up(x) > x);
            assert_eq!(
                <$type as Float>::powf(4.0 as $type, 0.5 as $type),
                2.0 as $type
            );
            assert_eq!(<$type as Float>::powi(4.0 as $type, 2), 16.0 as $type);
            assert_relative_eq!(
                <$type as Float>::recip(4.0 as $type),
                0.25 as $type,
                epsilon = $epsilon
            );
            assert_eq!(
                <$type as Float>::rem_euclid(7.0 as $type, 4.0 as $type),
                3.0 as $type
            );
            assert_eq!(<$type as Float>::round(1.5 as $type), 2.0 as $type);
            assert_eq!(
                <$type as Float>::round_ties_even(2.5 as $type),
                2.0 as $type
            );
            assert_eq!(<$type as Float>::signum(y), -1.0 as $type);
            assert_relative_eq!(
                <$type as Float>::sin(theta),
                0.5 as $type,
                epsilon = $epsilon
            );

            let (sin_theta, cos_theta) = <$type as Float>::sin_cos(theta);
            assert_relative_eq!(sin_theta, 0.5 as $type, epsilon = $epsilon);
            assert_relative_eq!(cos_theta, <$type as Float>::cos(theta), epsilon = $epsilon);

            assert_relative_eq!(
                <$type as Float>::sinh(0.5 as $type),
                0.5210953054937474 as $type,
                epsilon = 10.0 as $type * $epsilon
            );
            assert_eq!(<$type as Float>::small(), <$type>::EPSILON);
            assert_eq!(<$type as Float>::sqrt(4.0 as $type), 2.0 as $type);
            assert_relative_eq!(
                <$type as Float>::tan(theta),
                0.5773502691896257 as $type,
                epsilon = 10.0 as $type * $epsilon
            );
            assert_relative_eq!(
                <$type as Float>::tanh(0.5 as $type),
                0.46211715726000974 as $type,
                epsilon = 10.0 as $type * $epsilon
            );
            assert_eq!(
                <$type as Float>::to_degrees(std::f64::consts::PI as $type),
                180.0 as $type
            );

            let int_value = unsafe { <$type as Float>::to_int_unchecked::<u16>(4.75 as $type) };
            assert_eq!(int_value, 4_u16);

            assert_relative_eq!(
                <$type as Float>::to_radians(180.0 as $type),
                std::f64::consts::PI as $type,
                epsilon = 10.0 as $type * $epsilon
            );
            assert_eq!(
                <$type as Float>::total_cmp(&(-0.0 as $type), &(0.0 as $type)),
                Ordering::Less
            );
            assert_eq!(<$type as Float>::trunc(1.75 as $type), 1.0 as $type);
            assert_eq!(
                <$type as Float>::algebraic_add(1.5 as $type, 2.0 as $type),
                3.5 as $type
            );
            assert_eq!(
                <$type as Float>::algebraic_sub(1.5 as $type, 2.0 as $type),
                -0.5 as $type
            );
            assert_eq!(
                <$type as Float>::algebraic_mul(1.5 as $type, 2.0 as $type),
                3.0 as $type
            );
            assert_eq!(
                <$type as Float>::algebraic_div(3.0 as $type, 2.0 as $type),
                1.5 as $type
            );
            assert_eq!(
                <$type as Float>::algebraic_rem(7.0 as $type, 4.0 as $type),
                3.0 as $type
            );
        }
    };
}

float_trait_smoke_test!(float_trait_smoke_test_f32, f32, 1.0e-5_f32);
float_trait_smoke_test!(float_trait_smoke_test_f64, f64, 1.0e-12_f64);
