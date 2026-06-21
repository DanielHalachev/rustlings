// Calls of this function should be replaced with calls of `string_slice` or `string`.
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Here are a bunch of values - some are `String`, some are `&str`.
// Your task is to replace `placeholder(…)` with either `string_slice(…)`
// or `string(…)` depending on what you think each value is.
fn main() {
    string_slice("blue");

    string("red".to_string());

    string(String::from("hi"));

    // if you own something, it's not a reference
    // thus not &str
    // thus only a String can be owned
    string("rust is fun!".to_owned());

    string_slice("nice weather".into());

    // format appends/composes strings, so String makes sense to be the return value
    // after all, the new byte array for the string needs to be allocated and &str cannot do that
    string(format!("Interpolation {}", "Station"));

    // WARNING: This is byte indexing, not character indexing.
    // Character indexing can be done using `s.chars().nth(INDEX)`.
    // we are taking a slice of String here, which is &str
    string_slice(&String::from("abc")[0..1]);

    // trimming returns a substring of the String/&str, thus we can return &str here
    string_slice("  hello there ".trim());

    // replacement may increase the length of the &str/String
    // so a new allocation may be necessary
    // &str cannot do that => String
    string("Happy Monday!".replace("Mon", "Tues"));

    // to convert to lowercase, we need a &mut str
    // &str cannot do that
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());

    // TL;DR: slices are immutable, thus they can only be used as return types
    // for operations that return an unmodified segment of a string
}
