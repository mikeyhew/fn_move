#![feature(fn_traits, unboxed_closures, unsize, coerce_unsized)]

extern crate fn_move;

use fn_move::FnMove;
use std::mem;
use std::ops::CoerceUnsized;
use std::marker::Unsize;

struct WrapBox<T: ?Sized>(Box<T>);

impl<F: FnMove<Args> + ?Sized, Args> FnOnce<Args> for WrapBox<F> {
    type Output = F::Output;

    extern "rust-call" fn call_once(mut self, args: Args) -> F::Output {
        unsafe {
            let ret = self.0.call_move(args);
            mem::forget(self);
            ret
        }
    }
}

impl<S: Unsize<T>, T: ?Sized> CoerceUnsized<WrapBox<T>> for WrapBox<S> {}

#[test]
fn it_works() {
    let v1 = vec![1i32,2,3];
    let v2 = vec![-1i32, -2, -3];

    let closure = WrapBox(Box::new(move ||{
        v1.into_iter()
        .zip(v2)
        // have to return a Vec because fixed-size arrays still
        // don't implement IntoIterator
        .flat_map(|(x, y)| { vec![x, y] })
        .collect::<Vec<_>>()
    // the coercion to a trait object here is the key part of this test!
    })) as WrapBox<FnMove() -> Vec<i32>>;

    assert_eq!(closure(), &[1, -1, 2, -2, 3, -3]);
}
