//! # Messenger
//!
//! Library containing the Messenger trait and the LimitTracker

/// Trait defining the send method for sending messages regarding the quota
pub trait Messenger {
    fn send(&self, msg: &str);
}

/// Struct for tracking the quota of the messages
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}
impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    /// Constructor
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    /// Set value of the tracker and send message if quota over 75%
    /// This method doesn't return anything, so can't be used to make assertions
    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percenteage_of_max = self.value as f64 / self.max as f64;

        if percenteage_of_max >= 1.0 {
            self.messenger.send("Error: quota exceeded!");
        } else if percenteage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: quota over 90%");
        } else if percenteage_of_max >= 0.75 {
            self.messenger.send("Warning: quota over 75%");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    /// Mock object used to keep track of the sent messages in order to make assertion for the `set_value` method of the `LimitTracker`
    struct MockMessenger {
        // sent_messages: Vec<String>, // Change for internal mutability
        sent_messages: RefCell<Vec<String>>,
    }

    ///Constructor that initialises a vector for keeping track of the sent messages
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                // sent_messages: vec![], // Change for internal mutability
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    /// Implementation of the Messenger trait
    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            // self.sent_messages.push(String::from(msg)); // Not working
            // The code above doesn't work because the send method takes a n immutable reference to self
            // To fix it `&mut self` could be used in both `trait` and `impl`, but the trait shouldn't be modified just for testing.
            // In this case interior mutability can help: the sent messages can be stored in a `RefCell<T>`, so the send method will be able to modify `sent_messages`
            // The methods becomes as follows:
            self.sent_messages.borrow_mut().push(String::from(msg));
            // In this case self is still immutable, mathcing to the trait, but the `borrow_mut` method on `self.sent_messages` allows to get a mutable reference of the `RefCell<Vec<String>>` value.

            // With `RefCell<T>` the `borrow` method returns a `Ref<T>`, while `borrow_mut()` `RefMut<T>`, and both implement `Deref` so they can be used as regular references
            // `RefCell<T>` keeps track of how many `Ref<T>` and `RefMut<T>` are active, and every `borrow` increasees the count of immutable borrows, it dereases when the reference goes out of scope.
            // `RefCell<T>`, lets use many immutable borrows, or one mutable at any point in time. If this rule is violated, `RefCell<T>` will panic at runtime:
            // Example:
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();

            one_borrow.push(String::from(msg));
            two_borrow.push(String::from(msg));

            // Having two `borrow_mut` makes the program panic with the error: `already borrowed: BorrowMutError`
            // Choosing to catch borrowing errors at runtime means potentially finding mistakes in the code later in the development, and incur in a small runtime performance penality because of keeping track of the borrows
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // assert_eq!(mock_messenger.sent_messages.len(), 1); // Change for internal mutability
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
