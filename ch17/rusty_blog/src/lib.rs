pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
    approvals: u8,
}

impl Post {
    // Attempting to create a new Post will return a DraftPost
    // This ensures that the only way to proceed is to create a
    // Draft first
    pub fn new() -> DraftPost {
        DraftPost { content: String::new(), }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    fn completed_post(content: &str) -> Post {
        Post { content: String::from(content) }
    }
}

impl DraftPost {
    // Instead of creating a "content()" method that returns
    // an empty string, we've simply not implemented the content method
    // at all. This means that if someone tries to call the method, this
    // will provide a compile-time error. This can help avoid improper
    // use of methods.
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost { content: self.content, approvals: 0}
    }
}

pub enum PostApproval<'a> {
    Approved(Post),
    PendingApproval(&'a PendingReviewPost),
}

impl PendingReviewPost {
    pub fn approve(&mut self) -> PostApproval {
        if self.approvals < 1 {
            PostApproval::Approved(Post::completed_post(self.content.as_str()))
        } else {
            self.approvals += 1;
            PostApproval::PendingApproval(self)
        }
    }
}
