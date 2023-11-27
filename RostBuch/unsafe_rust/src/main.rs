use std::slice;

fn main() {
  pointer_dereference();
  split_at_mut_test();
  other_language();
  static_vars();
  unsafe {
    dangerous();
  }
}

fn pointer_dereference() {
  let mut num = 5;

  let r1 = &num as *const i32;
  let r2 = &mut num as *mut i32;

  unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
  }
}

unsafe fn dangerous() {
  println!("Test");
}

fn split_at_mut_test() {
  let mut v = vec![1, 2, 3, 4, 5, 6];
  let mut w = vec![1, 2, 3, 4, 5, 6];

  let r = &mut v[..];
  let e = &mut w[..];

  let (a, b) = r.split_at_mut(3);
  let (c, d) = own_split_at_mut(e, 3);

  dbg!(a, b);
  dbg!(c, d);
  // assert_eq!(a, &mut [1, 2, 3]);
  // assert_eq!(b, &mut [4, 5, 6]);
}

fn own_split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
  let len = values.len();
  let ptr = values.as_mut_ptr();

  assert!(mid <= len);

  unsafe {
    (
      slice::from_raw_parts_mut(ptr, mid),
      slice::from_raw_parts_mut(ptr.add(mid), len - mid),
    )
  }
}

extern "C" {
fn abs(input: i32) -> i32;
}

fn other_language() {
  unsafe {
    println!("Absolute value of -3 according to C: {}", abs(-3));
  }
}

fn static_vars() {
  static HELLO_WORLD: &str = "Hello world";

  println!("name is: {}", HELLO_WORLD);

  static mut COUNTER: u32 = 0;

  unsafe {
    COUNTER += 5;
    println!("COUNTER: {}", COUNTER);
  }
}
