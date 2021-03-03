use ch17_blog::{Post, PostEnum, PostType};

fn main() {
    let content = String::from("I'm not a BTC maxi");

    // //////////////////////
    // With OOP pattern

    let mut post = Post::new();

    post.add_text(&content);
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!(content, post.content());

    // //////////////////////
    // With types OOP pattern

    let mut post_type = PostType::new();

    post_type.add_text(&content);
    let post_type = post_type.request_review();
    let post_type = post_type.approve();
    assert_eq!(content, post_type.content());

    // //////////////////////
    // With match pattern

    let mut postEnum = PostEnum::new();

    postEnum.add_text(&content);
    assert_eq!("", postEnum.content());

    postEnum.request_review();
    assert_eq!("", postEnum.content());

    postEnum.approve();
    assert_eq!(content, postEnum.content());
}
