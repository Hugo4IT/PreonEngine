#[cfg(not(feature = "std"))]
#[macro_export] macro_rules! abs {
    ($expr:expr) => {{
        let e = ($expr);
        if e < 0.0 { -e } else { e }
    }};
}

#[cfg(feature = "std")]
#[macro_export] macro_rules! abs {
    ($expr:expr) => {
        ($expr).abs()
    };
}