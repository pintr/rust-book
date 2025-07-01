/// A collection that maintains a list of `i32` values and keeps track of their average.
///
/// The `AveragedCollection` struct provides a way to store a list of integers and
/// automatically update the average value whenever the collection is modified.
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    /// Creates a new, empty `AveragedCollection`.
    ///
    /// # Returns
    ///
    /// An instance of `AveragedCollection` with an empty list and an average of `0.0`.
    pub fn new() -> Self {
        AveragedCollection {
            list: Vec::new(),
            average: 0.0,
        }
    }
    /// Adds an integer to the collection and updates the average.
    ///
    /// # Arguments
    ///
    /// * `value` - The integer to add to the collection.
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    /// Removes the last integer from the collection and updates the average.
    ///
    /// # Returns
    ///
    /// * `Option<i32>` - The removed integer if the collection is not empty, or `None` if it is empty.
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    /// Returns the current average of the collection.
    ///
    /// # Returns
    ///
    /// * `f64` - The average value of the integers in the collection.
    pub fn average(&mut self) -> f64 {
        self.average
    }

    /// Recalculates and updates the average value based on the current contents of the collection.
    ///
    /// This method is called internally whenever the collection is modified.
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

pub mod gui {
    //! # Gui
    //!
    //! A library to draw components using trait objects for dynamic dispatch.
    //!
    //! This module provides the `Draw` trait for drawable UI components and the `Screen` struct to manage and render a collection of such components.

    /// A trait for drawable UI components.
    ///
    /// Types implementing this trait can be drawn onto a screen.
    pub trait Draw {
        /// Draws the component.
        fn draw(&self);
    }

    /// A container for drawable components.
    ///
    /// The `Screen` struct holds a list of components implementing the `Draw` trait, and can render all of them by calling their `draw` methods.
    pub struct Screen {
        /// The list of components to be drawn.
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        /// Runs the screen by drawing each component in order.
        ///
        /// Iterates over all components and calls their `draw` method.
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    /// A button component that can be drawn on the screen.
    ///
    /// The `Button` struct represents a UI button with a specified width, height, and label.
    /// It implements the `Draw` trait, allowing it to be rendered as part of a `Screen`.
    ///
    /// # Fields
    ///
    /// * `width` - The width of the button in pixels.
    /// * `height` - The height of the button in pixels.
    /// * `label` - The text label displayed on the button.
    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        /// Draws the button component.
        ///
        /// This method is called when rendering the button as part of a `Screen`.
        fn draw(&self) {
            // Draw the button
        }
    }
}

pub mod blog {
    //! Blog
    //!
    //! The `blog` module implements a simple state pattern for managing blog posts.
    //!
    //! It provides the `Post` struct, which encapsulates the content of a blog post and its publishing workflow.
    //! The post transitions through different states (draft, pending review, published) using internal state objects.
    //! State transitions and content visibility are controlled through the public API.

    /// Represents a blog post that has an internal state and associated content.
    ///
    /// The `Post` struct uses the state pattern to manage its publishing workflow.
    /// The current state is stored as a boxed trait object, allowing for dynamic
    /// state transitions. The `content` field holds the text of the post.
    ///
    /// # Fields
    /// - `state`: The current state of the post, implementing the `State` trait.
    /// - `content`: The textual content of the post.
    pub struct Post {
        state: Option<Box<dyn State>>,
        content: String,
    }

    impl Post {
        /// Creates a new `Post` in the draft state with empty content.
        pub fn new() -> Post {
            Post {
                state: Some(Box::new(Draft {})),
                content: String::new(),
            }
        }

        /// Appends the given text to the content of the post.
        ///
        /// # Arguments
        ///
        /// * `text` - A string slice that will be added to the post's content.
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        /// Returns the content of the post as a string slice.
        ///
        /// The content is only visible depending on the current state of the post.
        /// For example, in the draft state, this may return an empty string.
        pub fn content(&self) -> &str {
            self.state.as_ref().unwrap().content(self)
        }

