macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // TODO: Fix the macro call.
    // my_macro();
    // all macros have ! at the end of their names
    my_macro!();
}
