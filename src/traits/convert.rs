use super::ops::Conv;


impl Conv<f64> for f32 {
    fn conv(self) -> f64 {
        self as f64
    }
}

impl Conv<f32> for f64 {
    fn conv(self) -> f32 {
        self as f32
    }
}


#[macro_export]
macro_rules! conv {
    ($($a:ident),*) => {
        $(let $a = $a.conv();)*
    };
}


impl<const N: usize, F: Conv<T>, T> Conv<[T; N]> for [F; N] {
    fn conv(self) -> [T; N] {
        self.map(F::conv)
    }
}