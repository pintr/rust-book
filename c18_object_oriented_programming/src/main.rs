//! Object-oriented programming (OOP) is a way of modelling programs
//! Object-oriented programs are made up of objects. An object packages both data and the procedures that operate on that data.
//! The procedures are typically called methods or operations
//! Rust has some characteristics commonly considered object oriented: strcts and enums have data, and `impl` blocks provide methods.

fn main() {
    encapsulation_inheritance();
    traits_for_inheritance();
    state_pattern();
}

fn encapsulation_inheritance() {
    {
        // One aspect of OOP is encapsulation, where the implementation details of an object aren't accessible to code using the object
        // To interaction with the object is done using its public API.
        // Code using the object shouldn't be able to reach the object internals and change data or behaviour directly.
        // This enable the programmer to change and refactor an object's internals without having to change the code of the object
        // In Rust encapsulation is done using the `pub` keyword to decide what modules, types, functions, and methods to expose publicly, the rest is private.
        // An example is the `AveragedCollection` in lib.rs that has a list of integers and a value representing the average of that list.
        // The struct is marked `pub` so other code can use it, but the field within it remain private.
        // This is important to keep updated the average when elmeents in the list are added or removed using the `add` and `remove` functions, the `average` function gets the average.
        // The new funciton is the constructor and creates an empty `AveragedCollection`
        // The public methods `add`, `remove`, and `average` are the only ways to access or modify data in an instance of `AveragedCollection`.
        // When an item is added or removed from the list, each function calls the private `update_average` that handles the updating of the `average` as well.
        // The `list` and `average` fields are private so there is no way to update the items from the fields directly, otherwise `average` would go out of sync.
        // The `average` method return the `average` field value.
        use c18_object_oriented_programming::AveragedCollection;

        let mut collection = AveragedCollection::new();

        println!("Add 10 to the collection");
        collection.add(10);
        println!("Add 20 to the collection");
        collection.add(20);
        println!("Add 60 to the collection");
        collection.add(60);

        println!("The average is {}", collection.average());

        let value = collection.remove().unwrap();

        println!(
            "Element {value} removed, now the the average is {}",
            collection.average()
        );
        // Since the implementation details of `AveragedCollection` are encapsulated, aspects of it can be changed in the future.
        // For example using an `HashSet<i32>` instead of a `Vec<i32>` for the `list` field.
        // As long as the signature of the public methods remains the same, code using it doesn't need to change.
        // If `list` was public and the type changed, since `HasSet<i32>` and `Vec<i32>` have different methods for adding and removing items, the external code would need to change
    }
    {
        // Inheritance is a mechanism where an object can inherit elements from another object's definition
        // The object would gain parent object's data and behaviour without having to define it again
        // Rust can do it by using a macro
        // The main reasons to use inheritance are:
        // - Reuse of code: the bahaviour is implemented for one type, inheritance enables to reuse that iomplementation for another type
        // - Polymorphism: enable a child type to be used the same places as the parent type, which means that multiple objects can be substituted by others at runtime if they share certain characteristics.
        // inheritance may share more code than necessary: subclasses don't need to share all the characteristics of the parent, but they do.
        // This leads to make a program design less flexible, and introduces the possibility of calling method on subclasses that don't make sense, or produce an error.
        // Rust takes the different approache of using trait objects instead of inheritance
    }
}

