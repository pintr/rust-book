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
    ///
    /// # Example
    ///
    /// ```
    /// let collection = AveragedCollection::new();
    /// assert_eq!(collection.average(), 0.0);
    /// ```
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
    ///
    /// # Example
    ///
    /// ```
    /// let mut collection = AveragedCollection::new();
    /// collection.add(10);
    /// ```
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    /// Removes the last integer from the collection and updates the average.
    ///
    /// # Returns
    ///
    /// * `Option<i32>` - The removed integer if the collection is not empty, or `None` if it is empty.
    ///
    /// # Example
    ///
    /// ```
    /// let mut collection = AveragedCollection::new();
    /// collection.add(10);
    /// let removed = collection.remove();
    /// assert_eq!(removed, Some(10));
    /// ```
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
    ///
    /// # Example
    ///
    /// ```
    /// let mut collection = AveragedCollection::new();
    /// collection.add(10);
    /// collection.add(20);
    /// assert_eq!(collection.average(), 15.0);
    /// ```
    pub fn average(&mut self) -> f64 {
        self.average
    }

    /// Recalculates and updates the average value based on the current contents of the collection.
    ///
    /// This method is called internally whenever the collection is modified.
    ///
    /// # Example
    ///
    /// ```
    /// let mut collection = AveragedCollection::new();
    /// collection.add(10);
    /// collection.add(20);
    /// // The average is automatically updated after each modification.
    /// ```
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
    ///
    /// # Example
    ///
    /// ```
    /// use your_crate::gui::{Button, Draw};
    ///
    /// let button = Button {
    ///     width: 100,
    ///     height: 50,
    ///     label: String::from("Click me"),
    /// };
    /// button.draw();
    /// ```
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
