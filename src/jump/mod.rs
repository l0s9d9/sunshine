use crate::Ptr;

macro_rules! include_mods {
    ($($mod:ident),*) => {
        $(
            mod $mod;
            pub use $mod::*;
        )*
    };
}

include_mods!(absolute, relative);

pub trait Jump {
    const SIZE: usize;

    unsafe fn write(
        head: Ptr,
        dst: Ptr
    ) -> crate::Result<()>;
}
