#![no_std]

#[cfg(test)]
mod tests;

use core::marker::PhantomData;
use core::mem;


pub trait FpBase: From<*const ()> + Sized {
    fn is_unsafe(&self) -> bool;
}

pub unsafe trait CallFp<'f, Args, Ret>: FpBase {
    type Raw;

    fn as_fp(&self) -> Self::Raw;

    unsafe fn call(&self, args: Args) -> Ret;

    #[inline]
    fn into_fp(self) -> Self::Raw {
        self.as_fp()
    }
}

#[macro_export]
macro_rules! fp_t {
    { $(extern "Rust")? fn($($args:ty),*$(,)?) } => {
        Fp<'_, ($($args,)*), ()>
    };
    { $(extern "Rust")? fn($($args:ty),*$(,)?) -> $ret:ty } => {
        Fp<'_, ($($args,)*), $ret>
    };
    { $(extern "Rust")? fn$lifetime:lifetime($($args:ty),*$(,)?) } => {
        Fp<$lifetime, ($($args,)*), ()>
    };
    { $(extern "Rust")? fn$lifetime:lifetime($($args:ty),*$(,)?) -> $ret:ty } => {
        Fp<$lifetime, ($($args,)*), $ret>
    };
    { extern $("C")? fn($($args:ty),*$(,)?) } => {
        ExternFp<'_, ($($args,)*), ()>
    };
    { extern $("C")? fn($($args:ty),*$(,)?) -> $ret:ty } => {
        ExternFp<'_, ($($args,)*), $ret>
    };
    { extern $("C")? fn$lifetime:lifetime($($args:ty),*$(,)?) } => {
        ExternFp<$lifetime, ($($args,)*), ()>
    };
    { extern $("C")? fn$lifetime:lifetime($($args:ty),*$(,)?) -> $ret:ty } => {
        ExternFp<$lifetime, ($($args,)*), $ret>
    };
    { unsafe $(extern "Rust")? fn($($args:ty),*$(,)?) } => {
        UnsafeFp<'_, ($($args,)*), ()>
    };
    { unsafe $(extern "Rust")? fn($($args:ty),*$(,)?) -> $ret:ty } => {
        UnsafeFp<'_, ($($args,)*), $ret>
    };
    { unsafe $(extern "Rust")? fn$lifetime:lifetime($($args:ty),*$(,)?) } => {
        UnsafeFp<$lifetime, ($($args,)*), ()>
    };
    { unsafe $(extern "Rust")? fn$lifetime:lifetime($($args:ty),*$(,)?) -> $ret:ty } => {
        UnsafeFp<$lifetime, ($($args,)*), $ret>
    };
    { unsafe extern $("C")? fn($($args:ty),*$(,)?) } => {
        UnsafeExternFp<'_, ($($args,)*), ()>
    };
    { unsafe extern $("C")? fn($($args:ty),*$(,)?) -> $ret:ty } => {
        UnsafeExternFp<'_, ($($args,)*), $ret>
    };
    { unsafe extern $("C")? fn$lifetime:lifetime($($args:ty),*$(,)?) } => {
        UnsafeExternFp<$lifetime, ($($args,)*), ()>
    };
    { unsafe extern $("C")? fn$lifetime:lifetime($($args:ty),*$(,)?) -> $ret:ty } => {
        UnsafeExternFp<$lifetime, ($($args,)*), $ret>
    };
}

