fn main() {
    let s1 = String::from("hello");

    let len = caluculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len)
}

fn caluculate_length(s: &String) -> usize {
    s.len()
}
