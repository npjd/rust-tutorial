pub fn run(){
    // not mutable and fixed EXACT size of 5
    let numbers: [i32;5] = [1,2,3,4,5];

    let mut mutableNumbers = numbers;
    mutableNumbers[0] = 1;

    println!("{:?}",mutableNumbers);

}