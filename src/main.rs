use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.add_text(" and it was great");

    post.reject();
    post.approve();
    assert_eq!("", post.content());

    post.request_review();

    post.approve();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    post.add_text("!!!");
    assert_eq!("I ate a salad for lunch today", post.content());

    println!("All checks passed: {}", post.content());
}
