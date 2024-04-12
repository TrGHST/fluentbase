#[macro_export]
macro_rules! size_of {
    ($typ:ty) => {
        core::mem::size_of::<$typ>()
    };
}

#[macro_export]
macro_rules! size_aligned {
    ($alignment:expr, $size:expr) => {
        (($size + $alignment - 1) / $alignment * $alignment)
    };
}

#[macro_export]
macro_rules! size_aligned_padding {
    ($alignment:expr, $size:expr) => {{
        let padding = $crate::size_aligned!($alignment, $size) - $size;
        padding
    }};
}
