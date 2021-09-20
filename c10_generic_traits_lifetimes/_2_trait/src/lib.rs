pub mod aggregator {
    pub trait Summary { // 特征的方法应该是默认 pub 了
        fn summarize_author(&self) -> String;
        fn summarize(&self) -> String {
            String::from("(Read more...)") // 默认实现
        }
    }
    
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    
    impl Summary for NewsArticle {
        fn summarize_author(&self) -> String {
            format!("@{}", self.author)
        }
        // 已经有默认实现了，也可以不覆盖
        // fn summarize(&self) -> String {
        //     format!("{}, by {} ({})", self.headline, self.author, self.location)
        // }
    }
    
    
    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
    
}

