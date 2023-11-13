#![no_std]

pub trait ConstDefault {
    const DEFAULT: Self;
}

macro_rules! default_impl {
    ([$($t:ty),*]; $val:expr) => {
        $(
            default_impl!($t, $val);
        )*
    };
    ($t:ty, $val:expr) => {
        impl ConstDefault for $t {
            const DEFAULT: Self = $val;
        }
    };
}

default_impl!([i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32, f64]; 0 as Self);
default_impl!([i128, u128]; 0);
default_impl!(bool, false);
default_impl!(&str, "");

impl<T> ConstDefault for Option<T> {
    const DEFAULT: Self = None;
}
impl<T: ConstDefault, const N: usize> ConstDefault for [T; N] {
    const DEFAULT: Self = [T::DEFAULT; N];
}
impl<T> ConstDefault for &[T] {
    const DEFAULT: Self = &[];
}
