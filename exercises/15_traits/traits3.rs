trait Licensed {
    // TODO: Add a default implementation for `licensing_info` so that
    // implementors like the two structs below can share that default behavior
    // without repeating the function.
    // The default license information should be the string "Default license".
    // NOTE: the default implementation sits here, not in a separate impl block
    fn licensing_info(&self) -> String {
        String::from("Default license")
    }
}

// NOTE: this is wrong!
// This is not a default implementation, but an "inherent implementation"
// `dyn Licensed`` is a dynamically sized, type-erased view of some value implementing Licensed
// so it must normally be accessed through &dyn Licensed, Box<dyn Licensed>, and similar pointers.
impl dyn Licensed {
    // this method:
    // is available only through a trait object
    // is not part of the trait's virtual functions table
    // thus:
    // is not inherited by SomeSoftware
    // cannot be overridden by individual Licensed implementations
    // can call actual trait methods which then dynamically dispatch through the vtable
    // would throw an error if named exactly as the trait method and called on dyn Licensed
    fn licensing_info2(&self) -> String {
        String::from("Default license")
    }
    // an operation available for all objects that implement Licensed should be a default
    // this definition above is only useful to define behavior on trait objects (usually type-erased)
    // rather than on the concrete underlying types
    // effectively static vs dynamic dispatch behavior
    // a rare use case is a factory returning a trait object, e.g.
    // impl dyn Service {
    //     fn from_name(name: &str) -> Box<dyn Service> {
    //         Select a concrete implementation at runtime.
    //         ...
    //     }
    // }

    // let service = <dyn Service>::from_name("email");
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {} // Don't edit this line.
impl Licensed for OtherSoftware {} // Don't edit this line.

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = "Default license";
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
