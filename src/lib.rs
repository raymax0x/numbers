pub fn print(limit: u8) {
    let numbers = generate_sequence(limit);
    output_sequence(&numbers);
}

fn generate_sequence(limit: u8) -> Vec<u8> {
    let mut numbers = Vec::new();

    for n in 1..=limit {
        numbers.push(n);
    }
    numbers
}

fn output_sequence(numbers: &[u8]) {
    for n in numbers {
        println!("{}", n);
    }
}

// how to slice an array
// fn get_subset() {
//     let numbers = [1, 2, 3, 4, 5];
//     let subset = &numbers[1..3];
//     output_sequence(&subset);
// }
