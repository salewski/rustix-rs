// Ensure that the source and destination types are both primitive integer
// types and the same size, and then bitcast.
#[allow(unused_macros)]
macro_rules! bitcast {
    ($x:expr) => {{
        if false {
            // Ensure the source and destinations are primitive integer types.
            let _ = !$x;
            let _ = $x as u8;
            0
        } else {
            // SAFETY: We've ensured that both the source and destinations
            // are primitive integer types.
            #[allow(unsafe_code, unused_unsafe, clippy::useless_transmute)]
            unsafe {
                ::core::mem::transmute($x)
            }
        }
    }};
}

/// Return a [`bitcast`] of the value of `$x.bits()`, where `$x` is a
/// `bitflags` type.
#[allow(unused_macros)]
macro_rules! bitflags_bits {
    ($x:expr) => {{
        bitcast!($x.bits())
    }};
}
