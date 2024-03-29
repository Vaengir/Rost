use core::fmt;
use std::ops::Add;

pub trait Iterator {
  type Item;

  fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
  count: u32,
}

impl Counter {
  fn new() -> Counter {
    Counter { count: 0 }
  }
}

impl Iterator for Counter {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    if self.count < 5 {
      self.count += 1;
      Some(self.count)
    } else {
      None
    }
  }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
  x: i32,
  y: i32,
}

impl Add for Point {
  type Output = Point;

  fn add(self, rhs: Self) -> Self::Output {
    Point {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    }
  }
}

impl fmt::Display for Point {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

impl OutlinePrint for Point {}

#[derive(Debug, Clone, Copy)]
struct Millimeters(u32);
#[derive(Debug, Clone, Copy)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
  type Output = Millimeters;

  fn add(self, rhs: Meters) -> Self::Output {
    Millimeters(self.0 + (rhs.0 * 1000))
  }
}

trait Pilot {
  fn fly(&self);
}

trait Wizard {
  fn fly(&self);
}

struct Human;

impl Pilot for Human {
  fn fly(&self) {
    println!("This is your captain speaking.");
  }
}

impl Wizard for Human {
  fn fly(&self) {
    println!("Up.");
  }
}

impl Human {
  fn fly(&self) {
    println!("*waving arms furiously*");
  }
}

trait Animal {
  fn baby_name() -> String;
}

struct Dog;

impl Dog {
  fn baby_name() -> String {
    String::from("Spot")
  }
}

impl Animal for Dog {
  fn baby_name() -> String {
    String::from("puppy")
  }
}

trait OutlinePrint: fmt::Display {
  fn outline_print(&self) {
    let output = self.to_string();
    let len = output.len();
    println!("{}", "*".repeat(len + 4));
    println!("*{}*", " ".repeat(len + 2));
    println!("* {} *", output);
    println!("*{}*", " ".repeat(len + 2));
    println!("{}", "*".repeat(len + 4));
  }
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "[{}]", self.0.join(", "))
  }
}

fn main() {
  assert_eq!(
    Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
    Point { x: 3, y: 3 }
  );

  let x: Millimeters = Millimeters(150);
  let y: Meters = Meters(2);

  let z = x.add(y);
  dbg!(z);

  let person = Human;
  Pilot::fly(&person);
  Wizard::fly(&person);
  person.fly();

  println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

  let u = Point { x: 2, y: -3 };

  println!("{}", u);
  u.outline_print();

  let w = Wrapper(vec![String::from("hello"), String::from("world")]);
  println!("w = {}", w);
}
