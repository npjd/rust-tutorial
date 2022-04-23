pub fn run() {
    // fixed string
    let hello = "hello";

    println!("length: {}", hello.len());

    // growable string
    let mut new_hello = String::from("Hello");

    new_hello.push(' ');

    new_hello.push_str("World!");

    println!("{}", new_hello.contains("Hello"));

    for word in new_hello.split_whitespace() {
        println!("{}", word);
    }

    assert_eq!(true, new_hello.contains("Hello"))
}