#[macro_export]
macro_rules! fp {
    { $fp:expr => $(extern "Rust")? fn($($args:ty),*$(,)?) } => {
        Fp::<'_, ($($args,)*), ()>::from($fp)
    };
    { $fp:expr => $(extern "Rust")? fn($($args:ty),*$(,)?) -> $ret:ty } => {
        Fp::<'_, ($($args,)*), $ret>::from($fp)
    };
    { $fp:expr => $(extern "Rust")? fn$lifetime:lifetime($($args:ty),*$(,)?) } => {
        Fp::<$lifetime, ($($args,)*), ()>::from($fp)
    };
    { $fp:expr => $(extern "Rust")? fn$lifetime:lifetime($($args:ty),*$(,)?) -> $ret:ty } => {
        Fp::<$lifetime, ($($args,)*), $ret>::from($fp)
    };
    { $fp:expr => extern $("C")? fn($($args:ty),*$(,)?) } => {
        ExternFp::<'_, ($($args,)*), ()>::from($fp)
    };
    { $fp:expr => extern $("C")? fn($($args:ty),*$(,)?) -> $ret:ty } => {
        ExternFp::<'_, ($($args,)*), $ret>::from($fp)
    };
    { $fp:expr => extern $("C")? fn$lifetime:lifetime($($args:ty),*$(,)?) } => {
        ExternFp::<$lifetime, ($($args,)*), ()>::from($fp)
    };
    { $fp:expr => extern $("C")? fn$lifetime:lifetime($($args:ty),*$(,)?) -> $ret:ty } => {
        ExternFp::<$lifetime, ($($args,)*), $ret>::from($fp)
    };
    { $fp:expr => unsafe $(extern "Rust")? fn($($args:ty),*$(,)?) } => {
        UnsafeFp::<'_, ($($args,)*), ()>::from($fp)
    };
    { $fp:expr => unsafe $(extern "Rust")? fn($($args:ty),*$(,)?) -> $ret:ty } => {
        UnsafeFp::<'_, ($($args,)*), $ret>::from($fp)
    };
    { $fp:expr => unsafe $(extern "Rust")? fn$lifetime:lifetime($($args:ty),*$(,)?) } => {
        UnsafeFp::<$lifetime, ($($args,)*), ()>::from($fp)
    };
    { $fp:expr => unsafe $(extern "Rust")? fn$lifetime:lifetime($($args:ty),*$(,)?) -> $ret:ty } => {
        UnsafeFp::<$lifetime, ($($args,)*), $ret>::from($fp)
    };
    { $fp:expr => unsafe extern $("C")? fn($($args:ty),*$(,)?) } => {
        UnsafeExternFp::<'_, ($($args,)*), ()>::from($fp)
    };
    { $fp:expr => unsafe extern $("C")? fn($($args:ty),*$(,)?) -> $ret:ty } => {
        UnsafeExternFp::<'_, ($($args,)*), $ret>::from($fp)
    };
    { $fp:expr => unsafe extern $("C")? fn$lifetime:lifetime($($args:ty),*$(,)?) } => {
        UnsafeExternFp::<$lifetime, ($($args,)*), ()>::from($fp)
    };
    { $fp:expr => unsafe extern $("C")? fn$lifetime:lifetime($($args:ty),*$(,)?) -> $ret:ty } => {
        UnsafeExternFp::<$lifetime, ($($args,)*), $ret>::from($fp)
    };
    { fn $fp:expr => $(extern "Rust")? fn($($args:ty),*$(,)?) } => {
        <Fp::<'_, ($($args,)*), ()> as From::<fn($($args),*) -> ()>>::from($fp)
    };
    { fn $fp:expr => $(extern "Rust")? fn($($args:ty),*$(,)?) -> $ret:ty } => {
        <Fp::<'_, ($($args,)*), $ret> as From::<fn($($args),*) -> $ret>>::from($fp)
    };
    { fn $fp:expr => $(extern "Rust")? fn$lifetime:lifetime($($args:ty),*$(,)?) } => {
        <Fp::<$lifetime, ($($args,)*), ()> as From::<fn($($args),*) -> ()>>::from($fp)
    };
    { fn $fp:expr => $(extern "Rust")? fn$lifetime:lifetime($($args:ty),*$(,)?) -> $ret:ty } => {
        <Fp::<$lifetime, ($($args,)*), $ret> as From::<fn($($args),*) -> $ret>>::from($fp)
    };
    { fn $fp:expr => extern $("C")? fn($($args:ty),*$(,)?) } => {
        <ExternFp::<'_, ($($args,)*), ()> as From::<extern "C" fn($($args),*) -> ()>>::from($fp)
    };
    { fn $fp:expr => extern $("C")? fn($($args:ty),*$(,)?) -> $ret:ty } => {
        <ExternFp::<'_, ($($args,)*), $ret> as From::<extern "C" fn($($args),*) -> $ret>>::from($fp)
    };
    { fn $fp:expr => extern $("C")? fn$lifetime:lifetime($($args:ty),*$(,)?) } => {
        <ExternFp::<$lifetime, ($($args,)*), ()> as From::<extern "C" fn($($args),*) -> ()>>::from($fp)
    };
    { fn $fp:expr => extern $("C")? fn$lifetime:lifetime($($args:ty),*$(,)?) -> $ret:ty } => {
        <ExternFp::<$lifetime, ($($args,)*), $ret> as From::<extern "C" fn($($args),*) -> $ret>>::from($fp)
    };
    { fn $fp:expr => unsafe $(extern "Rust")? fn($($args:ty),*$(,)?) } => {
        <UnsafeFp::<'_, ($($args,)*), ()> as From::<unsafe fn($($args),*) -> ()>>::from($fp)
    };
    { fn $fp:expr => unsafe $(extern "Rust")? fn($($args:ty),*$(,)?) -> $ret:ty } => {
        <UnsafeFp::<'_, ($($args,)*), $ret> as From::<unsafe fn($($args),*) -> $ret>>::from($fp)
    };
    { fn $fp:expr => unsafe $(extern "Rust")? fn$lifetime:lifetime($($args:ty),*$(,)?) } => {
        <UnsafeFp::<$lifetime, ($($args,)*), ()> as From::<unsafe fn($($args),*) -> ()>>::from($fp)
    };
    { fn $fp:expr => unsafe $(extern "Rust")? fn$lifetime:lifetime($($args:ty),*$(,)?) -> $ret:ty } => {
        <UnsafeFp::<$lifetime, ($($args,)*), $ret> as From::<unsafe fn($($args),*) -> $ret>>::from($fp)
    };
    { fn $fp:expr => unsafe extern $("C")? fn($($args:ty),*$(,)?) } => {
        <UnsafeExternFp::<'_, ($($args,)*), ()> as From::<unsafe extern "C" fn($($args),*) -> ()>>::from($fp)
    };
    { fn $fp:expr => unsafe extern $("C")? fn($($args:ty),*$(,)?) -> $ret:ty } => {
        <UnsafeExternFp::<'_, ($($args,)*), $ret> as From::<unsafe extern "C" fn($($args),*) -> $ret>>::from($fp)
    };
    { fn $fp:expr => unsafe extern $("C")? fn$lifetime:lifetime($($args:ty),*$(,)?) } => {
        <UnsafeExternFp::<$lifetime, ($($args,)*), ()> as From::<unsafe extern "C" fn($($args),*) -> ()>>::from($fp)
    };
    { fn $fp:expr => unsafe extern $("C")? fn$lifetime:lifetime($($args:ty),*$(,)?) -> $ret:ty } => {
        <UnsafeExternFp::<$lifetime, ($($args,)*), $ret> as From::<unsafe extern "C" fn($($args),*) -> $ret>>::from($fp)
    };
}

