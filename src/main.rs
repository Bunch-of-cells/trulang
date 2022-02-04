use std::fs;

fn main() {
    trulang::run(&fs::read_to_string("std.tru").unwrap(), "std.tru")
        .unwrap_or_else(|err| println!("{}", err));
}
