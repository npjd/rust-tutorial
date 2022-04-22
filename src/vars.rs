pub fn run() {
    let name = "Nima";
    let mut age = 16;
    println!("My name is {}", name);
    println!("I am {}", age);

    age += 1;

    println!("I am now {}", age);

    const ID: i32 = 001;
    println!("{}", ID);

    let (newName, newAge) = ("hello", 12);

    println!("{} is {}", newName, newAge);
}