macro_rules! is_qualifier {
    ($tgt:tt in) => (false);
    (unsafe in unsafe $($rest:tt)*) => (true);
    ($tgt:tt in $first:tt $($rest:tt)*) => (is_qualifier!($tgt in $($rest)*));
}

macro_rules! impl_fn_trait {
    { $wrap:tt [ $($qual:tt)* ] } => {
        impl_fn_traits!($vis type $wrap [ $($qual)* ] : )
    };
    { $wrap:tt [ $($qual:tt)* ] : $($arg:ident),*$(,)? } => {
        impl <'f, $($arg,)* Ret> From<$($qual)* fn($($arg,)*) -> Ret> for $wrap<'f, ($($arg,)*), Ret> {
            fn from(fp: $($qual)* fn($($arg,)*) -> Ret) -> Self {
                let ptr = fp as *const ();
                Self::from(ptr)
            }
        }

        unsafe impl <'f, $($arg,)* Ret> CallFp<'f, ($($arg,)*), Ret> for $wrap<'f, ($($arg,)*), Ret> {
            type Raw = $($qual)* fn($($arg,)*) -> Ret;

            #[inline]
            fn as_fp(&self) -> Self::Raw {
                // SAFETY: Provided `Self::Raw` was implemented correctly, this will always be a
                // safe conversion.
                unsafe { mem::transmute(self.fp) }
            }

            #[inline]
            unsafe fn call(&self, args: ($($arg,)*)) -> Ret {
                #[allow(non_snake_case)]
                let ($($arg,)*) = args;
                self.as_fp()($($arg,)*)
            }
        }
    };
}

