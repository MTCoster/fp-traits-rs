#![cfg(test)]

use crate::{CallFp, UnsafeFp, UnsafeExternFp};

unsafe fn nil_fn(_val: u32) {}

unsafe extern "C" fn test_fn(val: u32) -> u32 {
    val + 42
}

unsafe extern "C" fn test_fn_2(val_1: u32, val_2: u32) -> u32 {
    val_1 + val_2
}

unsafe fn unsafe_fn(val: u32) -> u32 {
    val - 42
}

#[test]
fn simple() {
    let fp: UnsafeExternFp<'_, (u32,), u32> = fp! { fn test_fn => unsafe extern "C" fn(u32) -> u32 };
    assert_eq!(unsafe { fp.call((2,)) }, 44);
}

#[test]
fn raw_pointer() {
    let ptr = test_fn as *const ();
    let fp = fp! { ptr => unsafe extern "C" fn(u32,) -> u32 };
    assert_eq!(unsafe { fp.call((2,)) }, 44);
}

#[test]
fn two() {
    let fp1 = fp! { fn test_fn => unsafe extern "C" fn(u32,) -> u32 };
    let fp2 = fp! { fn test_fn_2 => unsafe extern "C" fn(u32, u32,) -> u32 };
    assert_eq!(unsafe { fp1.call((2,)) }, 44);
    assert_eq!(unsafe { fp2.call((2, 3)) }, 5);
}

#[test]
fn nil() {
    let fp: fp_t! { unsafe fn(u32) };
    fp = fp! { fn nil_fn => unsafe fn(u32) };
    unsafe { fp.call((42,)) }
}

#[test]
fn struct_2() {
    unsafe {
        let fp: fp_t! { unsafe fn'static(u32) -> u32 } = UnsafeFp::<'_, _, _>::from(unsafe_fn as unsafe fn(u32) -> u32);
        assert_eq!(fp.call((44, )), 2);
    }
}

#[test]
fn no_macro() {
    unsafe {
        let fp: UnsafeExternFp<'static, (u32,), u32> = UnsafeExternFp::<'static, (u32,), u32>::from(test_fn as unsafe extern "C" fn(u32) -> u32);
        assert_eq!(fp.call((2, )), 44);
    }
}

// This test always fails to compile
// #[test]
// fn lifetime() {
//     struct FpHolder<'a> {
//         fp: Option<&'a mut dyn UnsafeExternFp<'a, (u32,), u32>>,
//     }
//
//     struct Loader;
//
//     unsafe fn load_fn<'l, L: 'l>(_loader: &'l L) -> fp_t! { unsafe extern "C" fn'l(u32,) -> u32 } {
//         fp! { test_fn => unsafe extern "C" fn'l(u32,) -> u32 }
//     }
//
//     let mut fp_holder = FpHolder {
//         fp: None,
//     };
//
//     {
//         let loader = Loader;
//         let fp = unsafe { load_fn(&loader) };
//         fp_holder.fp = Some(fp);
//     }
//
//     assert_eq!(unsafe { fp_holder.fp.unwrap().call((2,)) }, 44);
// }

// This test always fails to compile
#[test]
fn lifetime_2() {
    struct FpHolder<'a> {
        fp: Option<UnsafeFp<'a, (u32,), u32>>,
    }

    struct Loader;

    unsafe fn load_fn<'l, L: 'l>(_loader: &'l L) -> UnsafeFp<'l, (u32,), u32> {
        UnsafeFp::<'l, (u32,), u32>::from(unsafe_fn as unsafe fn(u32) -> u32 )
    }

    let mut fp_holder = FpHolder {
        fp: None,
    };

    {
        let loader = Loader;
        let fp = unsafe { load_fn(&loader) };
        fp_holder.fp = Some(fp);
    }

    assert_eq!(
        core::mem::size_of::<UnsafeFp<'static, (u32,), u32>>(),
        core::mem::size_of::<<UnsafeFp<'static, (u32,), u32> as CallFp<(u32,), u32>>::Raw>(),
    );

    // assert_eq!(unsafe { fp_holder.fp.unwrap().call((44,)) }, 2);
}
