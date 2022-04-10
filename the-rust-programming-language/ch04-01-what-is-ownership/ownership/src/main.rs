fn main() {
    let s1 = String::from("hello");

    let (s2, len) = caluculate_length(s1);

    println!("The length of '{}' is {}.", s2, len)
}

fn caluculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
