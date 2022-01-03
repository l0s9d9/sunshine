pub unsafe trait AnyFnPtr: Copy {
    fn address(self) -> usize;
}

macro_rules! impl_any_fn_ptr {
    ($($gen:ident),*) => {
        unsafe impl <$($gen),*, R> AnyFnPtr for fn($($gen),*) -> R {
            fn address(self) -> usize {
                self as _
            }
        }

        unsafe impl <$($gen),*, R> AnyFnPtr for unsafe fn($($gen),*) -> R {
            fn address(self) -> usize {
                self as _
            }
        }
    };
}

impl_any_fn_ptr!(T1);
impl_any_fn_ptr!(T1, T2);
impl_any_fn_ptr!(T1, T2, T3);
impl_any_fn_ptr!(T1, T2, T3, T4);
impl_any_fn_ptr!(T1, T2, T3, T4, T5);
impl_any_fn_ptr!(T1, T2, T3, T4, T5, T6);
impl_any_fn_ptr!(T1, T2, T3, T4, T5, T6, T7);
impl_any_fn_ptr!(T1, T2, T3, T4, T5, T6, T7, T8);
