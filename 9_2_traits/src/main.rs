use std::fmt::Display;

pub trait Summary {
    // fn summarize(&self) -> String;
    // implement default behaviour
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}
// Implementation of trait
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
// use default
impl Summary for Tweet { }

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
        hockey team in the NHL."),
    };

    println!("New article available! {}", article.summarize());
}
// traits could be returned
// this works cause we return only one type (Tweet) see. trait objects
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
// any item that implements Summary
fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// any type that imple Summary
fn notify2<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}
// any item1 that implements Summary, any item1 that implements Summary
pub fn notify3(item1: impl Summary, item2: impl Summary) {}
// T is any type that implements Summary, item1 and item2 share that type
pub fn notify4<T: Summary>(item1: T, item2: T) { }
// more verbose
fn notify5<T>(item: T)
    where T: Summary
{
    println!("Breaking news! {}", item.summarize());
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}
// will be available only for Pair with those traits
// several traits could be used with +
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
