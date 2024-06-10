use core::slice;
use std::{ops::Add, str::SplitInclusive};

fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        *r2=7;
        println!("num is {}", *r1);
    }
    println!("num is {}", num);

    let mut v = vec![1,2,3,4,5,6];
    let r = &mut v[..];
    let (a , b) = r.split_at_mut( 3);
    assert_eq!(a, &mut [1,2,3]);
    assert_eq!(b, &mut [4,5,6]);

}

/*
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]){
    let len =slice.len();
    assert!(mid <= len);
    (&mut slice[..mid], &mut slice[mid..])
}
*/

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]){
    let len =slice.len();
    let ptr =slice.as_mut_ptr();

    assert!(mid <= len);
    unsafe{
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len-mid)
        )
    }

}

struct Point{
    x: i32,
    y:i32
}

impl Add for Point{
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point{
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }

}
