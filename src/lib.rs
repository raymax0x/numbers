pub fn print(limit: u8) {
    let numbers = generate_sequence1(limit);
    output_sequence(&numbers);
}

fn generate_sequence(limit: u8) -> Vec<u8> {
    let mut numbers = Vec::new();

    for n in 1..=limit {
        numbers.push(n);
    }
    numbers
}

// you can replace the whole generate sequence function from this
fn generate_sequence1(limit: u8) -> Vec<u8> {
    (1..=limit).collect()
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
