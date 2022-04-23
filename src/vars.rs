pub fn run() {
    let name = "Nima";
    let mut age = 16;
    println!("My name is {}", name);
    println!("I am {}", age);

    age += 1;

    println!("I am now {}", age);

    const ID: i32 = 001;
    println!("{}", ID);

    let (new_name, new_age) = ("hello", 12);

    println!("{} is {}", new_name, new_age);
}
