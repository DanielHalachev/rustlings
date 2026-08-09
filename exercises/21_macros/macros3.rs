// TODO: Fix the compiler error without taking the macro definition out of this
// module.
// Added the attribute `macro_use` attribute.
// #[macro_use]
mod macros {
    // Added the attribute `macro_export`.
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
