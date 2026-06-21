// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// The exact form of this will be:
// - The input is going to be a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - The output element is going to be a vector of strings.

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // fn append_n_times(input: &str, times: &usize) -> String {
    //     const WORD: &str = "bar";

    //     let mut appended = String::new();
    //     for _ in 0..*times {
    //         appended.push_str(WORD);
    //     }

    //     input.to_string() + &appended
    // }

    // TODO: Complete the function as described above.
    // NOTE: I would personally ask for a reference, but the tests expect consuming the Vec
    // however, transformations usually consume the parameter and here it's beneficial

    // pub fn transformer(input: &[(String, Command)] -> Vec<String> {
    //     let output = input.iter().map(|(string, command)| -> String {
    //         match command {
    //             Command::Uppercase => return string.to_uppercase(),
    //             Command::Trim => return string.trim().to_string(),
    //             Command::Append(times) => return append_n_times(string, times),
    //         }
    //     });

    //     // NOTE: we don't even need to create a variable `output` to return it cleanly
    //     // but I think it's more readable
    //     output.collect()
    // }

    // NOTE: the difference between iter() and into_iter() is that iter() creates a reference to the element
    // while into_iter() directly consumes the element
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let output = input.into_iter().map(|(string, command)| -> String {
            match command {
                Command::Uppercase => return string.to_uppercase(),
                Command::Trim => return string.trim().to_string(),
                Command::Append(times) => string + &"bar".repeat(times), // this directly consumes the string and avoids copies
            }
        });

        // NOTE: we don't even need to create a variable `output` to return it cleanly
        // but I think it's more readable
        output.collect()
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {

    // TODO: What do we need to import to have `transformer` in scope?
    use super::Command;
    use crate::my_module::transformer;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
