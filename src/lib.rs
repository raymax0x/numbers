pub fn say_hello() {
    println!("Hello, world this is yash");
}

pub fn print() {
    let numbers = [1,2,3,4,5];
    let () = numbers;
    for n in numbers.iter() {
        println!("{:?}", n);
        println!("{}", n);
    }
}

