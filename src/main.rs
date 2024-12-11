fn modify_string(s1: &String, s2: &String) -> String {
    s1.to_owned() + s2
}

fn main() {
    let s1 = String::from("asd");
    let s2 = String::from("ads");
    println!("{}", modify_string(&s1, &s2));
}
