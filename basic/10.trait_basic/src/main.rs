
mod lib;

// 1
pub trait Summary {
    // Must implement
    fn summarize(&self) -> String;
}

// 2
pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
// Tweet 에 의한 NewArticle
impl Summary for NewArticle {
    fn summarize(&self) -> String {
        // go to  println's  {}
        format!("{}, by {}, ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Tweet 에 의한 Summary
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


// both implement Summary and  Tweet
//  pub fn notify<T: Summary + Tweet>(item: T, item2: T) 

//  pub fn notify<T: Summary>(item: T, item2: T) 
//  arg is implement trait Summary
pub fn notify(item: impl Summary) -> String {
    println!("속보!: {}", item.summarize());
}



// use Where
// fn some_func<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 
fn some_func<T, U>(t: T, u: U) -> impl trait {
    where 
        T: Display + Clone,
        U: Clone + Debug
    // return 
    Tweet {
        username: String::from("RUST_EBOOK"),
        content: String::from("Start to Read"),
        reply: false,
        retweet: false,
    };
}


fn main() {
    println!("Hi Main");
    let tweet = Tweet {
        username: String::from("RUST_EBOOK"),
        content: String::from("Start to Read"),
        reply: false,
        retweet: false,
    };
    // return format!
    println!("New a Tweet: {} ", tweet.summarize());    
}