fn traits_for_inheritance() {
    // In chapter 8 an enum was defined to store different value types in a vector.
    // However it may be useful to let user extend the set of types that are valid
    // An example is a GUI tool, called `gui`, that lets the user draw GUI tools, some tools may be `Button` or `TextField` but other can be added too, such as `Image` or `SelectBox`
    // The `gui` library needs to keep track of the values of different types it needs to call the `draw` method on each of these typed values.
    // It's not important what the `draw` function does, just that the value has the method available.
    // In a language with inheritance this could be done by defining a class `Component` that has a method `draw` on it.
    // The other classes, such as `Button`, would inherit from `Component` and thus inherit the `draw` method too, that could be overriden to define a custom behaviour.
    // Since Rust doesn't have inheritance traits are used to allow users to extend the `gui` library.
    // To implement this behaviour a trait called `Draw` with a method `draw` is needed, then it is possible to define a vactor that takes a trait object.
    // A trait object points to both an instance of a type implementing a trait, and a table to llok up trait methods on that type at runtime.
    // A trait object can be created by specifying some sort of pointer, such as `&` or `Box<T>`, then the `dyn` keyword, and specifying the trait.
    // A trait object can be used in place of a generic or concrete type and Rust will ensure at compile time that any value used in the context implements that trait, without knowing all the possible types.
    // Trait objects, differently from structs or enum that require `impl to specify a behaviour, combine data and behaviour.
    // The purpose of a trait object is to allow abstraction across common behaviour
    // So, the `gui` module needs a trait `Draw` with a method `draw`, and a struct `Screen` that holds a vector called components of type `Box<dyn Draw>`
    // `Box<dyn Draw>` is a trait object and allows to insert any element that implement the `Draw` trait
    // On the `Screen` struct it's defined a method `run` that calls the `draw` method on each of the `components`
    // This is different from defining a struct that uses a generic type parameter with trait bounds, because it can only be substituted with one concrete type at a time, not multiple

    {
        // An example of this implementation would be the following:
        use c18_object_oriented_programming::gui::Draw;
        pub struct _Screen<T: Draw> {
            pub components: Vec<T>,
        }

        impl<T> _Screen<T>
        where
            T: Draw,
        {
            pub fn _run(&self) {
                for component in self.components.iter() {
                    component.draw();
                }
            }
        }
        // This restricts the `Screen` instance to use a list of components all of the same type (such as `Button`, or `TextField`)
        // Anyway, with an homogeneous collection, using generics and trait bounds is preferable because the definitions are monomorphised at compile time.
        // On the other hand with the method using trait objects can hold a `Vec<T>` that contains `Box<Button>` as well as `Box<TextField>`
    }
    // In the `gui` library a button has been created implementing the `Draw`trait
    // The fields on Button (`width`, `height`, and `label`) may differ from other components, such as `TextField` which would add a `placeholder`
    // Each of the types will implement the `Draw` trait, but the method `draw` is different for each of them, they could even have additional `impl` blocks containing methods realted to other events (e.g. click of button)
    // Here is the implementation of a `SelectBox` using `Draw`:

    use c18_object_oriented_programming::gui::{Button, Draw, Screen};

    {
        #[allow(dead_code)]
        struct SelectBox {
            width: u32,
            height: u32,
            options: Vec<String>,
        }

        impl Draw for SelectBox {
            fn draw(&self) {
                // Draw the select box
            }
        }
        // Here is the `Screen` instance used for adding the components and draw the using the `run` function, which will call the `draw` method of each component:
        let screen = Screen {
            components: vec![
                Box::new(SelectBox {
                    width: 75,
                    height: 10,
                    options: vec![
                        String::from("Yes"),
                        String::from("Maybe"),
                        String::from("No"),
                    ],
                }),
                Box::new(Button {
                    width: 50,
                    height: 10,
                    label: String::from("OK"),
                }),
            ],
        };

        screen.run();
    }
    // When the `gui` library was written the added components aren't known, such as `SelectBox`, but the `Screen` implementation allows it since it works with the `Draw` trait.
    // Similarly, when `screen.run()` is called it doesn't need to know what the concrete type of each component is, it just calls the `draw` method,which is present as specified by the `Box<dyn Draw>` type.
    // For example adding another element that doesn't implement it, results in an error:
    // let screen = Screen {
    //     components: vec![Box::new(String::from("Hi"))],
    // };
    // screen.run();
    // The error is the following: the trait `Draw` is not implemented for `String`
    // The compiler generates nongeneric implementations of functions and methods for each concrete type used in place of a generic type parameter.
    // The code that results from monomorphisation does static dispatch: the compiler knows the method called at compile time
    // In this case, instead, dynamic dispatch is used: the compiler doesn't know at compile time which concrete method is called, the compiler emits code that it is figured out at runtime.
    // At compile time the compiler doesn't know all the types that might be used, so the method to call, instead at runtime Rust uses the pointer inside the trait object to know which method to call.
    // Dynamic dispatch prevents prvents the compiler from choosing to inline a method's code, which prevents some optimisation, and Rust has rules called syn compatibility, about where dynamic dispatch can be used.
    // However this introduces flexibility, but the trade-off is to consider.
}