macro_rules! impl_fn_traits {
    { @single $wrap:tt [ $($qual:tt)* ] => $arg_count:literal: ($($args:ident),*$(,)?)$(,)? } => {
        impl_fn_trait!($wrap [ $($qual)* ] : $($args,)*);
    };
    { @single $wrap:tt [ $($qual:tt)* ] => $arg_count:literal: ($($args:ident),*$(,)?), $($tail:tt)* } => {
        impl_fn_traits! { @single $wrap [ $($qual)* ] => $arg_count: ($($args,)*) }
        impl_fn_traits! { @single $wrap [ $($qual)* ] => $($tail)* }
    };
    {
        $wrap:tt [ $($qual:tt)* ] {
            $($body:tt)*
        }
    } => {
        impl_fn_traits! { @single $wrap [ $($qual)* ] => $($body)* }
    };
}

macro_rules! impl_fn_traits_all {
    {
        $(
            $(#[$meta:meta])*
            $vis:vis type $wrap:tt [ $($qual:tt)* ];
        )*
    } => {
        $(
            $(#[$meta])*
            #[repr(transparent)]
            $vis struct $wrap<'f, Args, Ret> {
                fp: *const (),
                #[doc(hidden)]
                _lifetime_phantom: PhantomData<&'f ()>,
                #[doc(hidden)]
                _args_phantom: PhantomData<Args>,
                #[doc(hidden)]
                _ret_phantom: PhantomData<Ret>,
            }

            $(#[$meta])*
            impl <'f, Args, Ret> From<*const ()> for $wrap<'f, Args, Ret> {
                fn from(fp: *const ()) -> Self {
                    Self {
                        fp,
                        _lifetime_phantom: PhantomData,
                        _args_phantom: PhantomData,
                        _ret_phantom: PhantomData,
                    }
                }
            }

            $(#[$meta])*
            impl <'f, Args, Ret> FpBase for $wrap<'f, Args, Ret> {
                #[inline]
                fn is_unsafe(&self) -> bool {
                    return is_qualifier!(unsafe in $($qual)*);
                }
            }

            $(#[$meta])*
            impl_fn_traits! {
                $wrap [ $($qual)* ] {
                    0: (),
                    1: (A0),
                    2: (A0, A1),
                    3: (A0, A1, A2),
                    4: (A0, A1, A2, A3),
                    5: (A0, A1, A2, A3, A4),
                    6: (A0, A1, A2, A3, A4, A5),
                    7: (A0, A1, A2, A3, A4, A5, A6),
                    8: (A0, A1, A2, A3, A4, A5, A6, A7),
                    9: (A0, A1, A2, A3, A4, A5, A6, A7, A8),
                    10: (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9),
                    11: (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10),
                    12: (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11),
                }
            }
        )*
    };
}

impl_fn_traits_all! {
    pub type Fp [ extern "Rust" ];
    pub type ExternFp [ extern "C" ];
    pub type UnsafeFp [ unsafe extern "Rust" ];
    pub type UnsafeExternFp [ unsafe extern "C" ];
}
