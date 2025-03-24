/// When there are a lot of integration tests it may be useful to organise them
/// For example tests may be grouped by functions and functionalities
/// Files in the tests directory don't share the same behaviour as files in src
/// Each file in the tests directory is managed as a single crate
/// Files in subdirectories of tests don't get compiled as separate crates, and don't have sections in the test output
/// However function in the common file can be used in the integration test files as modules
pub fn setup() {
    // setup code specific to your library's tests would go here
}
