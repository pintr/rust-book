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
