//! Summary
//! There are multiple structs that hold various kind and amount of text
//! I.e. `NewsArticle`, `Tweet`, ...
//! The purpose is creating a media aggregator library that displays a summary of data contained in the structs
//! To do this, they need to implement a trait `Summary` that defines a `summarise` method on an instance

// This is the definition of the Summary public trait using the `trait` keyword
pub trait Summary {
    // Method signature that need to be implemented.
    // They end with a semicolon because each type needs to implement the methods
    // fn summarise(&self) -> String;
    // Traits allow to define a default implementation of a method, that can be overrided
    // Default implmentation can call other methods in the same trait, even if they don't have a default implementation
    fn summarise(&self) -> String {
        format!("(Read more from {}...)", self.summarise_author())
    }

    fn summarise_author(&self) -> String;
}

// The following are the definitions for the structs `NewsArticle` and `Tweet`
// They both implement the Summary trait
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl Summary for NewsArticle {
//     // Implementing a trait is similar to implementing regular methods
//     // The only difference is in the form: `impl` + trait + `for` + type
//     fn summarise(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }

//     fn summarise_author(&self) -> String {
//         format!("{}", self.author)
//     }
// }

// To use the default implementation for summarise this is the syntax:
impl Summary for NewsArticle {
    fn summarise_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarise(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarise_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// To use the default implementation for summarise this is the syntax:
// impl Summary for Tweet {
//     fn summarise_author(&self) -> String {
//          format!("@{}", self.username)
//      }
// }

// Traits can alse be used as parameters
// Instead of having a concrete type for `item`, the parameter is composed by `impl` and the trait
// Only the methods specified by by the trait are available in the body of the function.
// In this case only variables that implement `Summary` can be passed to `notify`
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarise());
// }
// A way to rewrite the notify funciton is the following:
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarise());
}
// It's equivalent but more verbose. It can be convenient with multiple parameters
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
// That becomes
// pub fn notify<T: Summary>(item1: &T, item2: &T) {
// It is possible to specify multiple trait bounds using `+`
// For example if we need parameters that implement more than one trait the following are the conventions:
// pub fn notify(item: &(impl Summary + Display)) {
// pub fn notify<T: Summary + Display>(item: &T) {
// Having multiple traits can contain lots of information, making the signature hard to read.
// For this reason Rust uses the `where` clause, making it easier to read:
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
// Becomes
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
// It's more verbose but easier to read.