fn state_pattern() {
    // The state pattern is a behavioural object oriented design pattern that expects a set of internal states represented as a set of state objects, and the value's behaviour changes based on this state
    // An example is a blog post struct with a field to hold its state, such as "draft", "reviewed", or "published".
    // Each state object is responsible of its own behaviour and for governing when the state should change, while the value that holds a state doesn't know anything about it
    // The advantage of the state pattern is that, when a business requirement changes, the code of the valuedoesn't need to be changed. Only the state objects changes by updating one, or add more states.
    // The implementationwill be incremental from a object oriented approach to a Rust approach and the final functionalities will be the following:
    // 1. A blog post starts as an empty draft.
    // 2. When the draft is done, a review of the post is requested.
    // 3. When the post is approved, it gets published.
    // 4. Only published blog posts return content to print, so unapproved posts can’t accidentally be published.
    // Any other changes have no effect. Here is an example using a not implemented module called `blog`
    // {
    //     let mut post = Post::new(); // User creates a new sraft blog post
    //     post.add_text("My post content"); // Text is added to the post
    //     assert_eq!("", post.content()); // This doesn't get any text because the post is not yet been approved
    //     post.request_review(); // Request a review
    //     assert_eq!("", post.content()); // NO text yet because it's not been approved
    //     post.approve(); // Approve the post
    //     assert_eq!("My post content", post.content()); // Now the text is available because the post was approved
    // }
    // The `Post` type is the only one used, and it uses the state pattern, the changing of the state is managed within the `Post` type.
    // The state chnages in response to calling the methods on the `Post` instance, bu the user doesn't need to manage it.
    // Now the `blog` module can be implemented in the library using a `Post` struct holding the content, with a `new` method to create a new instance.
    // Additionally a private `State` trait is used to define the behaviour of the state object for a `Post`
    // `Post` holds a trait object `Box<dyn State>` inside an `Option<T>` in a private field `state` to hold the state.
    // At this point there is a `State` trait that defines the behaviour shared by the different post states.
    // The state objects are `Draft`, `PendingReview`, and `Published`, and they all implement the `State` trait.
    // When a new `Post` is created its `state` field is `Some` value that holds a `Box` which points to a new instance of the `Draft` struct, ensuring that every new instance of `Post` starts as a draft.
    // Since the `state` field of `Post` is private there is no way to create a post in any other state, meanwhile the private `content` field is initialised to an empty string
    // On a post is necessary to set a new content using a method `add_text` and pass it a `&str`, without the need to exposing publibly `content`
    // The `add_text` method takes a mutable reference to `self` in order to change the `Post` instance on which `add_text` is called by pushing the passed string to `content`.
    // This behaviour is not part of the state pattern because `add_text` doesn't interact with the `state` field at all.
    // Even if the `add_text` is called, the `content` method of post must return an empty string because it's still in the draft state.
    // For now it simply returns `""` but it will change later, for now only the draft state exists.
    // The next feature is the request to reqview, which changes the state from `Draft`, to `PendingReview`
    // `Post` now has a public method `request_review` that takes a mutable reference to `self`, that calls a `request_review` method on `State` taht consumes the current state and returns a new one.
    // All the types implementing the `State` trait need a `request_review` method
    // The first parameter of the method is `self: Box<Self>` meanin gthat the method is only valid if called on a `Box` holding the type.
    // This syntax takes ownership of `Box<Self>`, invalidating the old state and transforming hte `Post` state to a new state
    // To consume the old state `request_review` needs to take ownership of the state value
    // This is where the `Option` in the `state` of `Post` comes in: the `take` method is called to take `Some` value out of the `state` leaving a `None` in its place because Rust doesn't let keeping a field unpopulated.
    // So the `state` value is moved out of `Post` rather then borrowing it, then the new `state` value is setted
    // It's needed to temporarily set the `state` field as `None` rather then setting it directly to get ownership of the `state` field, ensuring `Post` can't use the old `state` value after transformed.
    // The `request_review` method of `Draft` returns a new boxed instance of the `PendingReview` struct, which represent the state when a post waits for a review.
    // The `PendingReview` struct also needs to implement a `request_review` method, but it doesn't do any transformation, rather it returns itself.
    // The advantage of the state pattern is that the `request_review` method is the same independently of the `state` value, and each state is responsible for its own rules.
    // By now the `content` method doesn't change because it needs to be an empty string slice until approved, the `approve` method is implemented in this phase.
    // The `approve` method is added to the `State` trait, as well as the new struct `Published` state implementing `State`.
    // Similarly to `request_review` on `PendingReview` the `approve` method on `Draft` returns `self` because it has no effect.
    // `approve` can be called on `PendingReview` to return a new boxed instance of the `Published` struct,
    // The `Published` struct implements `State` and returns itself both for `request_preview` and `approve` because the post stays in the `Published` state
    // now the `content` method on `Post` needs to be updated to return the actual value after being published
    // Since the goal is to keep the the rules inside the structs that implement `State`, `Post` needs to delegate to a `content` method defined on its `state`
    // The `content` method on the value in `state` expects to pass the post instance `self` as an argument, then the `content` method in each struct decides what to do.
    // TO do this `as_ref` on `Option` is used to get the reference of the value inside `Option` rather than taking ownership. This returns a `Option<&Box<dyn State>>`.
    // Without `as_ref` teh compiler would give error because `state` can't be moved out of the borrowed `&self`
    // Then the `unwrap` method is called, and it would never panic because it is ensured that `state` always contains `Some` value, even if the compiler doesn't know it.
    // At this point, when `content` is called on the `&Box<dyn State>` deref coercion takes effect on `&` and the `Box` so the `content` method will be called on the type implementing the `State` trait
    // This means that `content` is added to the `State` definition, and by default it returns `""`, only `Published` overrides it returning the value in `post.content`
    // It needs a lifetime annotation because a reference to a `post` is taken as an argument and, returning part of it, the reference is related to the lifetime of the `post` argument
    // An enum could have been used with the disadvantage that every place that checks the value needs a `match` expression or similar to handle all the variants.
    // This is an implementation of the state pattern that doesn't require `match` expressions and, implementing new states, would mean add a new struct and implement the trait methods on it.
    // One trade-off is that , since states implement the transition between states, some are coupled to each other, meaning that adding a state between two would require to change both.
    // It would be easier if they didn't, but it would be another pattern.
    // Another downside is the duplicated logic: to eliminate them it sould be possible to make default implementation of `request_review` and `approved` on `State`
    // This wouldn't work because `State` doesn't know what the concrete `self` is, so the return type isn't known at compile time.
    // Another duplication ise the similar `request_review` and `approve` methods on `Post`, which use `Option::take` and delegate to the wrapped value method. This could be fixed with a `macro`.
    {
        use c18_object_oriented_programming::blog::Post;

        let mut post = Post::new(); // User creates a new sraft blog post
        post.add_text("My post content"); // Text is added to the post
        println!("Post content: {}", post.content()); // This doesn't get any text because the post is not yet been approved
        post.request_review(); // Request a review
        println!("Post content: {}", post.content()); // This doesn't get any text because the post is not yet been approved
        post.approve(); // Approve the post
        println!("Post content: {}", post.content()); // This doesn't get any text because the post is not yet been approved
    }
    // The state pattern can be rethinked encoding the states into different types, so Rust's type checking system issue a compiler error if draft posts are used where only published posts are allowed.
    // This means that the creation is still enabled using `Post::new`, and it is possible to add text on the content
    // The difference is that, instead of a `content` method on a draft post that returns an empty string, the draft post doesn't ahve the `content` method at all.
    // In this way, when trying to get the content of a draft, the compiler error tell that the method doesn't exist, making it impossible to accidently displaying a draft post content.
    // Both the `Post` and `DraftPost` structs have a private `content` field, but the strcts no longer have the `state` field because the encoding of the state is moved to the types of the struct.
    // The `Post` struct represents the published struct, and its `content` method returns the `content`
    // The `Post::new` function returns an instance of 'DraftPost` because `content` is private and there aren't any functions that return `Post`, so it is not possible to create and instance of `Post` now
    // So `DraftPost` have an `add_text` method, but not a `content` method, so the content is not available at all, and any attempt results in a compiler error.
    // The constraints are implemented by adding a struct `PendingReviewPost` defining the `request_review` on `DraftPost` to return a `PendingReviewPost`, and defining an `approve` method on it to return a `Post`
    // The `request_review` and `approve` methods take owenrship of `self`, transforming `DraftPost` in `PendingReviewPost`, and the latter in a published `Post`
    // The only struct with a `content` method is `Post`, and to get it is through the `approve` method on `PendingReviewPost`, obtained only by calling `request_review` on `DraftPost`.
    // This time the methods return new instances rather than modifying the structs, so more `let post =` are needed,
    // Additionally is no longer possible to print the empty string of the contents of the structs other than `Post`
    {
        use c18_object_oriented_programming::blog_no_state::{DraftPost, PendingReviewPost, Post};

        let mut post: DraftPost = Post::new();

        post.add_text("My post content");

        let post: PendingReviewPost = post.request_review();

        let post: Post = post.approve();

        println!("Post content: {}", post.content());
    }
    // These changes don't follow the object-orineted state pattern because of the reassignment and the transformations are no longer encapsulated, but this prevents invalid states at compile time.
}
