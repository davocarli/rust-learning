use core::fmt::Debug;
use std::fmt::Display;

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn get_author(&self) -> String {
        format!("{}", self.author)
    }

    // fn summarize(&self) -> String {
    //     format!("{}, by {}", self.headline, self.author)
    // }
}

impl Summary for Tweet {
    fn get_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub trait Summary {
    fn get_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.get_author())
    }
}

// Returns something that implement Summary, but can be any type that implements it
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("username"),
        content: String::from("content"),
        reply: false,
        retweet: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y:T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Anything that implements Display will now implement ToString
impl<T: Display> ToString for T {
    //...
}

fn main() {
    let tweet = Tweet {
        username: String::from("@davocarli"),
        content: String::from("I tweeted!"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("David Carli-Arnold"),
        headline: String::from("Goodbye World!"),
        content: String::from("Global warming is like seriously a problem."),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_same_type<T: Summary>(item1: &T, item2: &T) {
    //...
}

pub fn notify_multiple_traits(item1: &(impl Summary + Clone), item2: &impl Summary) {
    //...
}

pub fn notify_same_type_multiple_traits<T: Summary + Clone>(item1: &T, item2: &T) {
    //...
}

pub fn many_traits<T: Summary + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    //...
    10
}

pub fn many_traits_other(t: &(impl Summary + Clone), u: &(impl Clone + Debug)) -> i32 {
    //...
    10
}

pub fn many_traits_where<T, U>(t: &T, u: &U) -> i32 
    where T: Summary + Clone,
          U: Clone + Debug,
{
    //...
    10
}
