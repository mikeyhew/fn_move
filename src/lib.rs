#![no_std]
#![feature(fn_traits, unboxed_closures)]

use core::ptr;

/// A version of FnOnce that (unsafely) does not take `self` by value.
/// 
/// This allows you to implement FnOnce on a type owning an FnMove trait object, as long as
/// the wrapper type is Sized.
/// 
/// This trait will be obsolete once the [unsized rvalues RFC](https://github.com/rust-lang/rfcs/pull/1909)
/// is implemented.
#[rustc_paren_sugar]
pub trait FnMove<Args>: FnOnce<Args> {

    /// unsafe because this takes `self` by reference, but treats it as moved.
    /// The caller must somehow stop the closure's destructor from running afterwards,
    /// whether that's by calling mem::forget on it or something else.
    unsafe fn call_move(&mut self, args: Args) -> Self::Output;
}

impl<F, Args> FnMove<Args> for F where F: FnOnce<Args> {

    unsafe fn call_move(&mut self, args: Args) -> Self::Output {
        ptr::read(self).call_once(args)
    }
}
