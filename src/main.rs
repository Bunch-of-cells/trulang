fn main() {
    trulang::run("c.tru").unwrap_or_else(|err| println!("{}", err));
}
