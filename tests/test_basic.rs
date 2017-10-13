extern crate fn_move;

use fn_move::FnMove;

use std::mem;

#[test]
fn test_it_works() {
    let v: Vec<i32> = vec![1,2,3];
    let mut v2: Vec<i32> = vec![];
    let v2_ptr = &mut v2 as *mut Vec<i32>;

    let mut moves_v = Box::new(move || {
        for x in v {
            // have to use a raw pointer, because the Rust borrow checker doesn't realize
            // that moves_v is no longer (safely) callable when mem::forget is called on it
            unsafe {(*v2_ptr).push(x + 3)};
        }
    }) as Box<FnMove()>;

    unsafe {
        moves_v.call_move(());
        mem::forget(moves_v);
    }

    assert_eq!(v2, &[4,5,6]);
}
