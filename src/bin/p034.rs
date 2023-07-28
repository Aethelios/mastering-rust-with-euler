use factorial::Factorial;

fn main() {
    let mut sum = 0;

    for i in 3..1000000 {
        let digit_sum: u32 = i
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap().factorial())
            .sum();

        if i as u32 == digit_sum {
            sum += i;
        }
    }

    println!("Sum: {}", sum); 
}