        /// Requests a review of the post, transitioning it to the next state if possible.
        ///
        /// If the post is in the draft state, it will move to the pending review state.
        pub fn request_review(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review());
            }
        }

        /// Approves the post, transitioning it to the next state if possible.
        ///
        /// If the post is in the pending review state, it will move to the published state.
        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }
    }

    trait State {
        /// Requests a review of the current state, consuming the current state and returning a new state.
        ///
        /// # Returns
        ///
        /// A boxed trait object representing the next state after requesting a review.
        fn request_review(self: Box<Self>) -> Box<dyn State>;

        /// Approves the current state, consuming the current state and returning a new state.
        ///
        /// # Returns
        ///
        /// A boxed trait object representing the next state after approval.
        fn approve(self: Box<Self>) -> Box<dyn State>;

        /// Returns the content of the post if the state allows it, otherwise returns an empty string.
        ///
        /// # Arguments
        ///
        /// * `_post` - A reference to the `Post` whose content may be returned.
        ///
        /// # Returns
        ///
        /// A string slice containing the post content or an empty string if not allowed.
        fn content<'a>(&self, _post: &'a Post) -> &'a str {
            ""
        }
    }

    /// Represents the draft state of a blog post.
    ///
    /// In this state, the post is being written and edited. The content is not visible to readers.
    /// Transitions:
    /// - On `request_review`, moves to the `PendingReview` state.
    /// - On `approve`, remains in the `Draft` state.
    struct Draft {}

    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview {})
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }

    /// Represents the pending review state of a blog post.
    ///
    /// In this state, the post is awaiting approval before being published. The content is not visible to readers.
    /// Transitions:
    /// - On `approve`, moves to the `Published` state.
    /// - On `request_review`, remains in the `PendingReview` state.
    struct PendingReview {}

    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            Box::new(Published {})
        }
    }

    /// Represents the published state of a blog post.
    ///
    /// In this state, the post has been approved and is visible to readers. The content is accessible.
    /// Transitions:
    /// - On `request_review` or `approve`, remains in the `Published` state.
    struct Published {}

    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
    }
}

pub mod blog_no_state {
    //! Blog no state
    //!
    //! The `blog_no_state` module provides a simple blog post workflow without using the state pattern.
    //! It defines types representing the different stages of a blog post's lifecycle: draft, pending review, and published.

    /// Represents a published blog post.
    ///
    /// Use [`Post::new`] to start creating a new post as a draft.
    pub struct Post {
        content: String,
    }

    /// Represents a blog post in draft state.
    ///
    /// Use [`DraftPost::add_text`] to add content, and [`DraftPost::request_review`] to move to the pending review state.
    pub struct DraftPost {
        content: String,
    }

    impl Post {
        /// Creates a new draft post.
        ///
        /// # Returns
        ///
        /// A [`DraftPost`] instance that can be edited before review.
        pub fn new() -> DraftPost {
            DraftPost {
                content: String::new(),
            }
        }

        /// Returns the content of the published post.
        pub fn content(&self) -> &str {
            &self.content
        }
    }

    impl DraftPost {
        /// Appends text to the draft post's content.
        ///
        /// # Arguments
        ///
        /// * `text` - The text to add to the draft.
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        /// Requests a review for the draft post, moving it to the pending review state.
        ///
        /// # Returns
        ///
        /// A [`PendingReviewPost`] instance.
        pub fn request_review(self) -> PendingReviewPost {
            PendingReviewPost {
                content: self.content,
            }
        }
    }

    /// Represents a blog post that is pending review.
    ///
    /// Use [`PendingReviewPost::approve`] to publish the post.
    pub struct PendingReviewPost {
        content: String,
    }

    impl PendingReviewPost {
        /// Approves the post, publishing it.
        ///
        /// # Returns
        ///
        /// A [`Post`] instance representing the published post.
        pub fn approve(self) -> Post {
            Post {
                content: self.content,
            }
        }
    }
}
