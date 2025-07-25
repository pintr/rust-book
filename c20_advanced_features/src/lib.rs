use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree that we can manipulate.
    // Use unwrap to get a `TokenStream` instead of `Result`, which is not compliant with the macro API.
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation.
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    // Get an `Ident` struct instance containing the name (identifier) of the annotated type using `ast.ident`
    // In this example, when the `impl_hello_macro` function is called, the `ident` will have the field with value `"Pancakes"`.
    // The `name` variable will contain an `Ident` struct such that, when printed, will be the string `"Pancakes"`: the name of the struct.
    let name = &ast.ident;
    // The `quote!` macro let's defining the Rust code that will return.
    // The compiler expects something different to the direct result of the `quote!` macro's execution, so it needs to be converted to `TokenStream`
    // This is done by calling the `into` method, that consumes the intermediate representation and returns the value of the required `TokenStream` type.
    // The `qupte!` macro also provides some templating mechanics: such as entering `#name`, and `quote!` will replace it with the calue in the variable `name`.
    let generated = quote! {
        // The procedural macro needs to generate an implementation of the `HelloMacro` trait for the type the user annotated, and can be get by using `#name`.
        // The trait implementation has the one funciton `hello_macro`, whose body contains the functionality to provide: printing `Hello, Macro! My name is`, and the name of the annotated type.
        impl HelloMacro for #name {
            fn hello_macro() {
                // The `stringify!` macro used here is built into Rust, and it takes a Rust expression, and converts it into a string literal. (`1 + 2` becomes `"1 + 2"`)
                // This is different from `format!` and `println!` macros, which evaluate the expression and turn the result into `String`
                // there is the possibility that `#name` input might be an expression to print literally.
                // using `stringify!` also saves an allocation by converting `#name` to  astring literal at compile time.
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    generated.into()
}
