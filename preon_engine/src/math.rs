const PRECISION: usize = 20;

#[cfg(not(feature = "std"))]
pub fn inv_sqrt(z: f32) -> f32 {
    union _SqrtVal { f: f32, i: u32 }

    unsafe {
        let half = z * 0.5;
        let mut val: _SqrtVal = _SqrtVal { f: z };

        val.i = 0x5F375A86 - (val.i >> 1);
        val.f = val.f * (1.5 - half * val.f * val.f);

        val.f
    }
}

#[cfg(not(feature = "std"))]
#[inline]
pub fn sqrt(z: f32) -> f32 {
    1.0 / inv_sqrt(z)
}

#[cfg(not(feature = "std"))]
#[macro_export] macro_rules! sqrt {
    ($expr:expr) => {
        $crate::math::sqrt(($expr))
    };
}

#[cfg(feature = "std")]
#[macro_export] macro_rules! sqrt {
    ($expr:expr) => {
        ($expr).sqrt()
    };
}

#[cfg(not(feature = "std"))]
#[macro_export] macro_rules! inv_sqrt {
    ($expr:expr) => {
        $crate::math::inv_sqrt(($expr))
    };
}

#[cfg(feature = "std")]
#[macro_export] macro_rules! inv_sqrt {
    ($expr:expr) => {
        (1.0 / ($expr).sqrt())
    };
}

#[cfg(not(feature = "std"))]
#[macro_export] macro_rules! powi {
    ($num:expr, $exp:expr) => {
        $crate::math::powi(($num), ($exp))
    };
}

#[cfg(feature = "std")]
#[macro_export] macro_rules! powi {
    ($num:expr, $exp:expr) => {
        ($num).powi($exp)
    };
}

#[cfg(not(feature = "std"))]
#[macro_export] macro_rules! powf {
    ($num:expr, $exp:expr) => {
        $crate::math::powf(($num), ($exp))
    };
}

#[cfg(feature = "std")]
#[macro_export] macro_rules! powf {
    ($num:expr, $exp:expr) => {
        ($num).powf($exp)
    };
}

#[cfg(not(feature = "std"))]
macro_rules! get_integer {
    ($n:expr) => {
        (($n) as i64 as f32)
    };
}

#[cfg(not(feature = "std"))]
macro_rules! get_decimal {
    ($n:expr) => {
        (($n) - ($n) as i64 as f32)
    };
}

#[cfg(not(feature = "std"))]
fn min(x: f32, y: f32) -> f32 {
    if x < y {
        x
    } else {
        y
    }
}

#[cfg(not(feature = "std"))]
fn max(x: f32, y: f32) -> f32 {
    if x > y {
        x
    } else {
        y
    }
}

#[cfg(not(feature = "std"))]
fn is_integer(n: f32) -> bool {
    max(n, get_integer!(n)) - min(n, get_integer!(n)) <= core::f32::EPSILON
}

#[cfg(not(feature = "std"))]
fn decimal_to_fraction(mut x: f32) -> (i32, i32) {
    let mut magnitude = 1;

    while !is_integer(x) {
        x *= 10.0;
        magnitude *= 10;
    }

    (x as i32, magnitude)
}

#[cfg(not(feature = "std"))]
pub fn powf(num: f32, exp: f32) -> f32 {
    if is_integer(exp) {
        powi(num, exp as i32)
    } else {
        let exp_integer = get_integer!(exp);
        let exp_decimal = get_decimal!(exp);
        let (numerator, denominator) = decimal_to_fraction(exp_decimal);
        powi(num, exp_integer as i32) * root(denominator, powi(num, numerator))
    }
}

#[cfg(not(feature = "std"))]
pub fn powi(num: f32, mut exp: i32) -> f32 {
    let mut total = 1.0;

    while exp > 0 {
        total *= num;
        exp -= 1;
    }

    total
}

#[cfg(not(feature = "std"))]
fn root(exp: i32, num: f32) -> f32 {
    let mut guess = max(1.0, num / exp as f32);

    for _ in 0..PRECISION {
        guess -= powi(guess, exp) - num / (exp as f32 * powi(guess, exp - 1));
    }

    guess
}