fn return_reference(some_string: &String) -> &String {
    some_string
} // some_string goes out of scope, but reference_to_s still points to the memory location

fn main() {
    let s = String::from("Hello");
    let reference_to_s = return_reference(&s);
    println!("{}", reference_to_s);
    println!("HEY {}", reference_to_s)
}