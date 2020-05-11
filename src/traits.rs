
#![allow(unused_variables)]

use std::fmt::format;

pub trait Summary {
    fn summarize(&self) -> String;
    fn content(&self) -> String {
        String::from("content")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn content(&self) ->  String {
        self.content.clone()
    }
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl NewsArticle {
    fn headline(&self) -> &String{
        &(&self.headline)
    }
}
fn main() {
    let a = NewsArticle{
        headline:String::from("aa"),
        location:String::from("bb"),
        author:String::from("cc"),
        content:String::from("dd"),
    };
    println!("{}", a.summarize());
    println!("{}", a.content());

}