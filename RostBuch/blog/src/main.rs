use blog::Post;

fn main() {
  let mut post = Post::new();

  post.add_text("I ate a salad for lunch today");
  assert_eq!("", post.content());

  post.request_review();
  assert_eq!("", post.content());

  // First draft gets rejected
  post.reject();

  // Draft has to be reviewed again
  post.request_review();

  // Approve needs to be called twice before State is changed to Published
  post.approve();
  post.approve();
  assert_eq!("I ate a salad for lunch today", post.content());
}
