// TODO: Fix the compiler error by adding one or two characters.
#[rustfmt::skip]
// () wrap the pattern
// $x denotes a macro variable called x
// the $ sign helps differentiate a macro variable from a regular rust variable inside the macro code
// $x: expr matches any non-empty expression
// semicolons ";" are required to separate the macro arms, the last one is optional
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    }; // FIX: added ;
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }; // FIX: added ;
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
