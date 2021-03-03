fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let mut s3 = String::from("");
    s3 = s3 + &s1 + &s2;

    println!("{} {} {}", s1, s2, s3);

    for c in s3.bytes() {
        println!("{}", c);
    }
}
