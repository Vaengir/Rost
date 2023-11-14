// Adds the string "world" to `s`.
///
/// # Example
/// ```
/// let mut s = "Hello ".to_string();
/// worldly::add_world(&mut s);
/// assert_eq!(s, "Hello world");
/// ```

pub fn add_world(s: &mut String) {
  s.push_str("world");
}

#[test]

fn test_add_world() {
  let mut s = String::new();
  add_world(&mut s);
  assert_eq!(s, "world");
}
