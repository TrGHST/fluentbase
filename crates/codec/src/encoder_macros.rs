#[macro_export]
macro_rules! is_align_default {
    ($alignment:expr) => {
        $alignment == $crate::encoder::ALIGN_DEFAULT
    };
}

#[macro_export]
macro_rules! if_align_default_then {
    ($alignment:expr, $true_block:block, $false_block:block) => {
        if $crate::is_align_default!($alignment) $true_block else $false_block
    };
}

#[macro_export]
macro_rules! header_item_size {
    ($alignment:expr) => {
        $crate::if_align_default_then!($alignment, { $crate::encoder::HEADER_ITEM_SIZE_DEFAULT }, {
            $alignment
        })
    };
    ($alignment:expr, $typ:ty) => {
        $crate::if_align_default_then!($alignment, { $crate::size_of!($typ) }, { $alignment })
    };
}

#[macro_export]
macro_rules! header_size {
    ($alignment:expr, $items_count:expr) => {
        $crate::header_item_size!($alignment) * $items_count
    };
    ($alignment:expr, $typ:ty, $items_count:expr) => {
        $crate::header_item_size!($alignment, $typ) * $items_count
    };
}

#[macro_export]
macro_rules! effective_offset {
    ($offset:ident, $alignment:expr, $typ:ty) => {
        if $crate::is_align_default!($alignment) {
            $offset
        } else {
            $offset + $crate::header_item_size!(A) - $crate::size_of!($typ)
        }
    };
}

#[macro_export]
macro_rules! fixed_size_aligned {
    ($alignment:expr, $size:expr) => {
        if $crate::is_align_default!($alignment) {
            $size
        } else {
            assert!($size < $alignment);
            $alignment
        }
    };
}

#[macro_export]
macro_rules! fixed_type_size_aligned {
    ($alignment:expr, $typ:ty) => {
        $crate::fixed_size_aligned!($alignment, $crate::size_of!($typ))
    };
}

#[macro_export]
macro_rules! fixed_size_aligned_padding {
    ($alignment:expr, $size:expr) => {{
        let padding = $crate::fixed_size_aligned!($alignment, $size) - $size;
        assert!(padding >= 0);
        padding
    }};
}

#[macro_export]
macro_rules! fixed_type_size_aligned_padding {
    ($alignment:expr, $typ:ty) => {{
        $crate::fixed_size_aligned_padding!($alignment, $crate::size_of!($typ))
    }};
}

#[macro_export]
macro_rules! dynamic_size_aligned {
    ($alignment:expr, $size:expr) => {
        if $crate::is_align_default!($alignment) {
            $size
        } else {
            $crate::size_aligned!($alignment, $size)
        }
    };
    ($alignment:expr, $len:expr, $item_ty:ty) => {
        if $crate::is_align_default!($alignment) {
            $len
        } else {
            $crate::size_aligned!($alignment, $len, $item_ty)
        }
    };
}

#[macro_export]
macro_rules! dynamic_size_aligned_padding {
    ($alignment:expr, $size:expr) => {{
        let padding = $crate::dynamic_size_aligned!($alignment, $size) - $size;
        assert!(padding >= 0);
        padding
    }};
}
