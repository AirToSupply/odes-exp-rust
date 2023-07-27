use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    // Hello, Macro! My name is Pancakes!
    Pancakes::hello_macro();
}