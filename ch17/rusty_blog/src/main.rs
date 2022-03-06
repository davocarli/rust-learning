use::rusty_blog::{ Post, PostApproval };

fn main() {
    let mut post = Post::new();

    post.add_text("This is some great text!");

    let post = post.request_review();


    match post.approve() {
        PostApproval::PendingApproval(new_post) => {let post = new_post;},
        PostApproval::Approved(new_post) => {let post = new_post;},
    };
    
}
