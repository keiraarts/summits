use blog::Post;

fn main() {
    println!("Hello, world!");

    let mut post = Post::new();
    post.add_text("I wrote some Rust code today!");

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I wrote some Rust code today!", post.content());
}
