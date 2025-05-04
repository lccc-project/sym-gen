#![feature(macro_metavar_expr)]

pub mod map;
pub mod pool;
pub mod symbol;

#[doc(hidden)]
pub mod helpers;

#[doc(hidden)]
pub use std as _std_export;

pub use symbol::Symbol;

#[macro_export]
macro_rules! def_pool {
    {
        $vis:vis struct $pool_name:ident $(($init_hasher_expr:expr))? {
            $(type Hasher = $hasher_ty:ty;)?
            $(const $sym_name:ident $(= $sym_token:tt)?;)*
        }
    } => {
        $vis struct $pool_name;

        impl $pool_name {
            $($vis const $sym_name: $crate::symbol::Symbol<Self> = $crate::symbol::Symbol::new_unchecked(unsafe { $crate::_std_export::num::NonZeroU32::new_unchecked(${index()} + 1) });)*
        }

        const _: () = {
            type __HasherTy = <($crate::_std_export::hash::RandomState, $($hasher_ty)?) as $crate::helpers::TyOrDefault>::Type;
            impl $crate::pool::Pool for $pool_name {
                type Hasher = __HasherTy;
                fn get_map() -> &'static $crate::map::InternMap<Self::Hasher> {

                    const DYN_INIT: $crate::_std_export::num::NonZeroU32 = unsafe { $crate::_std_export::num::NonZeroU32::new_unchecked(${count($sym_name)} + 1)};

                    static MAP: $crate::_std_export::sync::OnceLock<
                    $crate::map::InternMap<__HasherTy>
                    > = $crate::_std_export::sync::OnceLock::new();

                    let basic_init_fn = (
                        $((|| $crate::map::InternMap::new_with_hashers(DYN_INIT, $init_hasher_expr, $init_hasher_expr)),)?
                        (|| $crate::map::InternMap::new(DYN_INIT)),
                    ).0;

                    MAP.get_or_init(move || {
                        #[allow(unused_mut)]
                        let mut map = basic_init_fn();

                        $(map.insert_mut(unsafe { $crate::_std_export::num::NonZeroU32::new_unchecked(${index()} + 1) }, $crate::def_pool!(@ $($sym_token,)? sym_name));)*

                        map
                    })
                }
            }
        };

        $vis type Symbol = $crate::symbol::Symbol<$pool_name>;
    };

    {@ $token:tt , $_ignored:tt} => {
        $crate::_std_export::stringify!($token)
    };

    {@ -$literal:literal} => {
        $crate::_std_export::concat!("-", $crate::_std_export::stringify!($literal))
    };

    {@ $literal:literal} => {
        $literal
    };

    {@ $token:tt} => {
        $crate::_std_export::stringify!($token)
    };


}

#[cfg(test)]
mod test {
    def_pool! {
        pub struct Pool {
            const FOO = foo;
        }
    }

    #[test]
    fn pool_test() {
        assert!(Pool::FOO == Symbol::intern("foo"))
    }
}
