use std::ops::Deref;
use std::rc::Rc;

use crate::BoxList::{BCons, BNil};
use crate::RcList::{RCons, RNil};

fn main() {
  boxes();
  reference();
  box_reference();
  mybox_reference();
  mybox_hello();
  custom_smart_pointer();
  manual_drop();
  reference_counting();
}

#[derive(Debug)]
enum BoxList {
  BCons(i32, Box<BoxList>),
  BNil,
}

fn boxes() {
  let list = BCons(1, Box::new(BCons(2, Box::new(BCons(3, Box::new(BNil))))));
  println!("{list:?}");
}

fn reference() {
  let x = 5;
  let y = &x;

  assert_eq!(5, x);
  assert_eq!(5, *y);
}

fn box_reference() {
  let x = 5;
  let y = Box::new(x);

  assert_eq!(5, x);
  assert_eq!(5, *y);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}

impl<T> Deref for MyBox<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

fn mybox_reference() {
  let x = 5;
  let y = MyBox::new(x);

  assert_eq!(5, x);
  assert_eq!(5, *y);
}

fn hello(name: &str) {
  println!("Hello, {name}");
}

fn mybox_hello() {
  let m = MyBox::new(String::from("Rust"));
  hello(&m);
}

struct CustomSmartPointer {
  data: String,
}

impl Drop for CustomSmartPointer {
  fn drop(&mut self) {
    println!("Dropping CustomSmartPointer with data `{}`!", self.data);
  }
}

fn custom_smart_pointer() {
  let _c = CustomSmartPointer {
    data: String::from("my stuff"),
  };
  let _d = CustomSmartPointer {
    data: String::from("other stuff"),
  };
  println!("CustomSmartPointer created.");
}

fn manual_drop() {
  let c = CustomSmartPointer {
    data: String::from("some data"),
  };
  println!("CustomSmartPointer created.");
  drop(c);
  println!("CustomSmartPointer dropped before the end of manual_drop.");
}

enum RcList {
  RCons(i32, Rc<RcList>),
  RNil,
}

fn reference_counting() {
  let a = Rc::new(RCons(5, Rc::new(RCons(10, Rc::new(RNil)))));
  println!("count after creating a = {}", Rc::strong_count(&a));
  let _b = RCons(3, Rc::clone(&a));
  println!("count after creating b = {}", Rc::strong_count(&a));
  {
    let _c = RCons(4, Rc::clone(&a));
    println!("count after creating c = {}", Rc::strong_count(&a));
  }
  println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
