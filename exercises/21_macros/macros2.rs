// TODO: Fix the compiler error by moving the whole definition of this macro.
// Solution: macros need to be defined before their first usage
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}
fn main() {
    my_macro!();
}

// // TODO: Fix the compiler error by moving the whole definition of this macro.
// macro_rules! my_macro {
//     () => {
//         println!("Check out my macro!");
//     };
// }
