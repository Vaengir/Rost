fn main() {
  kilometers();
  run_twice();
}

fn kilometers() {
  type Kilometers = i32;

  let x: i32 = 5;
  let y: Kilometers = 5;

  println!("x + y = {}", x + y);
}

fn add_once(x: i32) -> i32 {
  x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
  f(arg) + f(arg)
}

fn run_twice() {
  let answer = do_twice(add_once, 5);

  println!("The answer is: {}", answer)
}
