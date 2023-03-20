use bindings::streams;

fn main() {
     assert_eq!(streams::blocking_write(1, b"hello\n").unwrap(), 6);
}
